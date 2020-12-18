use crate::errors::FTPError;
use std::io::{Write, Read};
use std::fs::File;

pub fn stream_to_file<W: Write + Read>(stream: &mut W, filepath: &str) -> Result<(), FTPError> {
    match File::create(filepath) {
        Ok(mut file) => {
            loop {
                let mut buff: [u8; 256] = [0; 256];

                match stream.read(&mut buff) {
                    Ok(size) => {
                        if size == 0 {
                            return Ok(())
                        } else {
                            match file.write_all(&mut buff) {
                                Ok(_) => {
                                    // intentionally do nothing.
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
            }
        },
        Err(_) => {
            Err(FTPError::CannotCreateFile)
        }
    }
}

// TODO : Add tests