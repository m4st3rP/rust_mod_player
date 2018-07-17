extern crate rust_mod;
extern crate cpal;

use std::fs::File;

use rust_mod::song::*;
use cpal::EventLoop;

fn main() {
    let file = File::open("test.mod").expect("Could not find file"); // TODO temporary use only this file
    let song = Song::new(file);

    let event_loop = EventLoop::new();
    let device = cpal::default_output_device().expect("No output device available");

    /*
    let note_number = 0;
    let note = Note::new(song.patterns[note_number].data[0], song.patterns[note_number].data[1], song.patterns[note_number].data[2], song.patterns[note_number].data[3]);
    println!("{}", note.to_string());
    */
}