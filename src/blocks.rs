// Copyright (c) 2014-2015 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

//! Iterators of OpenStreetMap objects from a block.

use groups;
use objects::{Node, OsmObj, Relation, Way};
use osmformat::PrimitiveBlock;

pub_iterator_type! {
    #[doc="Iterator on the `OsmObj` of a `PrimitiveBlock`."]
    OsmObjs['a] = Box<Iterator<Item = OsmObj> + Send + 'a>
}

pub fn iter(block: &PrimitiveBlock) -> OsmObjs {
    let f = move |g| groups::iter(g, block);
    OsmObjs(Box::new(block.get_primitivegroup().iter().flat_map(f)))
}

pub_iterator_type! {
    #[doc="Iterator on the `Node` of a `PrimitiveBlock`."]
    Nodes['a] = Box<Iterator<Item = Node> + Send + 'a>
}

pub fn nodes(block: &PrimitiveBlock) -> Nodes {
    let f = move |g| groups::nodes(g, block);
    Nodes(Box::new(block.get_primitivegroup().iter().flat_map(f)))
}

pub_iterator_type! {
    #[doc="Iterator on the `Way` of a `PrimitiveBlock`."]
    Ways['a] = Box<Iterator<Item = Way> + 'a>
}

pub fn ways(block: &PrimitiveBlock) -> Ways {
    let f = move |g| groups::ways(g, block);
    Ways(Box::new(block.get_primitivegroup().iter().flat_map(f)))
}

pub_iterator_type! {
    #[doc="Iterator on the `Relation` of a `PrimitiveBlock`."]
    Relations['a] = Box<Iterator<Item = Relation> + Send + 'a>
}

pub fn relations(block: &PrimitiveBlock) -> Relations {
    let f = move |g| groups::relations(g, block);
    Relations(Box::new(block.get_primitivegroup().iter().flat_map(f)))
}
