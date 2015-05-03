// Copyright (c) 2014-2015 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

//! This crate provide an interface to easily read [OpenStreetMap PBF
//! files](http://wiki.openstreetmap.org/wiki/PBF_Format).  Its main
//! inspiration is
//! [libosmpbfreader](https://github.com/CanalTP/libosmpbfreader).
//!
//! # Usage
//!
//! You can add `osmpbfreader` to your dependencies in your project's
//! `Cargo.toml`.
//!
//! ```toml
//! [dependencies.osmpbfreader]
//! git = "https://github.com/TeXitoi/osmpbfreader-rs"
//! ```
//!
//! and this to your crate root:
//!
//! ```rust
//! extern crate osmpbfreader;
//! ```
//!
//! # Readding without error handling
//!
//! The easiest way to read a PBF file is to directly iterate on the
//! `OsmObj`.
//!
//! ```rust
//! let path = std::path::Path::new("/dev/null");
//! let r = std::fs::File::open(&path).unwrap();
//! let mut pbf = osmpbfreader::OsmPbfReader::with_reader(r);
//! for obj in pbf.iter() {
//!     println!("{:?}", obj);
//! }
//! ```
//!
//! Notice that, in case of any error, it'll panic.
//!
//! # Readding with error handling
//!
//! To manage error handling, a little more work is needed.  First,
//! iteration on the different blocks is done, and then, for each
//! blocks, after error handling, iteration on the `OsmObj` can be
//! done.
//!
//! ```rust
//! use std::process::exit;
//! use osmpbfreader::blocks;
//! let path = std::path::Path::new("/dev/null");
//! let r = std::fs::File::open(&path).unwrap();
//! let mut pbf = osmpbfreader::OsmPbfReader::with_reader(r);
//! for block in pbf.primitive_blocks() {
//!     // error handling:
//!     let block = block.unwrap_or_else(|e| {println!("{:?}", e); exit(1)});
//!
//!     for obj in blocks::iter(&block) {
//!         println!("{:?}", obj);
//!     }
//! }
//! ```
//!
//! # Into the details
//!
//! This crate is build around basic iterators on different parts of
//! the structure of the PBF format.  Then, several higher level
//! iterator are proposed.  It is then possible to iterate on the file
//! using the low level iterators.
//!
//! ```rust
//! use osmpbfreader::{primitive_block_from_blob, groups};
//! let path = std::path::Path::new("/dev/null");
//! let r = std::fs::File::open(&path).unwrap();
//! let mut pbf = osmpbfreader::OsmPbfReader::with_reader(r);
//! for block in pbf.blobs().map(|b| primitive_block_from_blob(&b.unwrap())) {
//!     let block = block.unwrap();
//!     for group in block.get_primitivegroup().iter() {
//!         for node in groups::simple_nodes(&group, &block) {
//!             println!("{:?}", node);
//!         }
//!         for node in groups::dense_nodes(&group, &block) {
//!             println!("{:?}", node);
//!         }
//!         for way in groups::ways(&group, &block) {
//!             println!("{:?}", way);
//!         }
//!         for relation in groups::relations(&group, &block) {
//!             println!("{:?}", relation);
//!         }
//!     }
//! }
//! ```
//!
//! Notice that `primitive_block_from_blob` can be costy as it
//! uncompress the blob.  Using some kind of parallel map can then
//! improve the reading speed of the PBF file.

//#![deny(missing_docs)]

extern crate protobuf;
extern crate flate2;
extern crate byteorder;

pub use objects::{OsmObj, Node, Way, Relation, Ref, OsmId, Tags};
pub use error::OsmPbfError;

/// Generated from protobuf.
#[allow(non_snake_case, missing_docs)] pub mod fileformat;

/// Generated from protobuf.
#[allow(missing_docs)] pub mod osmformat;

pub mod error;
pub mod objects;
pub mod groups;
pub mod blocks;
pub mod borrowed_iter;

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
    pub fn iter<'a>(&'a mut self) -> Box<Iterator<Item = OsmObj> + 'a> {
        let iter = self.primitive_blocks()
            .map(|r| r.unwrap())
            .flat_map(|b| borrowed_iter::borrowed_iter(blocks::iter, b));
        Box::new(iter)
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
