use std::fs::File;
use std::io::BufReader;
use std::io::Read;

const SONG_TITLE_START: usize = 0;
const SONG_TITLE_AMOUNT: usize = 20;
const SONG_LENGTH_START: usize = 950;
const SONG_LENGTH_AMOUNT: usize = 1;
const SONG_SPECIAL_BYTE_START: usize = 951;
const SONG_SPECIAL_BYTE_AMOUNT: usize = 1;
const SONG_PATTERN_POSITIONS_START: usize = 952;
const SONG_PATTERN_POSITIONS_AMOUNT: usize = 128;
const SONG_LABEL_START: usize = 1080;
const SONG_LABEL_AMOUNT: usize = 4;

const SAMPLE_AMOUNT: usize = 31;
const PATTERN_AMOUNT: usize = 90; // TODO need to find out dynamically, can go up to 128, test file has 90

const SAMPLE_BYTE_AMOUNT: usize = 30;
const SAMPLE_NAME_START: usize = 20;
const SAMPLE_NAME_AMOUNT: usize = 22;
const SAMPLE_LENGTH_START: usize = 42;
const SAMPLE_LENGTH_AMOUNT: usize = 2;
const SAMPLE_FINETUNE_START: usize = 44;
const SAMPLE_FINETUNE_AMOUNT: usize = 1;
const SAMPLE_VOLUME_START: usize = 45;
const SAMPLE_VOLUME_AMOUNT: usize = 2;
const SAMPLE_REPEAT_POINT_START: usize = 46;
const SAMPLE_REPEAT_POINT_AMOUNT: usize = 2;
const SAMPLE_REPEAT_LENGTH_START: usize = 48;
const SAMPLE_REPEAT_LENGTH_AMOUNT: usize = 2;

const PATTERN_BYTE_AMOUNT: usize = 1024;
const PATTERN_START: usize = 1084;

struct Sample {
    name: String,
    length: u16,
    finetune: i8,
    volume: u8,
    repeat_point: u16,
    repeat_length: u16
}

struct Pattern {
    data: Vec<u8>
}

struct Song {
    title: String,
    length: u8,
    special_byte: u8,
    pattern_positions: Vec<u8>,
    label: String,
    samples: Vec<Sample>,
    patterns: Vec<Pattern>
}

struct Note {
    sample_number: u8,
    note_period: u16,
    effect_command: u16
}

impl Note {
    fn new(b1: u8, b2: u8, b3: u8, b4: u8) -> Note {
        let mut sample_number = b1 & 0xF0;
        sample_number |= (b3 & 0xF0) >> 4;

        let mut note_period = u16::from(b2);
        note_period |= u16::from(b1 & 0x0F) << 8;

        let mut effect_command = u16::from(b4);
        effect_command |= u16::from(b3 & 0x0F) << 8;

        Note {
            sample_number,
            note_period,
            effect_command
        }

    }
}

fn main() {
    let file = File::open("test.mod").expect("Could not find file."); // TODO temporary use only this file
    let mut buf_reader = BufReader::new(file); // TODO is a buffered reader even useful in this situation?
    let mut file_vector: Vec<u8> = Vec::new();

    match buf_reader.read_to_end(&mut file_vector) {
        Ok(_) => {},
        Err(e) => {eprintln!("{}", e)},
    }

    let song = Song {
        title: get_string_from_file(&file_vector, SONG_TITLE_START, SONG_TITLE_AMOUNT),
        length: get_bytes_from_file(&file_vector, SONG_LENGTH_START, SONG_LENGTH_AMOUNT)[0],
        special_byte: get_bytes_from_file(&file_vector, SONG_SPECIAL_BYTE_START, SONG_SPECIAL_BYTE_AMOUNT)[0],
        pattern_positions: get_bytes_from_file(&file_vector, SONG_PATTERN_POSITIONS_START, SONG_PATTERN_POSITIONS_AMOUNT),
        label: get_string_from_file(&file_vector, SONG_LABEL_START, SONG_LABEL_AMOUNT),
        samples: get_samples(&file_vector),
        patterns: get_patterns(&file_vector)
    };

    print_debug_info(&song, 0, 0, 0);
}

fn get_bytes_from_file(file_vector: &[u8], start: usize, amount: usize) -> Vec<u8> {
    let mut ret = Vec::new();
    for i in start..start+amount {
        ret.push(match file_vector.get(i) {
            Some(t) => *t,
            None => panic!("Incorrect file at byte: {}", i)
        })
    }
    ret
}

fn get_string_from_file(file_vector: &[u8], start: usize, amount: usize) -> String {
    let mut string = String::new();
    for byte in get_bytes_from_file(file_vector, start, amount) {
        string.push(byte as char);
    }
    string.trim().to_string()
}

fn get_samples(file_vector: &[u8]) -> Vec<Sample> {
    let mut vec = Vec::new();
    for i in 0..SAMPLE_AMOUNT {
        vec.push(Sample {
            name: get_string_from_file(&file_vector, SAMPLE_NAME_START+i*SAMPLE_BYTE_AMOUNT, SAMPLE_NAME_AMOUNT),
            length: calculate_u16_from_two_u8(&get_bytes_from_file(&file_vector, SAMPLE_LENGTH_START+i*SAMPLE_BYTE_AMOUNT, SAMPLE_LENGTH_AMOUNT)),
            finetune: (get_bytes_from_file(&file_vector, SAMPLE_FINETUNE_START+i*SAMPLE_BYTE_AMOUNT, SAMPLE_FINETUNE_AMOUNT)[0] & 0x0F << 4) as i8 >> 4, // only lower 4 bits are relevant, mask the higher ones away just in case TODO check if this is correct
            volume: get_bytes_from_file(&file_vector, SAMPLE_VOLUME_START+i*SAMPLE_BYTE_AMOUNT, SAMPLE_VOLUME_AMOUNT)[0],
            repeat_point: calculate_u16_from_two_u8(&get_bytes_from_file(&file_vector, SAMPLE_REPEAT_POINT_START+i*SAMPLE_BYTE_AMOUNT, SAMPLE_REPEAT_POINT_AMOUNT)),
            repeat_length: calculate_u16_from_two_u8(&get_bytes_from_file(&file_vector, SAMPLE_REPEAT_LENGTH_START+i*SAMPLE_BYTE_AMOUNT, SAMPLE_REPEAT_LENGTH_AMOUNT)),
        });
    }
    vec
}

// big endian
fn calculate_u16_from_two_u8(vec: &[u8]) -> u16 {
    let mut ret = u16::from(vec[1]);
    ret <<= 8;
    ret |= u16::from(vec[0]);
    ret
}

fn get_patterns(file_vector: &[u8]) -> Vec<Pattern> {
    let mut vec = Vec::new();
    for i in 0..PATTERN_AMOUNT {
        vec.push(Pattern {
            data: get_bytes_from_file(&file_vector, PATTERN_START+i*PATTERN_BYTE_AMOUNT, PATTERN_BYTE_AMOUNT)
        });
    }
    vec
}

fn print_debug_info(song: &Song, sample_index: usize, pattern_index: usize, pattern_byte: usize) {
    println!("Song");
    println!("Title: {}", song.title);
    println!("Length: {}", song.length);
    println!("Special Byte: {}", song.special_byte);
    println!("Patter Positions Length: {}", song.pattern_positions.len());
    println!("Label: {}", song.label);
    println!("Samples Length: {}", song.samples.len());
    println!("Patterns Length: {}", song.patterns.len());
    println!();
    println!("Sample");
    println!("Name: {}", &song.samples[sample_index].name);
    println!("Length: {}", &song.samples[sample_index].length);
    println!("Finetune: {}", &song.samples[sample_index].finetune);
    println!("Volume: {}", &song.samples[sample_index].volume);
    println!("Repeat Point: {}", &song.samples[sample_index].repeat_point);
    println!("Repeat Length: {}", &song.samples[sample_index].repeat_length);
    println!();
    println!("Pattern: {}", song.patterns[pattern_index].data[pattern_byte]);
}