use std::error::Error;
use std::error::FromError;
use std::io::IoError;
use protobuf;

#[deriving(Show)]
pub enum OsmPbfError {
    Io(IoError),
    Pbf(protobuf::ProtobufError),
    UnsupportedData,
}
impl Error for OsmPbfError {
    fn description(&self) -> &str {
        match *self {
            OsmPbfError::Io(ref e) => e.description(),
            OsmPbfError::Pbf(ref e) => e.description(),
            OsmPbfError::UnsupportedData => "Unsupported data",
        }
    }
    fn detail(&self) -> Option<String> {
        match *self {
            OsmPbfError::Io(ref e) => e.detail(),
            OsmPbfError::Pbf(ref e) => e.detail(),
            OsmPbfError::UnsupportedData => None,
        }
    }
    fn cause(&self) -> Option<&Error> {
        match *self {
            OsmPbfError::Io(ref e) => Some(e as &Error),
            OsmPbfError::Pbf(ref e) => Some(e as &Error),
            OsmPbfError::UnsupportedData => None,
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
