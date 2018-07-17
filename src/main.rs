#![allow(dead_code)]
#![allow(unused_variables)]

extern crate rust_mod;
extern crate cpal;

use std::fs::File;
use rust_mod::song::*;

use cpal::EventLoop;


fn main() {
    let file = File::open("test.mod").expect("Could not find file"); // TODO temporary use only this file
    let song = Song::new(file);

    let device = cpal::default_output_device().expect("Failed to get default output device");
    let format = device.default_output_format().expect("Failed to get default output format");
    let event_loop = cpal::EventLoop::new();
    let stream_id = event_loop.build_output_stream(&device, &format).unwrap();
    event_loop.play_stream(stream_id.clone());

    let sample_rate = format.sample_rate.0 as f32;
    let mut sample_clock = 0f32;

    let mut freq = 200.0;
    let mut i = 0;
    // Produce a sinusoid of maximum amplitude.
    let mut next_value = || {
            freq = song.get_patterns()[0].get_note(i).get_frequency();
            i += 1;
            if i > 255 {
                i = 0;
            }
        sample_clock = (sample_clock + 1.0) % sample_rate;
        (sample_clock * freq as f32 * 2.0 * 3.141592 / sample_rate).sin()
    };

    event_loop.run(move |_, data| {
        match data {
            cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::U16(mut buffer) } => {
                for sample in buffer.chunks_mut(format.channels as usize) {
                    let value = ((next_value() * 0.5 + 0.5) * std::u16::MAX as f32) as u16;
                    for out in sample.iter_mut() {
                        *out = value;
                    }
                }
            },
            cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::I16(mut buffer) } => {
                for sample in buffer.chunks_mut(format.channels as usize) {
                    let value = (next_value() * std::i16::MAX as f32) as i16;
                    for out in sample.iter_mut() {
                        *out = value;
                    }
                }
            },
            cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer) } => {
                for sample in buffer.chunks_mut(format.channels as usize) {
                    let value = next_value();
                    for out in sample.iter_mut() {
                        *out = value;
                    }
                }
            },
            _ => (),
        }
    });
}