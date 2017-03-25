// Copyright (c) 2014-2017 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

use std::io;
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

fn new_rent_osm_objs(block: PrimitiveBlock) -> rent::OsmObjs<'static> {
    rent::OsmObjs::new(Box::new(block), |b| blocks::iter(b))
}

pub struct Iter<'a, R: io::Read + 'a> {
    blocks: PrimitiveBlocks<'a, R>,
    objs: rent::OsmObjs<'static>,
}

impl<'a, R: io::Read + 'a> Iter<'a, R> {
    pub fn new(blocks: PrimitiveBlocks<'a, R>) -> Self {
        Iter {
            blocks: blocks,
            objs: new_rent_osm_objs(PrimitiveBlock::new()),
        }
    }
}

impl<'a, R: io::Read + 'a> Iterator for Iter<'a, R> {
    type Item = Result<OsmObj>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(obj) = self.objs.rent_mut(|iter| iter.next()) {
                return Some(Ok(obj));
            }
            match self.blocks.next() {
                None => return None,
                Some(Err(e)) => return Some(Err(e)),
                Some(Ok(block)) => self.objs = new_rent_osm_objs(block),
            }
        }
    }
}
