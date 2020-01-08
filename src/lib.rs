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
//! # Getting objects and their dependencies
//!
//! Most of the time, you'll want a subset of the OSM objects and its
//! dependencies (i.e. the nodes inside a way, and not only the ids of
//! the nodes of this way).  For that, an easy to use function is
//! availlable.
//!
//! ```rust
//! let mut pbf = osmpbfreader::OsmPbfReader::new(std::io::Cursor::new([]));
//! let objs = pbf.get_objs_and_deps(|obj| {
//!     obj.is_way() && obj.tags().contains_key("highway")
//! })
//! .unwrap();
//! for (id, obj) in &objs {
//!     println!("{:?}: {:?}", id, obj);
//! }
//! ```
//!
//! # Reading
//!
//! The easiest way to read a PBF file is to directly iterate on the
//! `OsmObj`.
//!
//! ```rust
//! use std::process::exit;
//! let mut pbf = osmpbfreader::OsmPbfReader::new(std::io::empty());
//! for obj in pbf.iter() {
//!     // error handling:
//!     let obj = obj.unwrap_or_else(|e| {println!("{:?}", e); exit(1)});
//!
//!     println!("{:?}", obj);
//! }
//! ```
//!
//! There is also a parallel version of this iterator. The file is
//! decoded in parallel.
//!
//! ```rust
//! use std::process::exit;
//! let mut pbf = osmpbfreader::OsmPbfReader::new(std::io::empty());
//! for obj in pbf.par_iter() {
//!     // error handling:
//!     let obj = obj.unwrap_or_else(|e| {println!("{:?}", e); exit(1)});
//!
//!     println!("{:?}", obj);
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
//! let mut pbf = osmpbfreader::OsmPbfReader::new(std::io::empty());
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

#![deny(missing_docs)]

extern crate byteorder;
extern crate flat_map;
extern crate flate2;
extern crate protobuf;
#[macro_use]
extern crate rental;
extern crate par_map;
#[macro_use]
extern crate pub_iterator_type;
extern crate serde;
#[macro_use]
extern crate serde_derive;

pub use error::Error;
pub use error::Result;
pub use objects::*;
pub use reader::{primitive_block_from_blob, OsmPbfReader, StoreObjs};

pub mod blobs;
pub mod objects;
pub mod reader;

#[allow(missing_docs)]
pub mod blocks;
#[allow(missing_docs)]
pub mod error;
#[allow(missing_docs)]
pub mod groups;

mod pbf {
    include!(concat!(env!("OUT_DIR"), "/mod.rs"));
}

pub use pbf::{fileformat, osmformat};

/*
/// Generated from protobuf.
#[allow(non_snake_case, missing_docs)]
pub mod fileformat;

/// Generated from protobuf.
#[allow(missing_docs)]
pub mod osmformat;
*/
