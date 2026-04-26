// Copyright (c) 2014-2015 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

//! This module proposes objects to modelize OpenStreetMap objects.
//!
//! There are 3 types of objects: nodes, ways and relations.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(all(feature = "serde", feature = "fast-tags"))]
use serde::{Deserializer, Serializer};
use smartstring::alias::String;
use std::iter::FromIterator;

#[cfg(feature = "fast-tags")]
type TagStore = smallvec::SmallVec<[(String, String); 4]>;
#[cfg(not(feature = "fast-tags"))]
type TagStore = ::flat_map::FlatMap<String, String>;

/// Tags represents the features of the objects.  See the
/// [OpenStreetMap wiki page about
/// tags](http://wiki.openstreetmap.org/wiki/Tags) for more
/// information.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
    all(feature = "serde", not(feature = "fast-tags")),
    derive(Serialize, Deserialize)
)]
pub struct Tags(TagStore);

#[cfg(all(feature = "serde", feature = "fast-tags"))]
impl Serialize for Tags {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(self.len()))?;
        for (k, v) in self.iter() {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[cfg(all(feature = "serde", feature = "fast-tags"))]
impl<'de> Deserialize<'de> for Tags {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let pairs = std::collections::HashMap::<String, String>::deserialize(deserializer)?;
        Ok(Tags(smallvec::SmallVec::from_iter(pairs)))
    }
}

impl Tags {
    /// Creates a new, empty `Tags` object.
    pub fn new() -> Tags {
        Tags(TagStore::new())
    }

    /// Returns if it contains a tag with the given `key` and `value`.
    pub fn contains(&self, key: &str, value: &str) -> bool {
        self.get(key).is_some_and(|v| v.as_str() == value)
    }

    /// Returns the value associated with the given key, if any.
    pub fn get(&self, key: &str) -> Option<&String> {
        #[cfg(feature = "fast-tags")]
        {
            self.0
                .iter()
                .find(|(k, _)| k.as_str() == key)
                .map(|(_, v)| v)
        }
        #[cfg(not(feature = "fast-tags"))]
        {
            self.0.get(key)
        }
    }

    /// Inserts a key-value pair, returning the old value if the key existed.
    pub fn insert(&mut self, key: String, value: String) -> Option<String> {
        #[cfg(feature = "fast-tags")]
        {
            if let Some(pos) = self.0.iter().position(|(k, _)| k.as_str() == key.as_str()) {
                let (_, old) = self.0.get_mut(pos).unwrap();
                let prev = ::std::mem::replace(old, value);
                Some(prev)
            } else {
                self.0.push((key, value));
                None
            }
        }
        #[cfg(not(feature = "fast-tags"))]
        {
            self.0.insert(key, value)
        }
    }

    /// Returns true if the tags contain the given key.
    pub fn contains_key(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    /// Returns the number of tags.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if there are no tags.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns an iterator over the key-value pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.0.iter().map(|(k, v)| (k, v))
    }

    /// Shrinks the capacity of the underlying storage.
    pub fn shrink_to_fit(&mut self) {
        self.0.shrink_to_fit();
    }

    /// Consume tags into inner `FlatMap` representation.
    pub fn into_inner(self) -> ::flat_map::FlatMap<String, String> {
        #[cfg(feature = "fast-tags")]
        {
            self.0.into_iter().collect()
        }
        #[cfg(not(feature = "fast-tags"))]
        {
            self.0
        }
    }
}

impl FromIterator<(String, String)> for Tags {
    fn from_iter<T: IntoIterator<Item = (String, String)>>(iter: T) -> Self {
        let mut pairs: Vec<(String, String)> = iter.into_iter().collect();
        pairs.sort_by(|a, b| a.0.cmp(&b.0));
        pairs.dedup_by(|a, b| a.0 == b.0);
        #[cfg(feature = "fast-tags")]
        {
            Tags(smallvec::SmallVec::from_iter(pairs))
        }
        #[cfg(not(feature = "fast-tags"))]
        {
            Tags(::flat_map::FlatMap::from_iter(pairs))
        }
    }
}

/// A node identifier
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NodeId(pub i64);

/// A way identifier
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WayId(pub i64);

/// A relation identifier
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RelationId(pub i64);

/// An OpenStreetMap object identifier
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
        self.node().is_some()
    }
    /// Returns `true` if the id is a way id.
    pub fn is_way(&self) -> bool {
        self.way().is_some()
    }
    /// Returns `true` if the id is a relation id.
    pub fn is_relation(&self) -> bool {
        self.relation().is_some()
    }
    /// Returns the `NodeId` if it is a node, otherwise returns `None`.
    pub fn node(&self) -> Option<NodeId> {
        match *self {
            OsmId::Node(id) => Some(id),
            _ => None,
        }
    }
    /// Returns the `WayId` if it is a way, otherwise returns `None`.
    pub fn way(&self) -> Option<WayId> {
        match *self {
            OsmId::Way(id) => Some(id),
            _ => None,
        }
    }
    /// Returns the `RelationId` if it is a relation, otherwise returns `None`.
    pub fn relation(&self) -> Option<RelationId> {
        match *self {
            OsmId::Relation(id) => Some(id),
            _ => None,
        }
    }
    /// Returns the inner id.
    pub fn inner_id(&self) -> i64 {
        match *self {
            OsmId::Node(n) => n.0,
            OsmId::Way(n) => n.0,
            OsmId::Relation(n) => n.0,
        }
    }
}

/// An OpenStreetMap object.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
    /// Returns a reference to the `Node` if `self` is a node, otherwise returns `None`.
    pub fn node(&self) -> Option<&Node> {
        match *self {
            OsmObj::Node(ref n) => Some(n),
            _ => None,
        }
    }
    /// Returns a reference to the `Way` if `self` is a way, otherwise returns `None`.
    pub fn way(&self) -> Option<&Way> {
        match *self {
            OsmObj::Way(ref w) => Some(w),
            _ => None,
        }
    }
    /// Returns a reference to the `Relation` if `self` is a relation, otherwise returns `None`.
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Way {
    /// The id of the way.
    pub id: WayId,
    /// The tags of the way.
    pub tags: Tags,
    /// The ordered list of nodes as id.
    pub nodes: Vec<NodeId>,
}

impl Way {
    /// Returns true if the way is
    /// [open](http://wiki.openstreetmap.org/wiki/Way#Open_way).
    pub fn is_open(&self) -> bool {
        !self.is_closed()
    }
    /// Returns true if the way is
    /// [closed](http://wiki.openstreetmap.org/wiki/Way#Closed_way).
    pub fn is_closed(&self) -> bool {
        self.nodes.first() == self.nodes.last()
    }
}

/// A reference to an object with a role.  Used in the relation object.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
