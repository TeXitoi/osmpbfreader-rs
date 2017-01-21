// Copyright (c) 2014-2017 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

use std::io;
use borrowed_iter::BorrowedIter;
use reader::PrimitiveBlocks;
use osmformat::PrimitiveBlock;
use blocks;
use ::Result;
use objects::OsmObj;

pub struct Iter<'a, R: io::Read + 'a> {
    blocks: PrimitiveBlocks<'a, R>,
    objs: BorrowedIter<PrimitiveBlock, blocks::OsmObjs<'static>>,
}

impl<'a, R: io::Read + 'a> Iter<'a, R> {
    pub fn new(blocks: PrimitiveBlocks<'a, R>)-> Self {
        Iter {
            blocks: blocks,
            objs: BorrowedIter::new(PrimitiveBlock::new(), blocks::iter),
        }
    }
}

impl <'a, R: io::Read + 'a> Iterator for Iter<'a, R> {
    type Item = Result<OsmObj>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(obj) = self.objs.next() {
                return Some(Ok(obj));
            }
            match self.blocks.next() {
                None => return None,
                Some(Err(e)) => return Some(Err(e)),
                Some(Ok(block)) => self.objs = BorrowedIter::new(block, blocks::iter),
            }
        }
    }
}
