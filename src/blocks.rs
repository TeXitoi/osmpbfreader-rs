// Copyright (c) 2014-2015 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

//! Iterators of OpenStreetMap objects from a block.

use crate::groups;
use crate::objects::{Node, OsmObj, Relation, Way};
use crate::osmformat::PrimitiveBlock;
use pub_iterator_type::pub_iterator_type;

pub_iterator_type! {
    #[doc="Iterator on the `OsmObj` of a `PrimitiveBlock`."]
    OsmObjs['a] = Box<dyn Iterator<Item = OsmObj> + 'a + Send>
}

pub fn iter(block: &PrimitiveBlock) -> OsmObjs {
    let f = move |g| groups::iter(g, block);
    OsmObjs(Box::new(block.primitivegroup.iter().flat_map(f)))
}

pub_iterator_type! {
    #[doc="Iterator on the `Node` of a `PrimitiveBlock`."]
    Nodes['a] = Box<dyn Iterator<Item = Node> + 'a + Send>
}

pub fn nodes(block: &PrimitiveBlock) -> Nodes {
    let f = move |g| groups::nodes(g, block);
    Nodes(Box::new(block.primitivegroup.iter().flat_map(f)))
}

pub_iterator_type! {
    #[doc="Iterator on the `Way` of a `PrimitiveBlock`."]
    Ways['a] = Box<dyn Iterator<Item = Way> + 'a + Send>
}

pub fn ways(block: &PrimitiveBlock) -> Ways {
    let f = move |g| groups::ways(g, block);
    Ways(Box::new(block.primitivegroup.iter().flat_map(f)))
}

pub_iterator_type! {
    #[doc="Iterator on the `Relation` of a `PrimitiveBlock`."]
    Relations['a] = Box<dyn Iterator<Item = Relation> + 'a + Send>
}

pub fn relations(block: &PrimitiveBlock) -> Relations {
    let f = move |g| groups::relations(g, block);
    Relations(Box::new(block.primitivegroup.iter().flat_map(f)))
}
