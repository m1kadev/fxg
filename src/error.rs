use std::io;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    ParsingError(String),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IoError(value)
    }
}
