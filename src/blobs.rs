// Copyright (c) 2014-2017 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

//! Iterator and utilities for `fileformat::Blob`.

use std::iter;

use crate::blocks::{self, OsmObjs as OsmBlockObjs, Relations as BlockRelations, Ways as BlockWays, Nodes as BlockNodes};
use crate::fileformat::Blob;
use crate::objects::OsmObj;
use crate::osmformat::PrimitiveBlock;

macro_rules! wrap {
    ($name: ident, $wrap_type: ident => $inner_type: path) => {
self_cell::self_cell!(
    #[allow(missing_docs)]
    pub struct $name {
        owner: PrimitiveBlock,

        #[covariant]
        dependent: $wrap_type,
    }
);

impl Iterator for $name {
    type Item = $inner_type;

    fn next(&mut self) -> Option<Self::Item> {
        self.with_dependent_mut(|_, objs| objs.next())
    }
}

    };
}

wrap!(OsmBlobObjs, OsmBlockObjs => OsmObj);
wrap!(OsmBlobRelations, BlockRelations => super::Relation);
wrap!(OsmBlobWays, BlockWays => super::Way);
wrap!(OsmBlobNodes, BlockNodes => super::Node);

/// An iterator on `Result<OsmObj>`.
pub struct OsmObjs<T: Iterator>(OsmObjsImpl<T>);

enum OsmObjsImpl<T: Iterator> {
    OkIter(iter::Map<T, fn(<T as Iterator>::Item) -> crate::Result<<T as Iterator>::Item>>),
    ErrIter(iter::Once<crate::Result<<T as Iterator>::Item>>),
}

impl<T: Iterator> Iterator for OsmObjs<T> {
    type Item = crate::Result<<T as Iterator>::Item>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            OsmObjsImpl::OkIter(ref mut iter) => iter.next(),
            OsmObjsImpl::ErrIter(ref mut iter) => iter.next(),
        }
    }
}

/// Transforms a `Result<blob>` into a `Iterator<Item = Result<OsmObj>>`.
pub fn result_blob_into_iter(result: crate::Result<Blob>) -> OsmObjs<OsmBlobObjs> {
    match result.and_then(|b| crate::reader::primitive_block_from_blob(&b)) {
        Ok(block) => OsmObjs(OsmObjsImpl::OkIter(OsmBlobObjs::new(block, blocks::iter).map(Ok))),
        Err(e) => OsmObjs(OsmObjsImpl::ErrIter(iter::once(Err(e)))),
    }
}

/// Transforms a `Result<blob>` into a `Iterator<Item = Result<Node>>`.
pub fn result_blob_into_node_iter(result: crate::Result<Blob>) -> OsmObjs<OsmBlobNodes> {
    match result.and_then(|b| crate::reader::primitive_block_from_blob(&b)) {
        Ok(block) => OsmObjs(OsmObjsImpl::OkIter(OsmBlobNodes::new(block, blocks::nodes).map(Ok))),
        Err(e) => OsmObjs(OsmObjsImpl::ErrIter(iter::once(Err(e)))),
    }
}

/// Transforms a `Result<blob>` into a `Iterator<Item = Result<Way>>`.
pub fn result_blob_into_way_iter(result: crate::Result<Blob>) -> OsmObjs<OsmBlobWays> {
    match result.and_then(|b| crate::reader::primitive_block_from_blob(&b)) {
        Ok(block) => OsmObjs(OsmObjsImpl::OkIter(OsmBlobWays::new(block, blocks::ways).map(Ok))),
        Err(e) => OsmObjs(OsmObjsImpl::ErrIter(iter::once(Err(e)))),
    }
}

/// Transforms a `Result<blob>` into a `Iterator<Item = Result<Relation>>`.
pub fn result_blob_into_relation_iter(result: crate::Result<Blob>) -> OsmObjs<OsmBlobRelations> {
    match result.and_then(|b| crate::reader::primitive_block_from_blob(&b)) {
        Ok(block) => OsmObjs(OsmObjsImpl::OkIter(OsmBlobRelations::new(block, blocks::relations).map(Ok))),
        Err(e) => OsmObjs(OsmObjsImpl::ErrIter(iter::once(Err(e)))),
    }
}
