use std::path::Path;
use std::fs::File;


#[no_mangle]
pub extern "C" fn does_file_exist() {
    let file_path = "data.txt";

    if !std::path::Path::new(&file_path).exists() {
        File::create(&file_path).expect("failed to create file");
    } else {
        println!("file already exists");
    }
}