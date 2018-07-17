#![allow(dead_code)]
#![allow(unused_variables)]

extern crate rust_mod;

use std::fs::File;

use rust_mod::song::*;

fn main() {
    let file = File::open("test.mod").expect("Could not find file"); // TODO temporary use only this file
    let song = Song::new(file);
}