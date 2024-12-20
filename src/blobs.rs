// Copyright (c) 2014-2017 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

//! Iterator and utilities for `fileformat::Blob`.

use std::iter;

use crate::blocks;
use crate::blocks::OsmObjs as OsmBlockObjs;
use crate::fileformat::Blob;
use crate::objects::OsmObj;
use crate::osmformat::PrimitiveBlock;

self_cell::self_cell!(
    struct OsmBlobObjs {
        owner: PrimitiveBlock,

        #[covariant]
        dependent: OsmBlockObjs,
    }
);

impl Iterator for OsmBlobObjs {
    type Item = OsmObj;

    fn next(&mut self) -> Option<Self::Item> {
        self.with_dependent_mut(|_, objs| objs.next())
    }
}

/// An iterator on `Result<OsmObj>`.
pub struct OsmObjs(OsmObjsImpl);

enum OsmObjsImpl {
    OkIter(iter::Map<OsmBlobObjs, fn(OsmObj) -> crate::Result<OsmObj>>),
    ErrIter(iter::Once<crate::Result<OsmObj>>),
}

impl Iterator for OsmObjs {
    type Item = crate::Result<OsmObj>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            OsmObjsImpl::OkIter(ref mut iter) => iter.next(),
            OsmObjsImpl::ErrIter(ref mut iter) => iter.next(),
        }
    }
}

/// Transforms a `Result<blob>` into a `Iterator<Item = Result<OsmObj>>`.
pub fn result_blob_into_iter(result: crate::Result<Blob>) -> OsmObjs {
    match result.and_then(|b| crate::reader::primitive_block_from_blob(&b)) {
        Ok(block) => OsmObjs(OsmObjsImpl::OkIter(new_rent_osm_objs(block).map(Ok))),
        Err(e) => OsmObjs(OsmObjsImpl::ErrIter(iter::once(Err(e)))),
    }
}

fn new_rent_osm_objs(block: PrimitiveBlock) -> OsmBlobObjs {
    OsmBlobObjs::new(block, blocks::iter)
}
