use std::fmt;
use std::error;
use std::io;
use std::num;

#[derive(Debug)]
pub enum GridError {
    OutOfBoundCoords,
}


impl fmt::Display for GridError {
    // TODO: implementation
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl error::Error for GridError {
    // TODO: implementation
    fn description(&self) -> &str {
        unimplemented!()
    }

    // TODO: implementation
    fn cause(&self) -> Option<&error::Error> {
        unimplemented!()
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
    OutOfBoundCoords,
}

impl fmt::Display for FileParsingError {
    // TODO: implementation
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl error::Error for FileParsingError {
    // TODO: implementation
    fn description(&self) -> &str {
        unimplemented!()
    }

    // TODO: implementation
    fn cause(&self) -> Option<&error::Error> {
        unimplemented!()
    }
}

impl From<io::Error> for FileParsingError {
    fn from(err: io::Error) -> FileParsingError {
        FileParsingError::IoError
    }
}

impl From<num::ParseIntError> for FileParsingError {
    fn from(err: num::ParseIntError) -> FileParsingError {
        FileParsingError::CoordParsingError
    }
}

impl From<GridError> for FileParsingError {
    fn from(err: GridError) -> FileParsingError {
        FileParsingError::OutOfBoundCoords
    }
}
