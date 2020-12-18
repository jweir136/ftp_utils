use crate::errors::FTPError;
use std::io::{Write, Read};
use std::fs::File;

/// Description:
///     This function is used to continously write the bytes from a streamable item to a file.
///
/// Arguments:
///     stream: &mut Write      =>  This arugment is used to specify the item to write the bytes to.
///                                     All items passed must implement the 'Write' trait.
///     filename: &str          =>  This argument is the complete path and filename to the file
///                                     to create and read the bytes from.
///
/// Points of Failure:
///     CannotWrite       =>      This error occurs whenever the streamable item cannot have bytes written to it.
///     CannotRead        =>      This occurs whenever the instant of the file cannot be read from.
///     CannotCreateFile  =>      This occurs whenever the given file cannot be created.
///
/// Dev. Notes:
///     (1)     The ```return``` statements are intentional. They were used over the most 'rust prefered'
///                 method. The ```return``` statements are used to that the multiple ```match``` statements may be used 
///                 may be used in order to account for custom error handling of all errors.
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