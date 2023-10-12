use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufRead;
use std::fs;
use std::fs::OpenOptions;
use std::io::BufWriter;
use chrono::prelude::*;
use chrono::{Local, Datelike, Utc, offset::TimeZone};
use std::thread::sleep;
use std::thread;
use std::time::Duration;





#[no_mangle]
pub extern "C" fn check_if_event_expired() {
    
    loop {
        let file = File::open("data.txt").expect("Failed to open file");
        let reader = BufReader::new(file);

        let current_date = Local::today().naive_local();

        for line in reader.lines() {
            let line = line.expect("failed to get lines");

            if line.starts_with("Ending date:") {
                let getdate = line.trim_start_matches("Ending date: ").trim();
                let target_date = NaiveDate::parse_from_str(&getdate, "%Y-%m-%d").unwrap();
                println!("{}", getdate);

                if current_date > target_date {
                    println!("The target date has passed");
                } else if current_date == target_date {
                    println!("the target date is today");
                    delete_blocks((&target_date).to_string())
                } else {
                    println!("still not expired huh");
                }
            
            }
        }

        sleep(Duration::from_secs(5));

    }
    
}

pub fn delete_blocks(input: String) {
     
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