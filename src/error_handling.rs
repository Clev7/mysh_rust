use std::path::PathBuf;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum CliError {
    IoError(std::io::Error),
    FileNotFound(PathBuf),
    BadLen(usize),
    InvalidUsage(&'static str),
    ParseError(ParseIntError),
    OutOfBounds(OutOfBoundsParams),
}

#[derive(Debug)]
pub struct OutOfBoundsParams {
    pub idx: usize,
    pub len: usize,
}

pub fn handle_err(err: CliError) {
    use CliError::*;
    match err {
        IoError(e) => eprintln!("IOError({:?})", e),
        FileNotFound(file_path) => eprintln!("File not found: {:?}", file_path),
        BadLen(arg) => eprintln!("Incorrect length of arguments: {:?}", arg),
        ParseError(arg) => eprintln!("An error occurred while parsing argument \"{arg}\""),
        OutOfBounds(OutOfBoundsParams { idx, len }) => {
            eprintln!("Index {idx} out of bounds for length {len}")
        },
        InvalidUsage(usage) => eprintln!("{:?}", usage)
    }
}
