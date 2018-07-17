#![allow(dead_code)]
#![allow(unused_variables)]

extern crate rust_mod;
extern crate rodio;

use std::fs::File;
use rust_mod::song::*;
use rodio::Source;
use rodio::Sink;

use std::{thread, time};

fn main() {
    let file = File::open("test.mod").expect("Could not find file"); // TODO temporary use only this file
    let song = Song::new(file);
    let pattern_positions = song.get_pattern_positions();


    let device = rodio::default_output_device().unwrap();

    let mut i = 0;
    let mut pp = 0;
    loop {
        let mut freq1 = song.get_patterns()[pattern_positions[pp] as usize].get_note(i).get_frequency();
        let mut freq2 = song.get_patterns()[pattern_positions[pp] as usize].get_note(i+1).get_frequency();
        let mut freq3 = song.get_patterns()[pattern_positions[pp] as usize].get_note(i+2).get_frequency();
        let mut freq4 = song.get_patterns()[pattern_positions[pp] as usize].get_note(i+3).get_frequency();

        let c1 = Sink::new(&device);
        let s1 = rodio::source::SineWave::new(freq1 as u32 );

        let c2 = Sink::new(&device);
        let s2 = rodio::source::SineWave::new(freq2 as u32 );

        let c3 = Sink::new(&device);
        let s3 = rodio::source::SineWave::new(freq3 as u32 );

        let c4 = Sink::new(&device);
        let s4 = rodio::source::SineWave::new(freq4 as u32 );

        c1.append(s1);
        c2.append(s2);
        c3.append(s3);
        c4.append(s4);

        let time = time::Duration::from_millis(480); // TODO get this number from song itself
        thread::sleep(time);
        i += 4;
        if i > 255 {
            i = 0;
            pp += 1;
        }
    }
}