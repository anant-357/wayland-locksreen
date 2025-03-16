use std::fmt;
use std::io;

#[derive(Debug)]
pub enum ParseError {
    Io(io::Error),
    Utf8(std::string::FromUtf8Error),
    UnexpectedEndOfBuffer,
    InvalidArgument
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Io(err) => write!(f, "IO error: {}", err),
            ParseError::Utf8(err) => write!(f, "UTF-8 conversion error: {}", err),
            ParseError::UnexpectedEndOfBuffer => write!(f, "Unexpected end of buffer"),
            ParseError::InvalidArgument => write!(f, "Argument invalid")
        }
    }
}

impl From<io::Error> for ParseError {
    fn from(err: io::Error) -> Self {
        ParseError::Io(err)
    }
}

impl From<std::string::FromUtf8Error> for ParseError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        ParseError::Utf8(err)
    }
}

pub type ParseResult<T> = Result<T, ParseError>;
