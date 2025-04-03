use std::path::PathBuf;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum CliError {
    IoError(std::io::Error),
    FileNotFound(PathBuf),
    BadLen(usize),
    InvalidUsage,
    ParseError(ParseIntError),
    OutOfBounds(OutOfBoundsParams),
}

#[derive(Debug)]
pub struct OutOfBoundsParams {
    pub idx: usize,
    pub len: usize,
}

