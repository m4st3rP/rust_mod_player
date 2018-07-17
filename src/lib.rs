// TODO remove this later
#![allow(dead_code)]
#![allow(unused_variables)]

mod general {
    pub fn get_bytes_from_file(file_vector: &[u8], start: usize, amount: usize) -> Vec<u8> {
        let mut ret = Vec::new();
        for i in start..start+amount {
            ret.push(match file_vector.get(i) {
                Some(t) => *t,
                None => panic!("Incorrect file at byte: {}", i)
            })
        }
        ret
    }

    pub fn get_string_from_file(file_vector: &[u8], start: usize, amount: usize) -> String {
        let mut string = String::new();
        for byte in get_bytes_from_file(file_vector, start, amount) {
            string.push(byte as char);
        }
        string.trim().to_string()
    }

    // big endian
    pub fn calculate_u16_from_two_u8(vec: &[u8]) -> u16 {
        let mut ret = u16::from(vec[1]);
        ret <<= 8;
        ret |= u16::from(vec[0]);
        ret
    }
}

pub mod song {
    use std::fs::File;
    use std::io::BufReader;
    use std::io::Read;

    use general::*;
    use song::sample::Sample;
    use song::pattern::Pattern;

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

    const PATTERN_AMOUNT: usize = 90; // TODO need to find out dynamically, can go up to 128, test file has 90
    const SAMPLE_AMOUNT: usize = 31;

    pub struct Song {
        title: String,
        length: u8,
        special_byte: u8,
        pattern_positions: Vec<u8>,
        label: String,
        samples: Vec<Sample>,
        patterns: Vec<Pattern>,
        highest_pattern: u8
    }

    impl Song {
        pub fn new(file: File) -> Song {
            let mut buf_reader = BufReader::new(file); // TODO is a buffered reader even useful in this situation?
            let mut file_vector: Vec<u8> = Vec::new();

            match buf_reader.read_to_end(&mut file_vector) {
                Ok(_) => {},
                Err(e) => {eprintln!("{}", e)},
            }

            let pattern_positions = get_bytes_from_file(&file_vector, SONG_PATTERN_POSITIONS_START, SONG_PATTERN_POSITIONS_AMOUNT);
            let highest_pattern = *pattern_positions.iter().max().unwrap();

            Song {
                title: get_string_from_file(&file_vector, SONG_TITLE_START, SONG_TITLE_AMOUNT),
                length: get_bytes_from_file(&file_vector, SONG_LENGTH_START, SONG_LENGTH_AMOUNT)[0],
                special_byte: get_bytes_from_file(&file_vector, SONG_SPECIAL_BYTE_START, SONG_SPECIAL_BYTE_AMOUNT)[0],
                pattern_positions,
                label: get_string_from_file(&file_vector, SONG_LABEL_START, SONG_LABEL_AMOUNT),
                samples: get_samples(&file_vector),
                patterns: get_patterns(&file_vector, highest_pattern),
                highest_pattern
            }
        }

        pub fn print_debug_info(&self) {
            println!("Song");
            println!("Title: {}", self.title);
            println!("Length: {}", self.length);
            println!("Special Byte: {}", self.special_byte);
            println!("Patter Positions Length: {}", self.pattern_positions.len());
            println!("Label: {}", self.label);
            println!("Samples Length: {}", self.samples.len());
            println!("Patterns Length: {}", self.patterns.len());
        }
    }

    fn get_samples(file_vector: &[u8]) -> Vec<Sample> {
        let mut vec = Vec::new();
        for i in 0..SAMPLE_AMOUNT {
            vec.push(Sample::new(&file_vector, i));
        }
        vec
    }

    fn get_patterns(file_vector: &[u8], max_pat: u8) -> Vec<Pattern> {
        let mut vec = Vec::new();
        for i in 0..PATTERN_AMOUNT {
            vec.push(Pattern::new(&file_vector, i, max_pat));
        }
        vec
    }

    mod sample {
        use general::*;

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

        pub struct Sample {
            name: String,
            length: u16,
            finetune: i8,
            volume: u8,
            repeat_point: u16,
            repeat_length: u16
        }

        impl Sample {
            pub fn new(file_vector: &[u8], i: usize) -> Sample {
                Sample {
                    name: get_string_from_file(&file_vector, SAMPLE_NAME_START+i*SAMPLE_BYTE_AMOUNT, SAMPLE_NAME_AMOUNT),
                    length: calculate_u16_from_two_u8(&get_bytes_from_file(&file_vector, SAMPLE_LENGTH_START+i*SAMPLE_BYTE_AMOUNT, SAMPLE_LENGTH_AMOUNT)),

                    // only lower 4 bits are relevant, mask the higher ones away just in case TODO check if this is correct
                    finetune: (get_bytes_from_file(&file_vector, SAMPLE_FINETUNE_START+i*SAMPLE_BYTE_AMOUNT, SAMPLE_FINETUNE_AMOUNT)[0] & 0x0F << 4) as i8 >> 4,
                    volume: get_bytes_from_file(&file_vector, SAMPLE_VOLUME_START+i*SAMPLE_BYTE_AMOUNT, SAMPLE_VOLUME_AMOUNT)[0],
                    repeat_point: calculate_u16_from_two_u8(&get_bytes_from_file(&file_vector, SAMPLE_REPEAT_POINT_START+i*SAMPLE_BYTE_AMOUNT, SAMPLE_REPEAT_POINT_AMOUNT)),
                    repeat_length: calculate_u16_from_two_u8(&get_bytes_from_file(&file_vector, SAMPLE_REPEAT_LENGTH_START+i*SAMPLE_BYTE_AMOUNT, SAMPLE_REPEAT_LENGTH_AMOUNT)),
                }
            }

            pub fn print_debug_info(&self) {
                println!("Sample");
                println!("Name: {}", self.name);
                println!("Length: {}", self.length);
                println!("Finetune: {}", self.finetune);
                println!("Volume: {}", self.volume);
                println!("Repeat Point: {}", self.repeat_point);
                println!("Repeat Length: {}", self.repeat_length);
            }
        }
    }

    mod pattern {
        use general::*;
        use song::note::*;

        const PATTERN_BYTE_AMOUNT: usize = 1024;
        const PATTERN_START: usize = 1084;

        pub struct Pattern {
            data: Vec<u8>,
            notes: Vec<Note>
        }

        impl Pattern {
            pub fn new(file_vector: &[u8], i: usize, max_pat: u8) -> Pattern {
                // a pattern consists of 1024 bytes
                let data = get_bytes_from_file(&file_vector, PATTERN_START+i*PATTERN_BYTE_AMOUNT, PATTERN_BYTE_AMOUNT);
                let mut notes = Vec::new();

                let mut i = 0;
                loop {
                    notes.push(Note::new(data[i], data[i+1], data[i+2], data[i+3])); // TODO find out why we sometimes get 0 as note_period, maybe this is supposed to be like this?
                    i += 4;
                    if i >= PATTERN_BYTE_AMOUNT { // break out of the loop when we reached the end of data
                        break;
                    }
                }

                Pattern {
                    data,
                    notes
                }
            }

            pub fn print_debug_info(&self, pattern_byte: usize) {
                println!("Pattern");
                println!("Pattern: {}", self.data[pattern_byte]);
            }
        }
    }

    pub mod note {
        use std::fmt;

        pub struct Note {
            sample_number: u8,
            note_period: u16,
            effect_data: u8,
            effect_command: u16,
            extended_command: Option<u8>,
            musical_note: MusicalNotes,
            frequency: f64
        }

        #[derive(Debug)] // so we can print the enum
        enum MusicalNotes {
            C1,
            Csharp1,
            D1,
            Dsharp1,
            E1,
            F1,
            Fsharp1,
            G1,
            Gsharp1,
            A1,
            Asharp1,
            B1,
            C2,
            Csharp2,
            D2,
            Dsharp2,
            E2,
            F2,
            Fsharp2,
            G2,
            Gsharp2,
            A2,
            Asharp2,
            B2,
            C3,
            Csharp3,
            D3,
            Dsharp3,
            E3,
            F3,
            Fsharp3,
            G3,
            Gsharp3,
            A3,
            Asharp3,
            B3,
            Quiet
        }

        impl fmt::Display for MusicalNotes {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                fmt::Debug::fmt(self, f)
            }
        }

        impl Note {
            pub fn new(b1: u8, b2: u8, b3: u8, b4: u8) -> Note {
                let mut sample_number = b1 & 0xF0;
                sample_number |= (b3 & 0xF0) >> 4;

                let mut note_period = u16::from(b2);
                note_period |= u16::from(b1 & 0x0F) << 8;

                let effect_command = u16::from(b3 & 0xF);
                let mut effect_data = b4;

                let mut extended_command = None;
                if effect_command == 0xE {
                    extended_command = Some(effect_data >> 4);
                    effect_data &= 0xF;
                }

                let musical_note = match note_period {
                    856 => MusicalNotes::C1,
                    808 => MusicalNotes::Csharp1,
                    762 => MusicalNotes::D1,
                    720 => MusicalNotes::Dsharp1,
                    678 => MusicalNotes::E1,
                    640 => MusicalNotes::F1,
                    604 => MusicalNotes::Fsharp1,
                    570 => MusicalNotes::G1,
                    538 => MusicalNotes::Gsharp1,
                    508 => MusicalNotes::A1,
                    480 => MusicalNotes::Asharp1,
                    453 => MusicalNotes::B1,
                    428 => MusicalNotes::C2,
                    404 => MusicalNotes::Csharp2,
                    381 => MusicalNotes::D2,
                    360 => MusicalNotes::Dsharp2,
                    339 => MusicalNotes::E2,
                    320 => MusicalNotes::F2,
                    302 => MusicalNotes::Fsharp2,
                    285 => MusicalNotes::G2,
                    269 => MusicalNotes::Gsharp2,
                    254 => MusicalNotes::A2,
                    240 => MusicalNotes::Asharp2,
                    226 => MusicalNotes::B2,
                    214 => MusicalNotes::C3,
                    202 => MusicalNotes::Csharp3,
                    190 => MusicalNotes::D3,
                    180 => MusicalNotes::Dsharp3,
                    170 => MusicalNotes::E3,
                    160 => MusicalNotes::F3,
                    151 => MusicalNotes::Fsharp3,
                    143 => MusicalNotes::G3,
                    135 => MusicalNotes::Gsharp3,
                    127 => MusicalNotes::A3,
                    120 => MusicalNotes::Asharp3,
                    113 => MusicalNotes::B3,
                    _   => MusicalNotes::Quiet
                    //_ => panic!("Musical note does not exist: NP: {}, b1: {}, b2: {}, b3: {}, b4: {}", note_period, b1, b2, b3, b4)
                };

                let frequency = match musical_note {
                    MusicalNotes::C1 => 32.7032,
                    MusicalNotes::Csharp1 => 34.6478,
                    MusicalNotes::D1 => 36.7081,
                    MusicalNotes::Dsharp1 => 38.8909,
                    MusicalNotes::E1 => 41.2034,
                    MusicalNotes::F1 => 43.6535,
                    MusicalNotes::Fsharp1 => 46.2493,
                    MusicalNotes::G1 => 48.9994,
                    MusicalNotes::Gsharp1 => 51.9131,
                    MusicalNotes::A1 => 55.0,
                    MusicalNotes::Asharp1 => 58.2705,
                    MusicalNotes::B1 => 61.7354,
                    MusicalNotes::C2 => 65.4064,
                    MusicalNotes::Csharp2 => 69.2957,
                    MusicalNotes::D2 => 73.4162,
                    MusicalNotes::Dsharp2 => 77.7817,
                    MusicalNotes::E2 => 82.4069,
                    MusicalNotes::F2 => 87.3071,
                    MusicalNotes::Fsharp2 => 92.4986,
                    MusicalNotes::G2 => 97.9989,
                    MusicalNotes::Gsharp2 => 103.826,
                    MusicalNotes::A2 => 110.0,
                    MusicalNotes::Asharp2 => 116.541,
                    MusicalNotes::B2 => 123.471,
                    MusicalNotes::C3 => 130.813,
                    MusicalNotes::Csharp3 => 138.591,
                    MusicalNotes::D3 => 146.832,
                    MusicalNotes::Dsharp3 => 155.563,
                    MusicalNotes::E3 => 164.814,
                    MusicalNotes::F3 => 174.614,
                    MusicalNotes::Fsharp3 => 184.997,
                    MusicalNotes::G3 => 195.998,
                    MusicalNotes::Gsharp3 => 207.652,
                    MusicalNotes::A3 => 220.0,
                    MusicalNotes::Asharp3 => 233.082,
                    MusicalNotes::B3 => 246.942,
                    MusicalNotes::Quiet => 0.0
                };

                Note {
                    sample_number,
                    note_period,
                    effect_data,
                    effect_command,
                    extended_command,
                    musical_note,
                    frequency
                }
            }

            fn to_string(&self) -> String {
                format!("Sample Number: {}, Note Period: {}, Effect Command: {}, Musical Note: {}", self.sample_number, self.note_period, self.effect_command, self.musical_note)
            }

            fn get_frequency(&self) -> f64 {
                self.frequency
            }
        }
    }
}