// Copyright (c) 2014 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

//#![deny(warnings)]

extern crate protobuf;
extern crate flate2;
extern crate byteorder;

pub use objects::{OsmObj, Node, Way, Relation, Ref, OsmId, Tags};
pub use error::OsmPbfError;

#[allow(non_snake_case)] pub mod fileformat;
pub mod osmformat;

pub mod error;
pub mod objects;
pub mod groups;
pub mod blocks;

pub struct OsmPbfReader<R> {
    buf: Vec<u8>,
    r: R,
    finished: bool,
}

impl<R: std::io::Read> OsmPbfReader<R> {
    pub fn with_reader(r: R) -> OsmPbfReader<R> {
        OsmPbfReader {
            buf: vec![],
            r: r,
            finished: false,
        }
    }
    pub fn blobs<'a>(&'a mut self) -> Blobs<'a, R> {
        Blobs { opr: self }
    }
    pub fn primitive_blocks<'a>(&'a mut self) -> PrimitiveBlocks<'a, R> {
        fn and_then_primitive_block(blob_res: Result<fileformat::Blob, OsmPbfError>)
                                    -> Result<osmformat::PrimitiveBlock, OsmPbfError>
        {
            blob_res.and_then(|b| primitive_block_from_blob(&b))
        }
        self.blobs().map(and_then_primitive_block)
    }

    fn push(&mut self, sz: u64) -> Result<(), OsmPbfError> {
        use std::io::Read;
        self.buf.clear();
        try!(self.r.by_ref().take(sz).read_to_end(&mut self.buf));
        assert_eq!(sz, self.buf.len() as u64);
        Ok(())
    }
    fn try_blob(&mut self, sz: u64)
                 -> Result<Option<fileformat::Blob>, OsmPbfError>
    {
        try!(self.push(sz));
        let header: fileformat::BlobHeader =
            try!(protobuf::parse_from_bytes(&self.buf));
        let sz = header.get_datasize() as u64;
        try!(self.push(sz));
        let blob: fileformat::Blob = try!(protobuf::parse_from_bytes(&self.buf));
        if header.get_field_type() == "OSMData" {
            Ok(Some(blob))
        } else if header.get_field_type() == "OSMHeader" {
            Ok(None)
        } else {
            println!("Unknown type: {}", header.get_field_type());
            Ok(None)
        }
    }
    fn next_blob(&mut self) -> Option<Result<fileformat::Blob, OsmPbfError>>
    {
        use byteorder::{BigEndian, ReadBytesExt};
        use byteorder::Error::{UnexpectedEOF, Io};
        if self.finished { return None; }
        let sz = match self.r.read_u32::<BigEndian>() {
            Ok(sz) if sz > 64 * 1024 => return Some(Err(OsmPbfError::InvalidData)),
            Ok(sz) => sz,
            Err(UnexpectedEOF) => {
                self.finished = true;
                return None;
            }
            Err(Io(e)) => {
                self.finished = true;
                return Some(Err(std::convert::From::from(e)));
            }
        } as u64;
        match self.try_blob(sz) {
            Ok(Some(p)) => Some(Ok(p)),
            Ok(None) => self.next_blob(),
            Err(e) => {
                self.finished = true;
                Some(Err(e))
            }
        }
    }
}

pub struct Blobs<'a, R: 'a> {
    opr: &'a mut OsmPbfReader<R>
}
impl<'a, R: std::io::Read> Iterator for Blobs<'a, R> {
    type Item = Result<fileformat::Blob, OsmPbfError>;
    fn next(&mut self) -> Option<Self::Item> {
        self.opr.next_blob()
    }
}

pub type PrimitiveBlocks<'a, R: 'a> =
    std::iter::Map<Blobs<'a, R>, fn(Result<fileformat::Blob, OsmPbfError>)
                                    -> Result<osmformat::PrimitiveBlock, OsmPbfError>
    >;

pub fn primitive_block_from_blob(blob: &fileformat::Blob)
                                 -> Result<osmformat::PrimitiveBlock, OsmPbfError>
{
    use std::convert::From;
    if blob.has_raw() {
        protobuf::parse_from_bytes(blob.get_raw()).map_err(From::from)
    } else if blob.has_zlib_data() {
        use flate2::read::ZlibDecoder;
        let r = std::io::Cursor::new(blob.get_zlib_data());
        let mut zr = ZlibDecoder::new(r);
        protobuf::parse_from_reader(&mut zr).map_err(From::from)
    } else {
        Err(OsmPbfError::UnsupportedData)
    }
}
