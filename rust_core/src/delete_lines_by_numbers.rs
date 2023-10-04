use std::fs::File;
use std::io::{self, prelude::*, BufReader, BufWriter};
use std::io::ErrorKind;
use std::io::Result;
use std::env;

pub fn delete_lines_by_numbers(filename: &str, line_numbers: &[usize]) -> io::Result<()> {
    let temp_filename = "temp2.txt";
    let temp_file = File::create(&temp_filename)?;

    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut temp_writer = BufWriter::new(temp_file);

    let mut total_lines = 0;

    for (current_line_number, line) in reader.lines().enumerate() {
        let line = line?;
        total_lines += 1;

        if !line_numbers.contains(&(current_line_number + 1)) {
            writeln!(temp_writer, "{}", line)?;

            if current_line_number + 1 == total_lines && line_numbers.contains(&total_lines) {
                println!("Deleted last line (Line {})", current_line_number + 1);
                continue;
            }
        }
    }

    temp_writer.flush()?;
    drop(temp_writer);

    let current_dir = env::current_dir()?;
    let temp_path = current_dir.join(temp_filename);

    println!("file path: {:?}", temp_path);
    Ok(())
}