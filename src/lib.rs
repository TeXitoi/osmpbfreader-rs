extern crate protobuf;
extern crate flate2;

use std::error::Error;
use std::error::FromError;
use std::io::IoError;

#[allow(non_snake_case)]
pub mod fileformat;
pub mod osmformat;

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
            OsmPbfError::Pbf(_) => "Protobuf Error",
            OsmPbfError::UnsupportedData => "Unsupported data",
        }
    }
    fn detail(&self) -> Option<String> {
        match *self {
            OsmPbfError::Io(ref e) => e.detail(),
            OsmPbfError::Pbf(_) => None,
            OsmPbfError::UnsupportedData => None,
        }
    }
    fn cause(&self) -> Option<&Error> {
        match *self {
            OsmPbfError::Io(ref e) => Some(e as &Error),
            OsmPbfError::Pbf(_) => None,
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


pub struct OsmPbfReader<R> {
    buf: Vec<u8>,
    r: R,
}

impl<R: Reader> OsmPbfReader<R> {
    pub fn with_reader(r: R) -> OsmPbfReader<R> {
        OsmPbfReader {
            buf: vec![],
            r: r,
        }
    }
    fn push(&mut self, sz: uint) -> Result<(), OsmPbfError> {
        self.buf.clear();
        let readed = try!(self.r.push_at_least(sz, sz, &mut self.buf));
        assert_eq!(sz, readed);
        Ok(())
    }
    pub fn read_header(&mut self) -> Result<fileformat::BlobHeader, OsmPbfError> {
        let sz = try!(self.r.read_be_u32()) as uint;
        try!(self.push(sz));
        Ok(try!(protobuf::parse_from_bytes(self.buf.as_slice())))
    }
    fn read_primitive_block(&mut self, blob: fileformat::Blob)
                            -> Result<osmformat::PrimitiveBlock, OsmPbfError>
    {
        if blob.has_raw() {
            println!("raw");
            Ok(try!(protobuf::parse_from_bytes(blob.get_raw())))
        } else if blob.has_zlib_data() {
            use flate2::reader::ZlibDecoder;

            println!("zlib");
            let r = std::io::BufReader::new(blob.get_zlib_data());
            let mut zr = ZlibDecoder::new(r);
            let buf = try!(zr.read_to_end());// TODO use self.buf
            Ok(try!(protobuf::parse_from_bytes(buf.as_slice())))
        } else {
            Err(OsmPbfError::UnsupportedData)
        }
    }
    pub fn read_blob(&mut self, header: &fileformat::BlobHeader)
                     -> Result<(), OsmPbfError>
    {
        let sz = header.get_datasize() as uint;
        try!(self.push(sz));
        let blob: fileformat::Blob = try!(protobuf::parse_from_bytes(self.buf.as_slice()));
        if header.get_field_type() == "OSMData" {
            let primitive_block = try!(self.read_primitive_block(blob));
        } else if header.get_field_type() == "OSMHeader" {
            println!("OSMHeader");
        } else {
            println!("Unknown type: {}", header.get_field_type());
        }
        Ok(())
    }
}
