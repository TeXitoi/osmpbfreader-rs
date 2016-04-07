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
//! let mut pbf = osmpbfreader::OsmPbfReader::new(r);
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
//! let mut pbf = osmpbfreader::OsmPbfReader::new(r);
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
//! let mut pbf = osmpbfreader::OsmPbfReader::new(r);
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
pub use error::Error;
pub use error::Result;
pub use reader::{OsmPbfReader, primitive_block_from_blob};

/// Generated from protobuf.
#[allow(non_snake_case, missing_docs)] pub mod fileformat;

/// Generated from protobuf.
#[allow(missing_docs)] pub mod osmformat;

pub mod error;
pub mod objects;
pub mod groups;
pub mod blocks;
pub mod borrowed_iter;
pub mod reader;
