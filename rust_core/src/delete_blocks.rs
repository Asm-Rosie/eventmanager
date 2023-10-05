use std::fs::File;
use std::io::{self, prelude::*, BufReader, BufWriter};
use std::io::ErrorKind;
use std::io::Result;
use std::fs::OpenOptions;

pub fn delete_Blocks(input: String) {
     
    let enddate = input;
    
    let input_path = "data.txt";
    let temp_path = "data.temp.txt";

    let input_file = File::open(input_path).expect("Failed to open input file");
    let reader = BufReader::new(input_file);

    let temp_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(temp_path)
        .expect("Failed to create temporary file");

    let mut temp_writer = BufWriter::new(temp_file);

    let mut lines_to_delete = Vec::new();
    let mut found_matching_line = false;
    let mut deleted_sets = 0;

    for line in reader.lines() {
        let line = line.expect("failed to read lines");

        if line.contains(&enddate) {
            found_matching_line = true;
            continue;
        }

        if found_matching_line && lines_to_delete.len() >= 5 {
            found_matching_line = false;
            deleted_sets += 1;

            if deleted_sets == 1 {
                lines_to_delete.clear();
            }
        }

        if found_matching_line {
            lines_to_delete.push(line.clone()); // Clone the line to preserve it
        } else if deleted_sets == 0 {
            writeln!(temp_writer, "{}", line).expect("Failed to write line");
        }
    }

    temp_writer.flush().expect("couldn't flush");
    drop(temp_writer);

    std::fs::rename(temp_path, input_path).expect("Failed to replace original file with temporary file");
}
