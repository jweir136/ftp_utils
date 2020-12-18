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
                            match file.write_all(&mut buff[0..size]) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;
    use std::fs::File;
    use std::slice;
    use std::io;

    #[test]
    fn stream_to_file_test() {
        let mut f1 = File::open("newtest.txt").unwrap();

        match stream_to_file(&mut f1, "test.txt") {
            Ok(_) => {

            },
            Err(_) => {
                panic!();
            }
        };
    }
}