use std::io::{Write, Read};
use std::fs::File;
use crate::errors::FTPErrors;

fn write_file_to_string(filepath: &str) -> Result<String, FTPErrors> {
    match File::open(filepath) {
        Ok(mut file) => {
            let mut content: String = String::new();
            
            match file.read_to_string(&mut content) {
                Ok(_) => {
                   Ok(content) 
                },
                Err(_) => {
                    Err(FTPErrors::CannotRead)
                }
            }
        },
        Err(_) => {
            Err(FTPErrors::CannotOpenFile)
        }
    }
}