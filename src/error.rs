// Copyright (c) 2014-2015 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

use std::{self, io, fmt};
use std::convert::From;
use quick_protobuf;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Pbf(quick_protobuf::errors::Error),
    UnsupportedData,
    InvalidData,
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref e) => write!(f, "Io({})", e),
            Error::Pbf(ref e) => write!(f, "Pbf({})", e),
            Error::UnsupportedData => write!(f, "UnsupportedData"),
            Error::InvalidData => write!(f, "InvalidData"),
        }
    }
}
impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref e) => e.description(),
            Error::Pbf(ref e) => e.description(),
            Error::UnsupportedData => "Unsupported data",
            Error::InvalidData => "Invalid data",
        }
    }
    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            Error::Io(ref e) => Some(e),
            Error::Pbf(ref e) => Some(e),
            Error::UnsupportedData | Error::InvalidData => None,
        }
    }
}
impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}
impl From<quick_protobuf::errors::Error> for Error {
    fn from(err: quick_protobuf::errors::Error) -> Error {
        Error::Pbf(err)
    }
}
