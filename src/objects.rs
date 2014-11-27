use std::collections::BTreeMap;

pub type Tags = BTreeMap<String, String>;

#[deriving(Show, PartialEq, PartialOrd, Clone)]
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

#[deriving(Show, PartialEq, PartialOrd, Clone)]
pub struct Node {
    pub id: i64,
    pub lat: f64,
    pub lon: f64,
    pub tags: Tags,
}

#[deriving(Show, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct Way {
    pub id: i64,
    pub nodes: Vec<i64>,
    pub tags: Tags,
}

#[deriving(Show, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub enum RelMem {
    Node(i64),
    Way(i64),
    Relation(i64),
}

#[deriving(Show, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct Ref {
    pub member: RelMem,
    pub role: String,
}

#[deriving(Show, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct Relation {
    pub id: i64,
    pub refs: Vec<Ref>,
    pub tags: Tags,
}
