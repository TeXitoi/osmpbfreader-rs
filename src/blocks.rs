// Copyright (c) 2014 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

use osmformat::PrimitiveBlock;
use osmformat::PrimitiveGroup;
use groups;
use std::iter::FlatMap;
use std::slice;
use objects::{OsmObj, Node, Way, Relation};

pub type OsmObjs<'a> = FlatMap<&'a PrimitiveGroup, OsmObj, slice::Items<'a, PrimitiveGroup>, groups::OsmObjs<'a>, FnOsmObjs<'a>>;

struct FnOsmObjs<'a> { block: &'a PrimitiveBlock }
impl<'a> Fn(&'a PrimitiveGroup) -> groups::OsmObjs<'a> for FnOsmObjs<'a> {
    extern "rust-call" fn call(&self, (group,): (&'a PrimitiveGroup,))
                               -> groups::OsmObjs<'a>
    {
        groups::iter(group, self.block)
    }
}

pub fn iter<'a>(block: &'a PrimitiveBlock) -> OsmObjs<'a> {
    block.get_primitivegroup().iter().flat_map(FnOsmObjs { block: block })
}

pub type Nodes<'a> = FlatMap<&'a PrimitiveGroup, Node, slice::Items<'a, PrimitiveGroup>, groups::Nodes<'a>, FnNodes<'a>>;

struct FnNodes<'a> { block: &'a PrimitiveBlock }
impl<'a> Fn(&'a PrimitiveGroup) -> groups::Nodes<'a> for FnNodes<'a> {
    extern "rust-call" fn call(&self, (group,): (&'a PrimitiveGroup,))
                               -> groups::Nodes<'a>
    {
        groups::nodes(group, self.block)
    }
}

pub fn nodes<'a>(block: &'a PrimitiveBlock) -> Nodes<'a> {
    block.get_primitivegroup().iter().flat_map(FnNodes { block: block })
}

pub type Ways<'a> = FlatMap<&'a PrimitiveGroup, Way, slice::Items<'a, PrimitiveGroup>, groups::Ways<'a>, FnWays<'a>>;

struct FnWays<'a> { block: &'a PrimitiveBlock }
impl<'a> Fn(&'a PrimitiveGroup) -> groups::Ways<'a> for FnWays<'a> {
    extern "rust-call" fn call(&self, (group,): (&'a PrimitiveGroup,))
                               -> groups::Ways<'a>
    {
        groups::ways(group, self.block)
    }
}

pub fn ways<'a>(block: &'a PrimitiveBlock) -> Ways<'a> {
    block.get_primitivegroup().iter().flat_map(FnWays { block: block })
}

pub type Relations<'a> = FlatMap<&'a PrimitiveGroup, Relation, slice::Items<'a, PrimitiveGroup>, groups::Relations<'a>, FnRelations<'a>>;

struct FnRelations<'a> { block: &'a PrimitiveBlock }
impl<'a> Fn(&'a PrimitiveGroup) -> groups::Relations<'a> for FnRelations<'a> {
    extern "rust-call" fn call(&self, (group,): (&'a PrimitiveGroup,))
                               -> groups::Relations<'a>
    {
        groups::relations(group, self.block)
    }
}

pub fn relations<'a>(block: &'a PrimitiveBlock) -> Relations<'a> {
    block.get_primitivegroup().iter().flat_map(FnRelations { block: block })
}
