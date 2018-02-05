//! This module contains the various kinds of errors used by this crate.

use std::error;
use std::fmt;
use std::io;
use std::num;

/// Represents the possible errors which can occur when manipulating
/// a `Grid`.
#[derive(Debug)]
pub enum GridErrorKind {
    OutOfBoundCoords,
}

impl fmt::Display for GridErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GridErrorKind::OutOfBoundCoords => write!(f, "Error: out of bound index"),
        }
    }
}

impl error::Error for GridErrorKind {
    fn description(&self) -> &str {
        match *self {
            GridErrorKind::OutOfBoundCoords => "out of bound index",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

/// Represents the possible errors which can occur when manipulating
/// a life file.
#[derive(Debug)]
pub enum FileParsingErrorKind {
    UnknownFormat,
    IoError,
    // EmptyFile, TODO: add this Later
    IncompleteFile,
    RuleParsingError,
    CoordParsingError,
    OutOfBoundCoords(GridErrorKind),
}

impl fmt::Display for FileParsingErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileParsingErrorKind::UnknownFormat => write!(f, "Unknown file format"),
            FileParsingErrorKind::IoError => write!(f, "IO error"),
            FileParsingErrorKind::IncompleteFile => write!(f, "Incomplete or empty file"), // TODO: separately handle the case where the file is empty
            FileParsingErrorKind::RuleParsingError => write!(f, "Invalid ruleset"),
            FileParsingErrorKind::CoordParsingError => write!(f, "Invalid coordinates"),
            FileParsingErrorKind::OutOfBoundCoords(ref err) => write!(f, "{}", err),
        }
    }
}

impl error::Error for FileParsingErrorKind {
    fn description(&self) -> &str {
        match *self {
            FileParsingErrorKind::UnknownFormat => "unknown file format",
            FileParsingErrorKind::IoError => "IO error",
            FileParsingErrorKind::IncompleteFile => "incomplete or empty file", // TODO: separately handle the case where the file is empty
            FileParsingErrorKind::RuleParsingError => "invalid ruleset",
            FileParsingErrorKind::CoordParsingError => "invalid coordinates",
            FileParsingErrorKind::OutOfBoundCoords(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            FileParsingErrorKind::OutOfBoundCoords(ref err) => err.cause(),
            _ => None,
        }
    }
}

impl From<io::Error> for FileParsingErrorKind {
    fn from(_: io::Error) -> FileParsingErrorKind {
        FileParsingErrorKind::IoError
    }
}

impl From<num::ParseIntError> for FileParsingErrorKind {
    fn from(_: num::ParseIntError) -> FileParsingErrorKind {
        FileParsingErrorKind::CoordParsingError
    }
}

impl From<GridErrorKind> for FileParsingErrorKind {
    fn from(err: GridErrorKind) -> FileParsingErrorKind {
        FileParsingErrorKind::OutOfBoundCoords(err)
    }
}
