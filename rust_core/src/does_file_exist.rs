use std::path::Path;
use std::fs::File;
use std::env;
use std::fs;
extern crate dirs;


pub static mut DATA_FILE_PATH_WIN: String = String::new();

#[no_mangle]
pub extern "C" fn does_file_exist() {
    let file_path = "data.txt";

    if cfg!(target_os = "windows") {
        println!("detected OS: Windows");

        if let Some(document_dir) = dirs::document_dir() {
            println!("document_dir: {:?}", document_dir);
            let data_folder = "eventmanager_data";
            let combined_path = document_dir.join(data_folder);

            if !combined_path.exists() {
                println!("file does not exist");
                if let Err(err) = fs::create_dir_all(&combined_path.parent().unwrap()) {
                    eprintln!("failed to create data folder: {}", err);
                    println!("{}", combined_path.display());
                    
                } else {
                    println!("created folder : {}", data_folder);

                }
            }

            let data_file_path = combined_path.join(file_path);

            unsafe {
                DATA_FILE_PATH_WIN = data_file_path.to_str().unwrap().to_string();
            }

            if !data_file_path.exists() {
                match File::create(&data_file_path) {
                    Ok(_) => println!("file created {}", file_path),
                    Err(err) => println!("file not created {}", err),
                }
            }


            
        } else {
            println!("no document_dir located ? ? ? ? ? ?");
        }
        
    }

    
}