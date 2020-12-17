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
        },
        Err(_) => {
            Err(FTPError::CannotOpenFile)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};
    use std::fs::File;

    #[test]
    fn stream_file_test() {
        let filename = "test.txt";
        let mut writer = File::create("newtest.txt").unwrap();

        stream_file(&mut writer, filename);
    }
}