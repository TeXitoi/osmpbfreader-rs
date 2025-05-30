// Copyright (c) 2014-2017 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

//! Iterator and utilities for `fileformat::Blob`.

use std::iter;

#[cfg(feature = "full-metadata")]
use crate::blocks::OsmObjInfos as OsmBlockObjInfos;
use crate::blocks::{
    self, Nodes as BlockNodes, OsmObjs as OsmBlockObjs, Relations as BlockRelations,
    Ways as BlockWays,
};
use crate::fileformat::Blob;
use crate::objects::OsmObj;
#[cfg(feature = "full-metadata")]
use crate::objects::OsmObjInfo;
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
#[cfg(feature = "full-metadata")]
wrap!(OsmBlobObjInfos, OsmBlockObjInfos => OsmObjInfo);
wrap!(OsmBlobRelations, BlockRelations => super::Relation);
wrap!(OsmBlobWays, BlockWays => super::Way);
wrap!(OsmBlobNodes, BlockNodes => super::Node);

/// An iterator on `Result<OsmObj>`.
pub struct OsmObjs<T: Iterator>(OsmObjsImpl<T>);

#[allow(clippy::type_complexity)]
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
        Ok(block) => OsmObjs(OsmObjsImpl::OkIter(
            OsmBlobObjs::new(block, blocks::iter).map(Ok),
        )),
        Err(e) => OsmObjs(OsmObjsImpl::ErrIter(iter::once(Err(e)))),
    }
}

/// An iterator on `Result<OsmObjInfo>`.
#[cfg(feature = "full-metadata")]
pub struct OsmObjInfos<T: Iterator>(OsmObjInfosImpl<T>);

#[allow(clippy::type_complexity)]
#[cfg(feature = "full-metadata")]
enum OsmObjInfosImpl<T: Iterator> {
    OkIter(iter::Map<T, fn(<T as Iterator>::Item) -> crate::Result<<T as Iterator>::Item>>),
    ErrIter(iter::Once<crate::Result<<T as Iterator>::Item>>),
}

#[cfg(feature = "full-metadata")]
impl<T: Iterator> Iterator for OsmObjInfos<T> {
    type Item = crate::Result<<T as Iterator>::Item>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            OsmObjInfosImpl::OkIter(ref mut iter) => iter.next(),
            OsmObjInfosImpl::ErrIter(ref mut iter) => iter.next(),
        }
    }
}

/// Transforms a `Result<blob>` into a `Iterator<Item = Result<OsmObjInfo>>`.
#[cfg(feature = "full-metadata")]
pub fn result_blob_into_iter_with_metadata(
    result: crate::Result<Blob>,
) -> OsmObjInfos<OsmBlobObjInfos> {
    match result.and_then(|b| crate::reader::primitive_block_from_blob(&b)) {
        Ok(block) => OsmObjInfos(OsmObjInfosImpl::OkIter(
            OsmBlobObjInfos::new(block, blocks::iter_with_metadata).map(Ok),
        )),
        Err(e) => OsmObjInfos(OsmObjInfosImpl::ErrIter(iter::once(Err(e)))),
    }
}

/// Transforms a `Result<blob>` into a `Iterator<Item = Result<Node>>`.
pub fn result_blob_into_node_iter(result: crate::Result<Blob>) -> OsmObjs<OsmBlobNodes> {
    match result.and_then(|b| crate::reader::primitive_block_from_blob(&b)) {
        Ok(block) => OsmObjs(OsmObjsImpl::OkIter(
            OsmBlobNodes::new(block, blocks::nodes).map(Ok),
        )),
        Err(e) => OsmObjs(OsmObjsImpl::ErrIter(iter::once(Err(e)))),
    }
}

/// Transforms a `Result<blob>` into a `Iterator<Item = Result<Way>>`.
pub fn result_blob_into_way_iter(result: crate::Result<Blob>) -> OsmObjs<OsmBlobWays> {
    match result.and_then(|b| crate::reader::primitive_block_from_blob(&b)) {
        Ok(block) => OsmObjs(OsmObjsImpl::OkIter(
            OsmBlobWays::new(block, blocks::ways).map(Ok),
        )),
        Err(e) => OsmObjs(OsmObjsImpl::ErrIter(iter::once(Err(e)))),
    }
}

/// Transforms a `Result<blob>` into a `Iterator<Item = Result<Relation>>`.
pub fn result_blob_into_relation_iter(result: crate::Result<Blob>) -> OsmObjs<OsmBlobRelations> {
    match result.and_then(|b| crate::reader::primitive_block_from_blob(&b)) {
        Ok(block) => OsmObjs(OsmObjsImpl::OkIter(
            OsmBlobRelations::new(block, blocks::relations).map(Ok),
        )),
        Err(e) => OsmObjs(OsmObjsImpl::ErrIter(iter::once(Err(e)))),
    }
}
