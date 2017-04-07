//! This module contains the various error types used by this crate.

use std::fmt;
use std::error;
use std::io;
use std::num;

#[derive(Debug)]
pub enum GridError {
    OutOfBoundCoords,
}


impl fmt::Display for GridError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GridError::OutOfBoundCoords => write!(f, "Error: out of bound index"),
        }
    }
}

impl error::Error for GridError {
    fn description(&self) -> &str {
        match *self {
            GridError::OutOfBoundCoords => "out of bound index",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

#[derive(Debug)]
pub enum FileParsingError {
    UnknownFormat,
    IoError,
    // EmptyFile, TODO: add this Later
    IncompleteFile,
    RuleParsingError,
    CoordParsingError,
    OutOfBoundCoords(GridError),
}

impl fmt::Display for FileParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileParsingError::UnknownFormat => write!(f, "Unknown file format"),
            FileParsingError::IoError => write!(f, "IO error"),
            FileParsingError::IncompleteFile => write!(f, "Incomplete or empty file"), // TODO: separately handle the case where the file is empty
            FileParsingError::RuleParsingError => write!(f, "Invalid ruleset"),
            FileParsingError::CoordParsingError => write!(f, "Invalid coordinates"),
            FileParsingError::OutOfBoundCoords(ref err) => write!(f, "{}", err),
        }
    }
}

impl error::Error for FileParsingError {
    fn description(&self) -> &str {
        match *self {
            FileParsingError::UnknownFormat => "unknown file format",
            FileParsingError::IoError => "IO error",
            FileParsingError::IncompleteFile => "incomplete or empty file", // TODO: separately handle the case where the file is empty
            FileParsingError::RuleParsingError => "invalid ruleset",
            FileParsingError::CoordParsingError => "invalid coordinates",
            FileParsingError::OutOfBoundCoords(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            FileParsingError::OutOfBoundCoords(ref err) => err.cause(),
            _ => None,
        }
    }
}

impl From<io::Error> for FileParsingError {
    fn from(_: io::Error) -> FileParsingError {
        FileParsingError::IoError
    }
}

impl From<num::ParseIntError> for FileParsingError {
    fn from(_: num::ParseIntError) -> FileParsingError {
        FileParsingError::CoordParsingError
    }
}

impl From<GridError> for FileParsingError {
    fn from(err: GridError) -> FileParsingError {
        FileParsingError::OutOfBoundCoords(err)
    }
}
