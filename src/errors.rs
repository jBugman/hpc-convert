use std::io;
use std::num;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    DisparityError,
    FileError(io::Error),
    FormatError,
    FormatterError(fmt::Error),
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

impl From<fmt::Error> for Error {
    fn from(err: fmt::Error) -> Error {
        Error::FormatterError(err)
    }
}

