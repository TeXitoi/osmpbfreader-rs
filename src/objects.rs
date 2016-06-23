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
    pub fn id(&self) -> OsmId {
        match *self {
            OsmObj::Node(ref node) => OsmId::Node(node.id),
            OsmObj::Way(ref way) => OsmId::Way(way.id),
            OsmObj::Relation(ref rel) => OsmId::Relation(rel.id),
        }
    }
    pub fn is_node(&self) -> bool {
        self.node().is_some()
    }
    pub fn is_way(&self) -> bool {
        self.way().is_some()
    }
    pub fn is_relation(&self) -> bool {
        self.relation().is_some()
    }
    pub fn node(&self) -> Option<&Node> {
        match *self {
            OsmObj::Node(ref n) => Some(n),
            _ => None
        }
    }
    pub fn way(&self) -> Option<&Way> {
        match *self {
            OsmObj::Way(ref w) => Some(w),
            _ => None
        }
    }
    pub fn relation(&self) -> Option<&Relation> {
        match *self {
            OsmObj::Relation(ref r) => Some(r),
            _ => None
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
impl OsmId {
    pub fn is_node(&self) -> bool {
        match *self {
            OsmId::Node(_) => true,
            _ => false
        }
    }
    pub fn is_way(&self) -> bool {
        match *self {
            OsmId::Way(_) => true,
            _ => false
        }
    }
    pub fn is_relation(&self) -> bool {
        match *self {
            OsmId::Relation(_) => true,
            _ => false
        }
    }
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
