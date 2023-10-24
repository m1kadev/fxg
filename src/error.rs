use std::{io, ops::Range};

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    ParsingError {
        message: String,
        region: Range<usize>,
        source: String,
    },
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IoError(value)
    }
}
