// Copyright (c) 2014-2017 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

//! Iterator for `OsmPbfReader`.

use std::io;
use std::iter::{self, FlatMap};
use reader::Blobs;
use osmformat::PrimitiveBlock;
use fileformat::Blob;
use blocks;
use Result;
use objects::OsmObj;

rental!{
    mod rent {
        use osmformat::PrimitiveBlock;
        use blocks;
        #[rental]
        pub struct OsmObjs {
            block: Box<PrimitiveBlock>,
            objs: blocks::OsmObjs<'block>,
        }
    }
}

impl<'a> Iterator for rent::OsmObjs {
    type Item = OsmObj;
    fn next(&mut self) -> Option<Self::Item> {
        self.rent_mut(|objs| objs.next())
    }
}

/// Transforms a `Result<blob>` into a `Iterator<Item = Result<OsmObj>>`.
pub fn result_blob_into_iter(result: Result<Blob>) -> Box<Iterator<Item = Result<OsmObj>>> {
    match result.and_then(|b| ::reader::primitive_block_from_blob(&b)) {
        Ok(block) => Box::new(new_rent_osm_objs(block).map(Ok)),
        Err(e) => Box::new(iter::once(Err(e))),
    }
}

fn new_rent_osm_objs(block: PrimitiveBlock) -> rent::OsmObjs {
    rent::OsmObjs::new(Box::new(block), |b| blocks::iter(b))
}

pub_iterator_type! {
    #[doc="Iterator on the `OsmObj` of the pbf file."]
    Iter['a, R] = FlatMap<
        Blobs<'a, R>,
        Box<iter::Iterator<Item=Result<OsmObj>>>,
        fn(Result<Blob>) -> Box<iter::Iterator<Item=Result<OsmObj>>>>
    where R: io::Read + 'a
}

impl<'a, R: io::Read + 'a> Iter<'a, R> {
    /// Returns an iterator on the `OsmObj` of the pbf file.
    pub fn new(blobs: Blobs<'a, R>) -> Self {
        Iter(blobs.flat_map(result_blob_into_iter))
    }
}
