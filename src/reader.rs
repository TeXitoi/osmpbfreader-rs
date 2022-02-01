// Copyright (c) 2014-2017 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

//! Tools for reading a pbf file.

use crate::blobs::{self, result_blob_into_iter};
use crate::error::{Error, Result};
use crate::fileformat::{Blob, BlobHeader};
use crate::objects::{OsmId, OsmObj};
use crate::osmformat::PrimitiveBlock;
use par_map::{self, ParMap};
use protobuf::Message;
use pub_iterator_type::pub_iterator_type;
use std::collections::btree_map::BTreeMap;
use std::collections::BTreeSet;
use std::convert::From;
use std::io::{self, Read};
use std::iter;
use std::rc::Rc;

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

/// Pack an objects with its dependencies (ie. nodes if it is a way and
/// members if it is a relation).
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ObjAndDeps {
    /// The object itself
    pub inner: OsmObj,
    /// The dependencies of the object
    pub deps: Vec<Rc<ObjAndDeps>>,
}

impl ObjAndDeps {
    /// Get id of inner object
    pub fn id(&self) -> OsmId {
        self.inner.id()
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
            r,
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
    pub fn par_iter(&mut self) -> ParIter<'_, R> {
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
    pub fn get_objs_and_deps_store<F, T>(&mut self, mut pred: F, objects: &mut T) -> Result<()>
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
    /// validating a predicate and all their dependencies. The file
    /// will be decoded in parallel.
    ///
    /// # Example
    ///
    /// If you want to extract all the administrative boundaries
    /// and all their dependencies you can do something like that:
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

    /// Similarly to `get_objs_and_deps`, this function will give you all objects validating a
    /// predicate packed with their dependencies.
    ///
    /// The main difference with `get_objs_and_deps` is that objects are outputted via the input
    /// callback function `handle_obj` as soon as the object and its dependencies have been fetched
    /// from the PBF file. This makes this function less memory intensive as objects can be
    /// processed on the fly and freed while parsing the file, for example nodes satisfying the
    /// predicate will be immediately outputted.
    ///
    /// The parameter `max_loaded_objects` can be used to tweak the max number of incomplete
    /// objects that can be loaded into memory at once, a bigger values means more memory usage but
    /// potentially less sequential reads of input PBF file.
    ///
    /// # Example
    ///
    /// ```
    /// fn is_admin(obj: &osmpbfreader::OsmObj) -> bool {
    ///     // get relations with tags[boundary] == administrative
    ///     obj.is_relation() && obj.tags().contains("boundary", "administrative")
    /// }
    ///
    /// fn handle_obj(obj: osmpbfreader::reader::ObjAndDeps) {
    ///     println!(
    ///         "{:?} -> {:?}",
    ///         obj.id(),
    ///         obj.deps.into_iter().map(|child| child.inner.id()).collect::<Vec<_>>(),
    ///     )
    /// }
    ///
    /// let mut pbf = osmpbfreader::OsmPbfReader::new(std::io::Cursor::new([]));
    /// pbf.get_objs_and_deps_on_the_fly(is_admin, handle_obj, 1_000_000);
    /// ```
    pub fn get_objs_and_deps_on_the_fly<F, H>(
        &mut self,
        mut pred: F,
        mut handle_obj: H,
        max_loaded_objects: usize,
    ) where
        R: io::Seek,
        F: FnMut(&OsmObj) -> bool,
        H: FnMut(ObjAndDeps),
    {
        /// Same as ObjAndDeps, but `deps_fetched` can be partially filled and unordered.
        struct PendingObj {
            obj: OsmObj,
            deps_fetched: Vec<Rc<ObjAndDeps>>,
        }

        impl PendingObj {
            fn id(&self) -> OsmId {
                self.obj.id()
            }

            /// Return an iterator over all id's of this object's children
            fn deps_expected(&self) -> impl Iterator<Item = OsmId> + '_ {
                let way_children = (self.obj.way())
                    .into_iter()
                    .flat_map(|way| way.nodes.iter().copied().map(Into::into));

                let rel_children = (self.obj.relation())
                    .into_iter()
                    .flat_map(|way| way.refs.iter().map(|r| r.member));

                way_children.chain(rel_children)
            }

            /// Check if all the children for this node have been extracted from OSM file
            fn is_complete(&self) -> bool {
                match &self.obj {
                    OsmObj::Node(_) => true,
                    OsmObj::Way(w) => w.nodes.len() == self.deps_fetched.len(),
                    OsmObj::Relation(r) => r.refs.len() == self.deps_fetched.len(),
                }
            }

            /// Reorder dependencies and convert to return type `ObjAndDeps`
            fn finish(self) -> ObjAndDeps {
                // Execute some kind of radix sort: first we order objects by id
                let mut as_map: BTreeMap<OsmId, Vec<Rc<ObjAndDeps>>> = BTreeMap::default();

                for obj in &self.deps_fetched {
                    as_map.entry(obj.id()).or_default().push(obj.clone());
                }

                // Then we fetch objects in order based on their ID
                let deps: Vec<_> = self
                    .deps_expected()
                    .filter_map(move |id| as_map.get_mut(&id)?.pop())
                    .collect();

                ObjAndDeps {
                    inner: self.obj,
                    deps,
                }
            }
        }

        // Store objects that have not finished being built yet
        let mut pending: BTreeMap<OsmId, PendingObj> = BTreeMap::default();

        // Store dependency of one object to another and its depth
        let mut deps_graph: BTreeMap<OsmId, Vec<OsmId>> = BTreeMap::default();

        // ID of the last explicitly imported object (excludes objects that are picked as a dependancy)
        let mut last_imported_object = None;

        // When this is true, we import addresses that validate the filter, this is set to false when
        // the graph may take too much RAM
        let mut import_first_layer = true;

        // Check if current iteration made progress
        let mut made_progress = true;

        while made_progress {
            made_progress = false;
            self.rewind().expect("could not rewind PBF reader");

            'read_pbf: for obj in self.par_iter() {
                let obj = obj.expect("could not read pbf");

                // The first layer only consists of filtered objects. Next layers include objects that
                // are required by dependency and not yet pending
                let feasible = (import_first_layer && pred(&obj))
                    || (deps_graph.contains_key(&obj.id()) && !pending.contains_key(&obj.id()));

                // Keep track of the last explicitly imported object while import_first_layer is true.
                // Then, it is set to true when the run reaches a section of the file that is not
                // imported yet.
                if import_first_layer {
                    last_imported_object = Some(obj.id());
                } else if last_imported_object == Some(obj.id()) {
                    import_first_layer = true;
                }

                if !feasible {
                    continue;
                }

                // Convert into internal object format
                let obj = PendingObj {
                    obj,
                    deps_fetched: Vec::new(),
                };

                made_progress = true;

                for child in obj.deps_expected() {
                    deps_graph.entry(child).or_default().push(obj.id());
                }

                // Start a graph search from current node, which propagate on completed objects
                let mut todo = vec![obj];

                while let Some(obj) = todo.pop() {
                    if obj.is_complete() {
                        let obj = obj.finish();

                        if let Some(parents_id) = deps_graph.remove(&obj.id()) {
                            let obj = Rc::new(obj);

                            // Insert the object in its parents
                            for parent_id in parents_id {
                                // Fetch parent from pending objects. Occasionally it is dependency of
                                // two objects and will already be in the `todo` heap.
                                let parent_obj = {
                                    if let Some(parent_obj) = pending.remove(&parent_id) {
                                        todo.push(parent_obj);
                                        todo.last_mut().unwrap()
                                    } else {
                                        todo.iter_mut().find(|x| x.id() == parent_id).unwrap()
                                    }
                                };

                                parent_obj.deps_fetched.push(obj.clone());
                            }
                        } else {
                            // If this object has no parents it means that it was selected by input
                            // filter and must be handled
                            handle_obj(obj);
                        }
                    } else {
                        pending.insert(obj.id(), obj);
                    }
                }

                // Reset run if too many items are in dependencies
                if import_first_layer && pending.len() >= max_loaded_objects {
                    break 'read_pbf;
                }
            }

            import_first_layer = false;
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
        let header: BlobHeader = Message::parse_from_bytes(&self.buf)?;
        let sz = header.get_datasize() as u64;
        self.push(sz)?;
        let blob: Blob = Message::parse_from_bytes(&self.buf)?;
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
        Message::parse_from_bytes(blob.get_raw()).map_err(From::from)
    } else if blob.has_zlib_data() {
        use flate2::read::ZlibDecoder;
        let r = io::Cursor::new(blob.get_zlib_data());
        let mut zr = ZlibDecoder::new(r);
        Message::parse_from_reader(&mut zr).map_err(From::from)
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
