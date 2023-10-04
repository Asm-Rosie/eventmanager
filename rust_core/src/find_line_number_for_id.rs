use std::fs::File;
use std::io::{self, prelude::*, BufReader, BufWriter};
use std::io::ErrorKind;
use std::io::Result;


pub fn find_line_number_for_id(id: &str, filename: &str) -> io::Result<Option<usize>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut line_number = 0;

    for (current_line_number, line) in reader.lines().enumerate() {
        let line = line?;

        if line.contains(&format!("ID: {}", id)) {
            line_number = current_line_number + 1;
            break;
        }
    }

    Ok(if line_number > 0 {
        Some(line_number)
    } else {
        None
    })
}