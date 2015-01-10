// Copyright (c) 2014 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

//#![deny(warnings)]
#![feature(unboxed_closures)]

extern crate protobuf;
extern crate flate2;

pub use objects::{OsmObj, Node, Way, Relation, Ref, OsmId, Tags};
pub use error::OsmPbfError;

use std::error::FromError;

#[allow(non_snake_case, unstable)] pub mod fileformat;
#[allow(unstable)] pub mod osmformat;

pub mod error;
pub mod objects;
pub mod groups;
pub mod blocks;

pub struct OsmPbfReader<R> {
    buf: Vec<u8>,
    r: R,
    finished: bool,
}

impl<R: Reader> OsmPbfReader<R> {
    pub fn with_reader(r: R) -> OsmPbfReader<R> {
        OsmPbfReader {
            buf: vec![],
            r: r,
            finished: false,
        }
    }
    pub fn primitive_blocks<'a>(&'a mut self) -> PrimitiveBlocks<'a, R> {
        PrimitiveBlocks { opr: self }
    }

    fn push(&mut self, sz: usize) -> Result<(), OsmPbfError> {
        self.buf.clear();
        let readed = try!(self.r.push_at_least(sz, sz, &mut self.buf));
        assert_eq!(sz, readed);
        Ok(())
    }
    fn read_primitive_block(&mut self, blob: fileformat::Blob)
                            -> Result<osmformat::PrimitiveBlock, OsmPbfError>
    {
        if blob.has_raw() {
            Ok(try!(protobuf::parse_from_bytes(blob.get_raw())))
        } else if blob.has_zlib_data() {
            use flate2::reader::ZlibDecoder;
            let r = std::io::BufReader::new(blob.get_zlib_data());
            let mut zr = ZlibDecoder::new(r);
            Ok(try!(protobuf::parse_from_reader(&mut zr as &mut Reader)))
        } else {
            Err(OsmPbfError::UnsupportedData)
        }
    }
    fn try_primitive_block(&mut self, sz: usize)
                           -> Result<Option<osmformat::PrimitiveBlock>, OsmPbfError>
    {
        try!(self.push(sz));
        let header: fileformat::BlobHeader =
            try!(protobuf::parse_from_bytes(&*self.buf));
        let sz = header.get_datasize() as usize;
        try!(self.push(sz));
        let blob: fileformat::Blob = try!(protobuf::parse_from_bytes(&*self.buf));
        let primitive_opt = if header.get_field_type() == "OSMData" {
            Some(try!(self.read_primitive_block(blob)))
        } else if header.get_field_type() == "OSMHeader" {
            None
        } else {
            println!("Unknown type: {}", header.get_field_type());
            None
        };
        Ok(primitive_opt)
    }
    fn next_primitive_block(&mut self)
                            -> Option<Result<osmformat::PrimitiveBlock, OsmPbfError>>
    {
        use std::io::IoErrorKind;
        if self.finished { return None; }
        let sz = match self.r.read_be_u32() {
            Ok(sz) if sz > 64 * 1024 => return Some(Err(OsmPbfError::InvalidData)),
            Ok(sz) => sz,
            Err(ref e) if e.kind == IoErrorKind::EndOfFile => {
                self.finished = true;
                return None;
            }
            Err(e) => {
                self.finished = true;
                return Some(Err(FromError::from_error(e)));
            }
        } as usize;
        match self.try_primitive_block(sz) {
            Ok(Some(p)) => Some(Ok(p)),
            Ok(None) => self.next_primitive_block(),
            Err(e) => {
                self.finished = true;
                Some(Err(e))
            }
        }
    }
}

pub struct PrimitiveBlocks<'a, R: 'a> {
    opr: &'a mut OsmPbfReader<R>
}
impl<'a, R: Reader> Iterator for PrimitiveBlocks<'a, R> {
    type Item = Result<osmformat::PrimitiveBlock, OsmPbfError>;
    fn next(&mut self) -> Option<Result<osmformat::PrimitiveBlock, OsmPbfError>> {
        self.opr.next_primitive_block()
    }
}
