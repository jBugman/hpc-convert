use std::io;
use std::num;

#[derive(Debug)]
pub enum Error {
    FormatError,
    FileError(io::Error),
    ParseError(num::ParseIntError),
}

impl From<num::ParseIntError> for Error {
    fn from(err: num::ParseIntError) -> Error {
        Error::ParseError(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::FileError(err)
    }
}
