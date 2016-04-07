// Copyright (c) 2014-2015 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

use fileformat;
use osmformat;
use error::OsmPbfError;
use blocks;
use objects::OsmObj;
use borrowed_iter;
use protobuf;
use std::convert::From;
use std::io::{self, Read};
use std::iter;

pub struct OsmPbfReader<R> {
    buf: Vec<u8>,
    r: R,
    finished: bool,
}

impl<R: io::Read> OsmPbfReader<R> {
    pub fn new(r: R) -> OsmPbfReader<R> {
        OsmPbfReader {
            buf: vec![],
            r: r,
            finished: false,
        }
    }
    pub fn into_inner(self) -> R {
        self.r
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
    pub fn iter<'a>(&'a mut self) -> Box<Iterator<Item = OsmObj> + 'a> {
        let iter = self.primitive_blocks()
            .map(|r| r.unwrap())
            .flat_map(|b| borrowed_iter::BorrowedIter::new(b, blocks::iter));
        Box::new(iter)
    }
    pub fn rewind(&mut self) -> Result<(), OsmPbfError>
        where R: io::Seek
    {
        try!(self.r.seek(io::SeekFrom::Start(0)));
        self.finished = false;
        Ok(())
    }

    fn push(&mut self, sz: u64) -> Result<(), OsmPbfError> {
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
        use std::io::ErrorKind;
        if self.finished { return None; }
        let sz = match self.r.read_u32::<BigEndian>() {
            Ok(sz) if sz > 64 * 1024 => return Some(Err(OsmPbfError::InvalidData)),
            Ok(sz) => sz,
            Err(ref e) if e.kind() == ErrorKind::UnexpectedEof => {
                self.finished = true;
                return None;
            }
            Err(e) => {
                self.finished = true;
                return Some(Err(From::from(e)));
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
impl<'a, R: io::Read> Iterator for Blobs<'a, R> {
    type Item = Result<fileformat::Blob, OsmPbfError>;
    fn next(&mut self) -> Option<Self::Item> {
        self.opr.next_blob()
    }
}

pub type PrimitiveBlocks<'a, R: 'a> =
    iter::Map<Blobs<'a, R>, fn(Result<fileformat::Blob, OsmPbfError>)
                               -> Result<osmformat::PrimitiveBlock, OsmPbfError>
    >;

pub fn primitive_block_from_blob(blob: &fileformat::Blob)
                                 -> Result<osmformat::PrimitiveBlock, OsmPbfError>
{
    if blob.has_raw() {
        protobuf::parse_from_bytes(blob.get_raw()).map_err(From::from)
    } else if blob.has_zlib_data() {
        use flate2::read::ZlibDecoder;
        let r = io::Cursor::new(blob.get_zlib_data());
        let mut zr = ZlibDecoder::new(r);
        protobuf::parse_from_reader(&mut zr).map_err(From::from)
    } else {
        Err(OsmPbfError::UnsupportedData)
    }
}
