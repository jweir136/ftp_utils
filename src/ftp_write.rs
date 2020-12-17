use std::io::{Write, Read};
use std::fs::File;
use crate::errors::FTPError;

/// Description:
///     This function is used to continously write the bytes from a file to a streamable item.
///
/// Arguments:
///     stream: &mut Write      =>  This arugment is used to specify the item to write the bytes to.
///                                     All items passed must implement the 'Write' trait.
///     filename: &str          =>  This argument is the complete path and filename to the file
///                                     to open and read the bytes from.
///
/// Points of Failure:
///     CannotWrite     =>      This error occurs whenever the streamable item cannot have bytes written to it.
///     CannotRead      =>      This occurs whenever the instant of the file cannot be read from.
///     CannotOpenFile  =>      This occurs whenever the given file cannot be opened or does not exist.
///
/// Possible Future Features:
///     (1)     Add a built-in utility to detect when a given file is 'too large'
///                 This could be used to prevent malicous attacks (such as zip-bombs).
///
/// Dev. Notes:
///     (1)     The ```return``` statements are intentional. They were used over the most 'rust prefered'
///                 method. The ```return``` statements are used to that the multiple ```match``` statements may be used 
///                 may be used in order to account for custom error handling of all errors.
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

    // NOTE: In order for this test to work, a "test.txt" file must be created.
    // This file is not included in the Github Repo
    #[test]
    fn stream_file_test() {
        let filename = "test.txt";
        let mut writer = File::create("newtest.txt").unwrap();

        match stream_file(&mut writer, filename) {
            Ok(_) => { },
            _ => { panic!(); }
        };
    }
}