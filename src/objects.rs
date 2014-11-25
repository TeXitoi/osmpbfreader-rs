use std::collections::BTreeMap;

pub type Tags = BTreeMap<String, String>;

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
pub enum RelationMember {
    Node(i64),
    Way(i64),
    Relation(i64),
}

#[deriving(Show, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct Reference {
    pub member: RelationMember,
    pub role: String,
}

#[deriving(Show, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct Relation {
    pub id: i64,
    pub refs: Vec<Reference>,
    pub tags: Tags,
}
