use std::fs::File;
use std::io::{BufRead, BufReader};
use std::os::raw::c_char;
use std::path::PathBuf;
use std::path::Path;
use std::ffi::CString;




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