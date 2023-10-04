use std::fs::File;
use std::io::{self, prelude::*, BufReader, BufWriter};
use std::io::ErrorKind;
use std::io::Result;


pub fn print_lines_around(line_number: usize, filename: &str) -> io::Result<Vec<usize>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let start_line = if line_number > 5 { line_number - 5 } else { 0 };
    let end_line = line_number;
    
    let mut lines_to_collect = Vec::new();

    for (current_line_number, line) in reader.lines().enumerate() {
        let line = line?;

        if current_line_number >= start_line && current_line_number <= end_line {
            println!("Line {}: {}", current_line_number + 1, line);
            lines_to_collect.push(current_line_number);
        }

        if current_line_number > end_line {
            break;
        }

        if current_line_number > end_line {
            break;
        }

        
    }

    Ok(lines_to_collect)
}