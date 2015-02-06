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

pub type OsmObjs<'a> = FlatMap<slice::Iter<'a, PrimitiveGroup>, groups::OsmObjs<'a>, FnOsmObjs<'a>>;

struct FnOsmObjs<'a> { block: &'a PrimitiveBlock }
impl<'a> Fn<(&'a PrimitiveGroup,)>  for FnOsmObjs<'a> {
    type Output = groups::OsmObjs<'a>;
    extern "rust-call" fn call(&self, (group,): (&'a PrimitiveGroup,))
                               -> groups::OsmObjs<'a>
    {
        groups::iter(group, self.block)
    }
}

pub fn iter<'a>(block: &'a PrimitiveBlock) -> OsmObjs<'a> {
    block.get_primitivegroup().iter().flat_map(FnOsmObjs { block: block })
}

pub type Nodes<'a> = FlatMap<slice::Iter<'a, PrimitiveGroup>, groups::Nodes<'a>, FnNodes<'a>>;

struct FnNodes<'a> { block: &'a PrimitiveBlock }
impl<'a> Fn<(&'a PrimitiveGroup,)> for FnNodes<'a> {
    type Output = groups::Nodes<'a>;
    extern "rust-call" fn call(&self, (group,): (&'a PrimitiveGroup,))
                               -> groups::Nodes<'a>
    {
        groups::nodes(group, self.block)
    }
}

pub fn nodes<'a>(block: &'a PrimitiveBlock) -> Nodes<'a> {
    block.get_primitivegroup().iter().flat_map(FnNodes { block: block })
}

pub type Ways<'a> = FlatMap<slice::Iter<'a, PrimitiveGroup>, groups::Ways<'a>, FnWays<'a>>;

struct FnWays<'a> { block: &'a PrimitiveBlock }
impl<'a> Fn<(&'a PrimitiveGroup,)> for FnWays<'a> {
    type Output = groups::Ways<'a>;
    extern "rust-call" fn call(&self, (group,): (&'a PrimitiveGroup,))
                               -> groups::Ways<'a>
    {
        groups::ways(group, self.block)
    }
}

pub fn ways<'a>(block: &'a PrimitiveBlock) -> Ways<'a> {
    block.get_primitivegroup().iter().flat_map(FnWays { block: block })
}

pub type Relations<'a> = FlatMap<slice::Iter<'a, PrimitiveGroup>, groups::Relations<'a>, FnRelations<'a>>;

struct FnRelations<'a> { block: &'a PrimitiveBlock }
impl<'a> Fn<(&'a PrimitiveGroup,)> for FnRelations<'a> {
    type Output = groups::Relations<'a>;
    extern "rust-call" fn call(&self, (group,): (&'a PrimitiveGroup,))
                               -> groups::Relations<'a>
    {
        groups::relations(group, self.block)
    }
}

pub fn relations<'a>(block: &'a PrimitiveBlock) -> Relations<'a> {
    block.get_primitivegroup().iter().flat_map(FnRelations { block: block })
}
