use std::fs::File;
use std::io::BufReader;
use std::io::Read;

const TITLE_LENGTH: usize = 20;

struct Sample {
    name: String,
    length: u16,
    finetune: i8,
    volume: u8,
    repeat_point: u16,
    repeat_length: u16
}

struct Pattern {
    data: [u8; 1024]
}

struct Song {
    title: String,
    length: u8,
    special_byte: u8,
    pattern_positions: [u8; 128],
    label: String,
    samples: [Sample; 31],
    patterns: [Pattern; 127]
}

fn main() {
    let file = File::open("test.mod").expect("Could not find file."); // TODO temporary use only this file
    let mut buf_reader = BufReader::new(file); // TODO is a buffered reader even useful in this situation?
    let mut file_vector: Vec<u8> = Vec::new();

    let mut title = String::with_capacity(TITLE_LENGTH);

    match buf_reader.read_to_end(&mut file_vector) {
        Ok(_) => {},
        Err(e) => {eprintln!("{}", e)},
    }

    let mut file_vector_iter = file_vector.iter();

    for _ in 0..TITLE_LENGTH {
        title.push(*file_vector_iter.next().unwrap() as char)
    }


    println!("{}", title);
}
/*
fn get_sample() -> Sample {

}

fn get_pattern() -> Pattern {

}

fn get_song() -> Song {

}
*/