// Copyright (c) 2014-2015 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

use std::collections::BTreeMap;

pub type Tags = BTreeMap<String, String>;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum OsmObj {
    Node(Node),
    Way(Way),
    Relation(Relation),
}
impl OsmObj {
    pub fn tags(&self) -> &Tags {
        match *self {
            OsmObj::Node(ref node) => &node.tags,
            OsmObj::Way(ref way) => &way.tags,
            OsmObj::Relation(ref rel) => &rel.tags,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Node {
    pub id: i64,
    pub lat: f64,
    pub lon: f64,
    pub tags: Tags,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct Way {
    pub id: i64,
    pub nodes: Vec<i64>,
    pub tags: Tags,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Copy)]
pub enum OsmId {
    Node(i64),
    Way(i64),
    Relation(i64),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct Ref {
    pub member: OsmId,
    pub role: String,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct Relation {
    pub id: i64,
    pub refs: Vec<Ref>,
    pub tags: Tags,
}
