// Copyright (c) 2014-2015 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

use protobuf;
use std::convert::From;
use std::{self, fmt, io};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Pbf(protobuf::ProtobufError),
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
    fn cause(&self) -> Option<&dyn std::error::Error> {
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

impl From<protobuf::ProtobufError> for Error {
    fn from(err: protobuf::ProtobufError) -> Error {
        Error::Pbf(err)
    }
}
