use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn read_input_lines() -> impl Iterator<Item = String> {
    let input_file = std::env::args()
        .skip(1)
        .next()
        .expect("Expected input FILE");
    let lines = BufReader::new(File::open(input_file).expect("Could not open FILE"))
        .lines()
        .map(|line| line.expect("Could not read line"));
    lines
}

pub fn read_file_lines(file: &str) -> impl Iterator<Item = String> {
    let input_file = file;
    let lines = BufReader::new(File::open(input_file).expect("Could not open FILE"))
        .lines()
        .map(|line| line.expect("Could not read line"));
    lines
}
