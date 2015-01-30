// Copyright (c) 2014 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

use std::error::Error;
use std::error::FromError;
use std::old_io::IoError;
use std::fmt;
use protobuf;

pub enum OsmPbfError {
    Io(IoError),
    Pbf(protobuf::ProtobufError),
    UnsupportedData,
    InvalidData,
}
impl fmt::Display for OsmPbfError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OsmPbfError::Io(ref e) => write!(f, "Io({})", e),
            OsmPbfError::Pbf(ref e) => write!(f, "Pbf({})", e),
            OsmPbfError::UnsupportedData => write!(f, "UnsupportedData"),
            OsmPbfError::InvalidData => write!(f, "InvalidData"),
        }
    }
}
impl Error for OsmPbfError {
    fn description(&self) -> &str {
        match *self {
            OsmPbfError::Io(ref e) => e.description(),
            OsmPbfError::Pbf(ref e) => e.description(),
            OsmPbfError::UnsupportedData => "Unsupported data",
            OsmPbfError::InvalidData => "Invalid data",
        }
    }
    fn cause(&self) -> Option<&Error> {
        match *self {
            OsmPbfError::Io(ref e) => Some(e as &Error),
            OsmPbfError::Pbf(ref e) => Some(e as &Error),
            OsmPbfError::UnsupportedData => None,
            OsmPbfError::InvalidData => None,
        }
    }
}
impl FromError<IoError> for OsmPbfError {
    fn from_error(err: IoError) -> OsmPbfError {
        OsmPbfError::Io(err)
    }
}
impl FromError<protobuf::ProtobufError> for OsmPbfError {
    fn from_error(err: protobuf::ProtobufError) -> OsmPbfError {
        OsmPbfError::Pbf(err)
    }
}
