// Copyright (c) 2014-2017 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

//! Iterator and utilities for `fileformat::Blob`.

use std::iter;
use blocks;
use Result;
use objects::OsmObj;

rental!{
    mod rent {
        #[rental]
        pub struct OsmObjs {
            #[subrental(arity = 2)]
            block: Box<::rent::PrimitiveBlock>,
            objs: ::blocks::OsmObjs<'block_1>,
        }
    }
}

impl<'a> Iterator for rent::OsmObjs {
    type Item = OsmObj;
    fn next(&mut self) -> Option<Self::Item> {
        self.rent_mut(|objs| objs.next())
    }
}

/// An iterator on `Result<OsmObj>`.
pub struct OsmObjs(OsmObjsImpl);
enum OsmObjsImpl {
    OkIter(iter::Map<rent::OsmObjs, fn(OsmObj) -> Result<OsmObj>>),
    ErrIter(iter::Once<Result<OsmObj>>),
}
impl Iterator for OsmObjs {
    type Item = Result<OsmObj>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            OsmObjsImpl::OkIter(ref mut iter) => iter.next(),
            OsmObjsImpl::ErrIter(ref mut iter) => iter.next(),
        }
    }
}

/// Transforms a `Result<blob>` into a `Iterator<Item = Result<OsmObj>>`.
pub fn result_blob_into_iter(result: Result<::rent::Blob>) -> OsmObjs {
    match result.and_then(|b| ::reader::primitive_block_from_blob(&b)) {
        Ok(block) => OsmObjs(OsmObjsImpl::OkIter(new_rent_osm_objs(block).map(Ok))),
        Err(e) => OsmObjs(OsmObjsImpl::ErrIter(iter::once(Err(e)))),
    }
}

fn new_rent_osm_objs(block: ::rent::PrimitiveBlock) -> rent::OsmObjs {
    rent::OsmObjs::new(Box::new(block), |b| blocks::iter(b.block))
}
