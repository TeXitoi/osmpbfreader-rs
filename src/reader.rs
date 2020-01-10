// Copyright (c) 2014-2017 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

//! Tools for reading a pbf file.

use blobs::{self, result_blob_into_iter};
use error::{Error, Result};
use fileformat::{Blob, BlobHeader};
use objects::{OsmId, OsmObj};
use osmformat::PrimitiveBlock;
use par_map::{self, ParMap};
use protobuf;
use std::collections::btree_map::BTreeMap;
use std::collections::BTreeSet;
use std::convert::From;
use std::io::{self, Read};
use std::iter;

/// Trait to allow generic objects (not just BTreeMap) in some methods.
pub trait StoreObjs {
    /// Insert given object at given key index.
    fn insert(&mut self, key: OsmId, value: OsmObj);
    /// Check if object contains the given key.
    fn contains_key(&self, key: &OsmId) -> bool;
}

impl StoreObjs for BTreeMap<OsmId, OsmObj> {
    fn insert(&mut self, key: OsmId, value: OsmObj) {
        self.insert(key, value);
    }

    fn contains_key(&self, key: &OsmId) -> bool {
        self.contains_key(key)
    }
}

/// The object to manage a pbf file.
pub struct OsmPbfReader<R> {
    buf: Vec<u8>,
    r: R,
    finished: bool,
}

impl<R: io::Read> OsmPbfReader<R> {
    /// Creates an OsmPbfReader from a Read object.
    pub fn new(r: R) -> OsmPbfReader<R> {
        OsmPbfReader {
            buf: vec![],
            r: r,
            finished: false,
        }
    }

    /// Returns an iterator on the OsmObj of the pbf file.
    ///
    /// # Example
    ///
    /// ```
    /// let mut pbf = osmpbfreader::OsmPbfReader::new(std::io::empty());
    /// for obj in pbf.iter().map(Result::unwrap) {
    ///     println!("{:?}", obj);
    /// }
    /// ```
    pub fn iter(&mut self) -> Iter<R> {
        Iter(self.blobs().flat_map(result_blob_into_iter))
    }

    /// Returns a parallel iterator on the OsmObj of the pbf file.
    ///
    /// Several threads decode in parallel the file.  The memory and
    /// CPU usage are guaranteed to be bounded even if the caller stop
    /// consuming items.
    ///
    /// # Example
    ///
    /// ```
    /// let mut pbf = osmpbfreader::OsmPbfReader::new(std::io::empty());
    /// for obj in pbf.par_iter().map(Result::unwrap) {
    ///     println!("{:?}", obj);
    /// }
    /// ```
    pub fn par_iter<'a>(&'a mut self) -> ParIter<'a, R> {
        ParIter(self.blobs().par_flat_map(result_blob_into_iter))
    }

    /// Rewinds the pbf file to the begining.
    ///
    /// Useful if you want to read several consecutive times the same
    /// file.
    ///
    /// # Example
    ///
    /// ```
    /// let mut cursor = std::io::Cursor::new([0, 0, 0]);
    /// cursor.set_position(2);
    /// let mut pbf = osmpbfreader::OsmPbfReader::new(cursor);
    /// pbf.rewind().unwrap();
    /// assert_eq!(pbf.into_inner().position(), 0);
    /// ```
    pub fn rewind(&mut self) -> Result<()>
    where
        R: io::Seek,
    {
        self.r.seek(io::SeekFrom::Start(0))?;
        self.finished = false;
        Ok(())
    }

    /// Same as `get_objs_and_deps` but generic.
    pub fn get_objs_and_deps_store<'a, F, T>(&mut self, mut pred: F, objects: &mut T) -> Result<()>
    where
        R: io::Seek,
        F: FnMut(&OsmObj) -> bool,
        T: StoreObjs,
    {
        let mut finished = false;
        let mut deps = BTreeSet::new();
        let mut first_pass = true;
        while !finished {
            self.rewind()?;
            finished = true;
            for obj in self.par_iter() {
                let obj = obj?;
                if (!first_pass || !pred(&obj)) && !deps.contains(&obj.id()) {
                    continue;
                }
                finished = match obj {
                    OsmObj::Relation(ref rel) => rel
                        .refs
                        .iter()
                        .filter(|r| !objects.contains_key(&r.member))
                        .fold(finished, |accu, r| !deps.insert(r.member) && accu),
                    OsmObj::Way(ref way) => way
                        .nodes
                        .iter()
                        .filter(|n| !objects.contains_key(&(**n).into()))
                        .fold(finished, |accu, n| !deps.insert((*n).into()) && accu),
                    OsmObj::Node(_) => finished,
                };
                deps.remove(&obj.id());
                objects.insert(obj.id(), obj);
            }
            first_pass = false;
        }
        Ok(())
    }

    /// This function give you the ability to find all the objects
    /// validating a predicate and all there dependencies.  The file
    /// will be decoded in parallel.
    ///
    /// # Example
    ///
    /// If you want to extract all the administrative boundaries
    /// and all there dependencies you can do something like that:
    ///
    /// ```
    /// fn is_admin(obj: &osmpbfreader::OsmObj) -> bool {
    ///     // get relations with tags[boundary] == administrative
    ///     obj.is_relation() && obj.tags().contains("boundary", "administrative")
    /// }
    ///
    /// let mut pbf = osmpbfreader::OsmPbfReader::new(std::io::Cursor::new([]));
    /// let objs = pbf.get_objs_and_deps(is_admin).unwrap();
    /// for (id, obj) in &objs {
    ///     println!("{:?}: {:?}", id, obj);
    /// }
    /// ```
    pub fn get_objs_and_deps<F>(&mut self, pred: F) -> Result<BTreeMap<OsmId, OsmObj>>
    where
        R: io::Seek,
        F: FnMut(&OsmObj) -> bool,
    {
        let mut objects = BTreeMap::new();
        match self.get_objs_and_deps_store(pred, &mut objects) {
            Ok(_) => Ok(objects),
            Err(e) => Err(e),
        }
    }
    /// Extract the Read object.
    ///
    /// Consumes the object.
    pub fn into_inner(self) -> R {
        self.r
    }
    /// Returns an iterator on the blobs of the pbf file.
    pub fn blobs(&mut self) -> Blobs<R> {
        Blobs { opr: self }
    }
    /// Returns an iterator on the blocks of the pbf file.
    pub fn primitive_blocks(&mut self) -> PrimitiveBlocks<R> {
        fn and_then_primitive_block(blob_res: Result<Blob>) -> Result<PrimitiveBlock> {
            blob_res.and_then(|b| primitive_block_from_blob(&b))
        }
        PrimitiveBlocks(self.blobs().map(and_then_primitive_block))
    }

    fn push(&mut self, sz: u64) -> Result<()> {
        self.buf.clear();
        self.r.by_ref().take(sz).read_to_end(&mut self.buf)?;
        assert_eq!(sz, self.buf.len() as u64);
        Ok(())
    }
    fn try_blob(&mut self, sz: u64) -> Result<Option<Blob>> {
        self.push(sz)?;
        let header: BlobHeader = protobuf::parse_from_bytes(&self.buf)?;
        let sz = header.get_datasize() as u64;
        self.push(sz)?;
        let blob: Blob = protobuf::parse_from_bytes(&self.buf)?;
        if header.get_field_type() == "OSMData" {
            Ok(Some(blob))
        } else if header.get_field_type() == "OSMHeader" {
            Ok(None)
        } else {
            println!("Unknown type: {}", header.get_field_type());
            Ok(None)
        }
    }
    fn next_blob(&mut self) -> Option<Result<Blob>> {
        use byteorder::{BigEndian, ReadBytesExt};
        use std::io::ErrorKind;
        if self.finished {
            return None;
        }
        let sz = match self.r.read_u32::<BigEndian>() {
            Ok(sz) if sz > 64 * 1024 => return Some(Err(Error::InvalidData)),
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

/// Iterator on the blobs of a file.
pub struct Blobs<'a, R: 'a> {
    opr: &'a mut OsmPbfReader<R>,
}
impl<'a, R: io::Read> Iterator for Blobs<'a, R> {
    type Item = Result<Blob>;
    fn next(&mut self) -> Option<Self::Item> {
        self.opr.next_blob()
    }
}

pub_iterator_type! {
    #[doc="Iterator on the blocks of a file."]
    PrimitiveBlocks['a, R] = iter::Map<Blobs<'a, R>, fn(Result<Blob>) -> Result<PrimitiveBlock>>
    where R: Read + 'a
}

/// Returns an iterator on the blocks of a blob.
pub fn primitive_block_from_blob(blob: &Blob) -> Result<PrimitiveBlock> {
    if blob.has_raw() {
        protobuf::parse_from_bytes(blob.get_raw()).map_err(From::from)
    } else if blob.has_zlib_data() {
        use flate2::read::ZlibDecoder;
        let r = io::Cursor::new(blob.get_zlib_data());
        let mut zr = ZlibDecoder::new(r);
        protobuf::parse_from_reader(&mut zr).map_err(From::from)
    } else {
        Err(Error::UnsupportedData)
    }
}

pub_iterator_type! {
    #[doc="Iterator on the `OsmObj` of the pbf file."]
    Iter['a, R] = iter::FlatMap<Blobs<'a, R>, blobs::OsmObjs, fn(Result<Blob>) -> blobs::OsmObjs>
    where R: io::Read + 'a
}

pub_iterator_type! {
    #[doc="Parallel iterator on the `OsmObj` of the pbf file."]
    ParIter['a, R] = par_map::FlatMap<Blobs<'a, R>,
                                      blobs::OsmObjs,
                                      fn(Result<Blob>) -> blobs::OsmObjs>
    where R: io::Read + 'a
}
