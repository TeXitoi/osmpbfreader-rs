// Copyright (c) 2014 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

use osmformat::PrimitiveBlock;
use groups;
use objects::{OsmObj, Node, Way, Relation};

pub type OsmObjs<'a> = Box<Iterator<Item = OsmObj> + 'a>;

pub fn iter<'a>(block: &'a PrimitiveBlock) -> OsmObjs<'a> {
    let f = move |g| groups::iter(g, block);
    Box::new(block.get_primitivegroup().iter().flat_map(f))
}

pub type Nodes<'a> = Box<Iterator<Item = Node> + 'a>;

pub fn nodes<'a>(block: &'a PrimitiveBlock) -> Nodes<'a> {
    let f = move |g| groups::nodes(g, block);
    Box::new(block.get_primitivegroup().iter().flat_map(f))
}

pub type Ways<'a> = Box<Iterator<Item = Way> + 'a>;

pub fn ways<'a>(block: &'a PrimitiveBlock) -> Ways<'a> {
    let f = move |g| groups::ways(g, block);
    Box::new(block.get_primitivegroup().iter().flat_map(f))
}

pub type Relations<'a> = Box<Iterator<Item = Relation> + 'a>;

pub fn relations<'a>(block: &'a PrimitiveBlock) -> Relations<'a> {
    let f = move |g| groups::relations(g, block);
    Box::new(block.get_primitivegroup().iter().flat_map(f))
}
