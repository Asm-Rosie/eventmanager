use std::fs::File;
use std::io::{self, prelude::*, BufReader, BufWriter};
use std::io::Result;
use std::mem;
use std::path::Path;
use std::path::PathBuf;




#[no_mangle]
pub extern "C" fn edit_content(
    input1: *const i8,
    input2: *const i8,
    input3: *const i8,
    input4: *const i8,
    input5: *const i8,
    input6: *const i8,
    input7: *const i8
)  -> io::Result<()> {
    let c_str = unsafe {
        std::ffi::CStr::from_ptr(input1)
    };
    let c_str1 = unsafe {
        std::ffi::CStr::from_ptr(input2)
    };
    let c_str2 = unsafe {
        std::ffi::CStr::from_ptr(input3)
    };
    let c_str3 = unsafe {
        std::ffi::CStr::from_ptr(input4)
    };
    let c_str4 = unsafe {
        std::ffi::CStr::from_ptr(input5)
    };
    let c_str5 = unsafe {
        std::ffi::CStr::from_ptr(input6)
    };
    let c_str6 = unsafe {
        std::ffi::CStr::from_ptr(input7)
    };

    let input_str = c_str.to_str().expect("invalid UTF-8 input");
    let input_str1 = c_str1.to_str().expect("invalid UTF-8 input");
    let input_str2 = c_str2.to_str().expect("invalid UTF-8 input");
    let input_str3 = c_str3.to_str().expect("invalid UTF-8 input");
    let input_str4 = c_str4.to_str().expect("invalid UTF-8 input");
    let input_str5 = c_str5.to_str().expect("invalid UTF-8 input");
    let input_str6 = c_str6.to_str().expect("invalid UTF-8 input");

    

    let file_path = "data.txt";
    let id_to_find = input_str6;

    match find_line_number_for_id(id_to_find, &file_path)? {
        Some(line_number) => {
            println!("Line number for ID {}: {}", id_to_find, line_number);
            let lines_allowed = print_lines_around(line_number, &file_path)?;

            let summary_before = input_str;
            let description_before = input_str1;
            let ending_date_before = input_str2;

            if let Err(err) = get_string_in_range(file_path, summary_before, description_before, ending_date_before, input_str3, input_str4, input_str5, lines_allowed) {
                eprintln!("Error: {}", err);
            }

            

            
        }

        None => println!("ID {} not found in the file.", id_to_find),
    }

    Ok(())

}



pub fn get_string_in_range(
    file_path: &str,
    summary_before: &str,
    description_before: &str,
    ending_date_before: &str,
    summary_after: &str,
    description_after: &str,
    ending_date_after: &str,
    line_range: Vec<usize>
) -> std::io::Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    
    let mut found_summary = false;
    let mut found_description = false;
    let mut found_ending_date = false;

    let mut contents = String::new();
    let mut starting_point = String::new();

    

    for (line_number, line_result) in reader.lines().enumerate() {
        let mut line = mem::replace(&mut line_result?, String::new()).trim().to_string();

        
        

        if line.contains("Starting point:") {
            starting_point = line.clone().to_string();
            contents.push_str(&starting_point);
            contents.push('\n');
            continue;
        }

        if line_range.contains(&(line_number + 1)) {
            if line.contains(ending_date_before) {
                let new_line = line.replace(ending_date_before, ending_date_after);
                found_ending_date = true;
                contents.push_str(&new_line);
                contents.push('\n');
                found_ending_date = true;
                println!("3. -------------\n{}\n-------------", ending_date_after); 
                
            } else if line.contains(summary_before) {
                let new_line = line.replace(summary_before, summary_after);
                found_description = true;
                contents.push_str(&new_line);
                contents.push('\n');
                
               
            } else if line.contains(description_before) {
                let new_line = line.replace(description_before, description_after);
                found_description = true;
                contents.push_str(&new_line);
                contents.push('\n');
                
                
            } else {
                contents.push_str(&line);
                contents.push('\n');
            }

            if found_summary && found_description && found_ending_date {
                
                println!("found all strings, breaking loop");
                break;
                
            }
        } else {
            contents.push_str(&line);
            contents.push('\n');
        }
    }

    
    
    let mut file = File::create(file_path)?;
    file.write_all(contents.as_bytes())?;
    

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