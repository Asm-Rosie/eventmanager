extern crate rodio;

use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::thread;

pub fn play_audio_mp3(input_file: &str) {
    let (stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let file = BufReader::new(File::open(&input_file).unwrap());
    let source = Decoder::new(file).unwrap();

    sink.append(source);

    sink.play();

    thread::park();
}