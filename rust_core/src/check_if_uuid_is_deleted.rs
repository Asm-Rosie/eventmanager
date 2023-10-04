use std::fs::File;
use std::io::{self, prelude::*, BufReader, BufWriter};
use std::io::ErrorKind;
use std::io::Result;

pub fn check_if_uuid_is_deleted(filename: &str, uuid_to_skip: &str) -> io::Result<()> {
    let temp_filename = "temp3.txt";
    let temp_file = File::create(&temp_filename)?;

    let file = File::open(filename)?; 
    let reader = BufReader::new(file);
    let mut temp_writer = BufWriter::new(temp_file);

    for line in reader.lines() {
        let line = line?;

        if line.contains(&uuid_to_skip) {
            println!("Skipping line: {}", uuid_to_skip);
            continue;
        }

        writeln!(temp_writer, "{}", line)?;
    }

    temp_writer.flush()?;
    drop(temp_writer);

    std::fs::rename(temp_filename, filename)?;


    Ok(())
}