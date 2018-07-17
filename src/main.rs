#![allow(dead_code)]
#![allow(unused_variables)]

extern crate rust_mod;
extern crate synth;

use std::fs::File;
use rust_mod::song::*;
use synth::*;

fn main() {
    let file = File::open("test.mod").expect("Could not find file"); // TODO temporary use only this file
    let song = Song::new(file);
    //let synth = Synth::new(1,2);
}