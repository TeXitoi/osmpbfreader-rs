// Copyright (c) 2014-2015 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

//! This module proposes objects to modelize OpenStreetMap objects.
//!
//! There is 3 types of object: nodes, ways and relations.

use std::ops::{Deref, DerefMut};
use std::iter::FromIterator;

/// Tags represents the features of the objects.  See the
/// [OpenStreetMap wiki page about
/// tags](http://wiki.openstreetmap.org/wiki/Tags) for more
/// information.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tags(TagsImpl);
type TagsImpl = ::flat_map::FlatMap<String, String>;
impl Tags {
    /// Creates a new, empty `Tags` object.
    pub fn new() -> Tags {
        Tags(TagsImpl::new())
    }
    /// Returns if contains the tag `(key, value)`.
    pub fn contains(&self, key: &str, value: &str) -> bool {
        self.0.get(key).map_or(false, |v| v.as_str() == value)
    }
}
impl Deref for Tags {
    type Target = TagsImpl;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Tags {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl FromIterator<(String, String)> for Tags {
    fn from_iter<T: IntoIterator<Item=(String, String)>>(iter: T) -> Self {
        Tags(iter.into_iter().collect())
    }
}

/// A node identifier
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Copy)]
pub struct NodeId(pub i64);

/// A way identifier
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Copy)]
pub struct WayId(pub i64);

/// A relation identifier
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Copy)]
pub struct RelationId(pub i64);

/// An OpenStreetMap object identifier
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Copy)]
pub enum OsmId {
    /// The identifier of a node
    Node(NodeId),
    /// The identifier of a way
    Way(WayId),
    /// The identifier of a relation
    Relation(RelationId),
}
impl OsmId {
    /// Returns `true` if the id is a node id.
    pub fn is_node(&self) -> bool {
        match *self {
            OsmId::Node(_) => true,
            _ => false,
        }
    }
    /// Returns `true` if the id is a way id.
    pub fn is_way(&self) -> bool {
        match *self {
            OsmId::Way(_) => true,
            _ => false,
        }
    }
    /// Returns `true` if the id is a relation id.
    pub fn is_relation(&self) -> bool {
        match *self {
            OsmId::Relation(_) => true,
            _ => false,
        }
    }
}

/// An OpenStreetMap object.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum OsmObj {
    /// A node
    Node(Node),
    /// A way
    Way(Way),
    /// A relation
    Relation(Relation),
}
impl OsmObj {
    /// Returns the tags of the object.
    pub fn tags(&self) -> &Tags {
        match *self {
            OsmObj::Node(ref node) => &node.tags,
            OsmObj::Way(ref way) => &way.tags,
            OsmObj::Relation(ref rel) => &rel.tags,
        }
    }
    /// Returns the id of the object.
    pub fn id(&self) -> OsmId {
        match *self {
            OsmObj::Node(ref node) => OsmId::Node(node.id),
            OsmObj::Way(ref way) => OsmId::Way(way.id),
            OsmObj::Relation(ref rel) => OsmId::Relation(rel.id),
        }
    }
    /// Returns `true` if the object is a node.
    pub fn is_node(&self) -> bool {
        self.node().is_some()
    }
    /// Returns `true` if the object is a way.
    pub fn is_way(&self) -> bool {
        self.way().is_some()
    }
    /// Returns `true` if the object is a relation.
    pub fn is_relation(&self) -> bool {
        self.relation().is_some()
    }
    /// Gets a reference to the node in an `Option`.
    pub fn node(&self) -> Option<&Node> {
        match *self {
            OsmObj::Node(ref n) => Some(n),
            _ => None,
        }
    }
    /// Gets a reference to the way in an `Option`.
    pub fn way(&self) -> Option<&Way> {
        match *self {
            OsmObj::Way(ref w) => Some(w),
            _ => None,
        }
    }
    /// Gets a reference to the relation in an `Option`.
    pub fn relation(&self) -> Option<&Relation> {
        match *self {
            OsmObj::Relation(ref r) => Some(r),
            _ => None,
        }
    }
}

/// An OpenStreetMap node.  See the [OpenStreetMap wiki page about
/// node](http://wiki.openstreetmap.org/wiki/Node) for more
/// information.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct Node {
    /// The id of the node.
    pub id: NodeId,
    /// The tags of the node.
    pub tags: Tags,
    /// The latitude in decimicro degrees (10⁻⁷ degrees).
    pub decimicro_lat: i32,
    /// The longitude in decimicro degrees (10⁻⁷ degrees).
    pub decimicro_lon: i32,
}
impl Node {
    /// Returns the latitude of the node in degrees.
    pub fn lat(&self) -> f64 {
        self.decimicro_lat as f64 * 1e-7
    }
    /// Returns the longitude of the node in degrees.
    pub fn lon(&self) -> f64 {
        self.decimicro_lon as f64 * 1e-7
    }
}

/// An OpenStreetMap way.  See the [OpenStreetMap wiki page about
/// way](http://wiki.openstreetmap.org/wiki/Way) for more
/// information.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct Way {
    /// The id of the way.
    pub id: WayId,
    /// The tags of the way.
    pub tags: Tags,
    /// The ordered list of nodes as id.
    pub nodes: Vec<NodeId>,
}

/// A reference to an object with a role.  Used in the relation object.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct Ref {
    /// Id of the member.
    pub member: OsmId,
    /// Role of the member.
    pub role: String,
}

/// An OpenStreetMap relation.  See the [OpenStreetMap wiki page about
/// relation](http://wiki.openstreetmap.org/wiki/Relation) for more
/// information.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct Relation {
    /// The id of the relation.
    pub id: RelationId,
    /// The tags of the relation.
    pub tags: Tags,
    /// Members of the relation.
    pub refs: Vec<Ref>,
}

impl ::std::convert::From<NodeId> for OsmId {
    fn from(n: NodeId) -> Self {
        OsmId::Node(n)
    }
}
impl ::std::convert::From<WayId> for OsmId {
    fn from(w: WayId) -> Self {
        OsmId::Way(w)
    }
}
impl ::std::convert::From<RelationId> for OsmId {
    fn from(r: RelationId) -> Self {
        OsmId::Relation(r)
    }
}
impl ::std::convert::From<Node> for OsmObj {
    fn from(n: Node) -> Self {
        OsmObj::Node(n)
    }
}
impl ::std::convert::From<Way> for OsmObj {
    fn from(w: Way) -> Self {
        OsmObj::Way(w)
    }
}
impl ::std::convert::From<Relation> for OsmObj {
    fn from(r: Relation) -> Self {
        OsmObj::Relation(r)
    }
}
