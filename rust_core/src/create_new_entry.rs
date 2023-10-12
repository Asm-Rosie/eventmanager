use std::fs::File;
use std::io::prelude::*;
use uuid::Uuid;
use chrono::prelude::*;
use std::env;
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;
use std::fs;
use std::fs::OpenOptions;
use std::io::BufWriter;


#[no_mangle]
pub extern "C" fn create_new_entry(
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

    let local = chrono::Local::now();
    let date = local.date();
    let date_time = date.format("%Y-%m-%d").to_string();
    let id = Uuid::new_v4();
    

    println!("summary: {}", input_str);
    println!("description: {}", input_str1);
    println!("starting point: {}", date_time);
    println!("ending date: {}", input_str2);
    println!("fixed: {}", date_time);
    println!("id: {}", id);

    // Get the current working directory
    let mut document_dir = env::current_dir().expect("Failed to get current directory");

    // Specify the file name
    document_dir.push("data.txt");

    let filecontent = format!(
        "Ending date: {}\nSummary: {}\nDescription: {}\nStarting point: {}\nID: {}\n\n",
        input_str2, input_str, input_str1, date_time, id
    );

    println!("File name: {}", document_dir.display());

    let file = OpenOptions::new()
        .append(true)
        .open(&document_dir)
        .unwrap_or_else(|err| panic!("Failed to open file: {}", err));

    let mut file = BufWriter::new(file);
    file.write_all(filecontent.as_bytes())
        .unwrap_or_else(|err| panic!("Failed to write content to file: {}", err));
    file.flush()
        .unwrap_or_else(|err| panic!("Failed to flush file: {}", err));
}