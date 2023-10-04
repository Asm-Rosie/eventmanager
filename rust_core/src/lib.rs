use std::env;
use std::fs::File;
use std::io::Write;
use std::io::{self, prelude::*, BufRead, BufReader, BufWriter, Read};
use std::path::PathBuf;
use std::path::Path;
use chrono::prelude::*;
use std::fs::OpenOptions;
use std::io::ErrorKind;
use std::io::Result;
use std::fs;
use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use std::fs::read_to_string;
use chrono::{NaiveDate, Utc};
use std::thread::sleep;
use std::time::Duration;
use uuid::Uuid;


use crate::check_if_uuid_is_deleted::check_if_uuid_is_deleted;
mod check_if_uuid_is_deleted;

use crate::deleteBlocks::deleteBlocks;
mod deleteBlocks;

use crate::find_line_number_for_id::find_line_number_for_id;
mod find_line_number_for_id;

use crate::delete_lines_by_numbers::delete_lines_by_numbers;
mod delete_lines_by_numbers;

use crate::print_lines_around::print_lines_around;
mod print_lines_around;


#[no_mangle]
pub extern "C" fn create_file() {
    let file_path = "data.txt";

    if !std::path::Path::new(&file_path).exists() {
        File::create(&file_path).expect("failed to create file");
    } else {
        println!("file already exists");
    }
}



pub struct ExposedStrings {
    pub summaries: *mut c_char,
    pub descriptions: *mut c_char,
    pub start_dates: *mut c_char,
    pub end_dates: *mut c_char,
    pub uuid: *mut c_char,
}

#[no_mangle]
pub extern "C" fn expose_strings() -> ExposedStrings {

    let file = File::open("data.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut summaries: Vec<String> = Vec::new();
    let mut descriptions: Vec<String> = Vec::new();
    let mut start_dates: Vec<String> = Vec::new();
    let mut end_dates: Vec<String> = Vec::new();
    let mut uuid: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("failed to get lines");
        
        if line.starts_with("Summary:") {
            let summary = line.trim_start_matches("Summary: ").trim();
            summaries.push(summary.to_string());
        } else if line.starts_with("Description:") {
            let description = line.trim_start_matches("Description: ").trim();
            descriptions.push(description.to_string());
        } else if line.starts_with("Starting point:") {
            let start_date = line.trim_start_matches("Starting point: ").trim();
            start_dates.push(start_date.to_string());
        } else if line.starts_with("Ending date:") {
            let end_date = line.trim_start_matches("Ending date: ").trim();
            end_dates.push(end_date.to_string());
        } else if line.starts_with("ID: ") {
            let id = line.trim_start_matches("ID: ").trim();
            uuid.push(id.to_string());
        }
    }

    let combined_summaries = summaries.join("\n");
    let combined_descriptions = descriptions.join("\n");
    let combined_start_dates = start_dates.join("\n");
    let combined_end_dates = end_dates.join("\n");
    let combined_id = uuid.join("\n");

    let c_summaries = CString::new(combined_summaries).expect("CString failed");
    let c_descriptions = CString::new(combined_descriptions).expect("CString failed");
    let c_start_dates = CString::new(combined_start_dates).expect("CString failed");
    let c_end_dates = CString::new(combined_end_dates).expect("CString failed");
    let c_id = CString::new(combined_id).expect("CString failed");
    

    ExposedStrings {
        summaries: c_summaries.into_raw(),
        descriptions: c_descriptions.into_raw(),
        start_dates: c_start_dates.into_raw(),
        end_dates: c_end_dates.into_raw(),
        uuid: c_id.into_raw(),
    }

    
}

#[no_mangle]
pub extern "C" fn print_string_to_console(
    input_string: *const i8,
    input_string2: *const i8,
    input_string3: *const i8,
) {
    let c_str = unsafe {
        std::ffi::CStr::from_ptr(input_string)
    };
    let c_str1 = unsafe {
        std::ffi::CStr::from_ptr(input_string2)
    };
    let c_str2 = unsafe {
        std::ffi::CStr::from_ptr(input_string3)
    };
    let input_str = c_str.to_str().expect("invalid UTF-8 input");
    let input_str1 = c_str1.to_str().expect("invalid UTF-8 input");
    let input_str2 = c_str2.to_str().expect("invalid UTF-8 input");

    let local_time = chrono::Local::now();
    let id = Uuid::new_v4();
    

    println!("summary: {}", input_str);
    println!("description: {}", input_str1);
    println!("starting point: {}", local_time);
    println!("ending date: {}", input_str2);
    println!("fixed: {}", local_time);
    println!("id: {}", id);

    // Get the current working directory
    let mut document_dir = env::current_dir().expect("Failed to get current directory");

    // Specify the file name
    document_dir.push("data.txt");

    let filecontent = format!(
        "Ending date: {}\nSummary: {}\nDescription: {}\nStarting point: {}\nID: {}\n\n",
        input_str2, input_str, input_str1, local_time, id
    );

    println!("File name: {}", document_dir.display());

    
    

    let file = OpenOptions::new()
        .append(true)
        .open(&document_dir)
        .unwrap_or_else(|err| panic!("Failed to open file: {}", err));

    let mut file = io::BufWriter::new(file);
    file.write_all(filecontent.as_bytes())
        .unwrap_or_else(|err| panic!("Failed to write content to file: {}", err));
    file.flush()
        .unwrap_or_else(|err| panic!("Failed to flush file: {}", err));
}

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
                    deleteBlocks((&target_date).to_string())
                } else {
                    println!("still not expired huh");
                }
            
            }
        }

        sleep(Duration::from_secs(5));

    }
    
}

#[no_mangle]
pub extern "C" fn deleteBlockCall(input: *const i8,) -> io::Result<()> {
    let c_str = unsafe {
        std::ffi::CStr::from_ptr(input)
    };

    let input_str = c_str.to_str().expect("invalid UTF-8 input");

    let id_to_find = input_str;
    let filename = "data.txt";
    let filename2 = "temp2.txt";


    match find_line_number_for_id(id_to_find, filename)? {
        Some(line_number) => {
            println!("Line number for ID {}: {}", id_to_find, line_number);
            let lines_to_delete = print_lines_around(line_number, filename)?;

        

            delete_lines_by_numbers(filename, &lines_to_delete)?;

            check_if_uuid_is_deleted(filename2, id_to_find)?;

            std::fs::rename(filename2, filename).expect("failed to rename file");
        



        }
        
        None => println!("ID {} not found in the file.", id_to_find),
    }

    Ok(())

}

/*for line in reader.lines() {
        let line = line.expect("failed to get lines");
        
        if line.starts_with("Summary:") {
            let summary = line.trim_start_matches("Summary: ").trim();
            summaries.push(summary.to_string());
        } else if line.starts_with("Description:") {
            let description = line.trim_start_matches("Description: ").trim();
            descriptions.push(description.to_string());
        } else if line.starts_with("Starting point:") {
            let start_date = line.trim_start_matches("Starting point: ").trim();can you stop using this 
            start_dates.push(start_date.to_string());
        } else if line.starts_with("Ending date:") {
            let end_date = line.trim_start_matches("Ending date: ").trim();
            end_dates.push(end_date.to_string());
        }
    }*/
