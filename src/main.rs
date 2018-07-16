use std::fs::File;
use std::io::BufReader;
use std::io::Read;

const TITLE_LENGTH: usize = 20;

fn main() {
    let file = File::open("addicti.mod").expect("Could not find file."); // TODO temporary use only this file
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
