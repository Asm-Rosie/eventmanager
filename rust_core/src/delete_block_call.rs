use std::fs::File;
use std::io::{self, prelude::*, BufReader, BufWriter};
use std::io::ErrorKind;
use std::io::Result;
use std::env;



#[no_mangle]
pub extern "C" fn delete_block_call(input: *const i8,) -> io::Result<()> {
    let c_str = unsafe {
        std::ffi::CStr::from_ptr(input)
    };

    let input_str = c_str.to_str().expect("invalid UTF-8 input");

    let id_to_find = input_str;

    let mut filename = "data.txt".to_string();

    if cfg!(target_os = "windows") {
        if let Some(document_dir) = dirs::document_dir() {
            
            filename = document_dir.join("eventmanager_data").join("data.txt").to_str().unwrap().to_string();
            println!("file path: {}", filename);
        } else {
            println!("error");
        }
    }

    
    let filename2 = "temp2.txt"; 


    match find_line_number_for_id(id_to_find, &filename)? {
        Some(line_number) => {
            println!("Line number for ID {}: {}", id_to_find, line_number);
            let lines_to_delete = print_lines_around(line_number, &filename)?;

            delete_lines_by_numbers(&filename, &lines_to_delete)?;

            check_if_uuid_is_deleted(filename2, id_to_find)?;

            std::fs::rename(filename2, &filename).expect("failed to rename file");
        
        }
        
        None => println!("ID {} not found in the file.", id_to_find),
    }

    Ok(())

}

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