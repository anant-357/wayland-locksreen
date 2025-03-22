use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    ParseError(String),
    Io(io::Error),
    Utf8(std::string::FromUtf8Error),
    UnexpectedEndOfBuffer,
    InvalidArgument,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseError(s) => write!(f, "Parse error: {s}"),
            Self::Io(err) => write!(f, "IO error: {}", err),
            Self::Utf8(err) => write!(f, "UTF-8 conversion error: {}", err),
            Self::UnexpectedEndOfBuffer => write!(f, "Unexpected end of buffer"),
            Self::InvalidArgument => write!(f, "Argument invalid"),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Self::Utf8(err)
    }
}

pub type WaylandResult<T> = Result<T, Error>;
