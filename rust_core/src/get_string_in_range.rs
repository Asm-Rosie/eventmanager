use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::mem;

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
