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
//! [dependencies]
//! osmpbfreader = "0.6"
//! ```
//!
//! and this to your crate root:
//!
//! ```rust
//! extern crate osmpbfreader;
//! ```
//!
//! # Getting objects and their dependencies
//!
//! Most of the time, you'll want a subset of the OSM objects and its
//! dependencies (i.e. the nodes inside a way, and not only the ids of
//! the nodes of this way).  For that, an easy to use function is
//! availlable.
//!
//!```
//! let path = std::path::Path::new("/dev/null");
//! let r = std::fs::File::open(&path).unwrap();
//! let mut pbf = osmpbfreader::OsmPbfReader::new(r);
//! let objs = osmpbfreader::get_objs_and_deps(&mut pbf, |obj| {
//!         obj.way().map_or(false, |w| w.tags.contains_key("highway"))
//!     })
//!     .unwrap();
//! for (id, obj) in &objs {
//!     println!("{:?}: {:?}", id, obj);
//! }
//!```
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

// #![deny(missing_docs)]

extern crate protobuf;
extern crate flate2;
extern crate byteorder;
extern crate flat_map;

pub use objects::{OsmObj, Node, Way, Relation, Ref, OsmId, Tags};
pub use error::Error;
pub use error::Result;
pub use reader::{OsmPbfReader, primitive_block_from_blob};

/// Generated from protobuf.
#[allow(non_snake_case, missing_docs)]pub mod fileformat;

/// Generated from protobuf.
#[allow(missing_docs)]pub mod osmformat;

pub mod error;
pub mod objects;
pub mod groups;
pub mod blocks;
pub mod borrowed_iter;
pub mod reader;

use std::collections::{BTreeSet, BTreeMap};
use std::io::{Seek, Read};


/// This function give you the ability to find all the objects validating
/// a predicate and all there dependencies.
///
/// # Examples
/// If you want to extract all the administrative boundaries
/// and all there dependencies you can do something like that:
///
/// ```
/// fn is_admin(obj: &osmpbfreader::OsmObj) -> bool{
///     match *obj {
///        osmpbfreader::OsmObj::Relation(ref rel) => {
///            rel.tags.get("boundary").map_or(false, |v| v == "administrative")
///         }
///        _ => false,
///     }
/// }
///
/// let path = std::path::Path::new("/dev/null");
/// let r = std::fs::File::open(&path).unwrap();
/// let mut pbf = osmpbfreader::OsmPbfReader::new(r);
/// for obj in osmpbfreader::get_objs_and_deps(&mut pbf, is_admin) {
///     println!("{:?}", obj);
/// }
/// ```
pub fn get_objs_and_deps<R, F>(reader: &mut OsmPbfReader<R>,
                               mut pred: F)
                               -> Result<BTreeMap<OsmId, OsmObj>>
    where R: Read + Seek,
          F: FnMut(&OsmObj) -> bool
{
    let mut finished = false;
    let mut deps = BTreeSet::new();
    let mut objects = BTreeMap::new();
    while !finished {
        finished = true;
        for block in reader.primitive_blocks() {
            let block = try!(block);
            for obj in blocks::iter(&block) {
                if !deps.contains(&obj.id()) && !pred(&obj) {
                    continue;
                }
                finished = match obj {
                    OsmObj::Relation(ref rel) => {
                        rel.refs.iter().fold(finished, |accu, r| !deps.insert(r.member) && accu)
                    }
                    OsmObj::Way(ref way) => {
                        way.nodes
                           .iter()
                           .fold(finished, |accu, n| !deps.insert(OsmId::Node(*n)) && accu)
                    }
                    OsmObj::Node(_) => finished,
                };
                objects.insert(obj.id(), obj);
            }
        }
        try!(reader.rewind());
    }
    Ok(objects)
}
