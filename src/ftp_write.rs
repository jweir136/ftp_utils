use std::io::{Write, Read};
use std::fs::File;
use crate::errors::FTPError;

pub fn stream_file<W: Write>(stream: &mut W, filename: &str) -> Result<(), FTPError> {
    match File::open(filename) {
        Ok(mut file) => {
            let mut buff: [u8; 256] = [0; 256];

            loop {
                match file.read(&mut buff) {
                    Ok(size) => {
                        if size == 0 {
                            return Ok(());
                        } else {
                            match stream.write_all(&buff[0..size]) {
                                Ok(_) => {

                                },
                                Err(_) => {
                                    return Err(FTPError::CannotWrite);
                                }
                            }
                        }
                    },
                    Err(_) => {
                        return Err(FTPError::CannotRead);
                    }
                }
            };

            Ok(())
        },
        Err(_) => {
            Err(FTPError::CannotOpenFile)
        }
    }
}