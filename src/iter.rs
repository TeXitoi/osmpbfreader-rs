// Copyright (c) 2014-2017 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

//! Iterator for `OsmPbfReader`.

use std::io;
use std::iter::{self, FlatMap};
use reader::PrimitiveBlocks;
use osmformat::PrimitiveBlock;
use blocks;
use Result;
use objects::OsmObj;

rental!{
    mod rent {
        use osmformat::PrimitiveBlock;
        use blocks;
        pub rental mut OsmObjs<'rental>(Box<PrimitiveBlock>, blocks::OsmObjs<'rental>);
    }
}

impl<'a> Iterator for rent::OsmObjs<'a> {
    type Item = OsmObj;
    fn next(&mut self) -> Option<Self::Item> {
        self.rent_mut(|iter| iter.next())
    }
}

fn result_block_into_iter(result: Result<PrimitiveBlock>) -> Box<Iterator<Item = Result<OsmObj>>> {
    match result {
        Ok(block) => Box::new(new_rent_osm_objs(block).map(Ok)),
        Err(e) => Box::new(iter::once(Err(e))),
    }
}

fn new_rent_osm_objs(block: PrimitiveBlock) -> rent::OsmObjs<'static> {
    rent::OsmObjs::new(Box::new(block), |b| blocks::iter(b))
}

pub_iterator_type! {
    #[doc="Iterator on the `OsmObj` of the pbf file."]
    Iter['a, R] = FlatMap<
        PrimitiveBlocks<'a, R>,
        Box<iter::Iterator<Item=Result<OsmObj>>>,
        fn(Result<PrimitiveBlock>) -> Box<iter::Iterator<Item=Result<OsmObj>>>>
    where R: io::Read + 'a
}

impl<'a, R: io::Read + 'a> Iter<'a, R> {
    /// Returns an iterator on the `OsmObj` of the pbf file.
    pub fn new(blocks: PrimitiveBlocks<'a, R>) -> Self {
        Iter(blocks.flat_map(result_block_into_iter))
    }
}
