use std::collections::BTreeMap;

pub type Tags = BTreeMap<String, String>;

#[deriving(Show)]
pub struct Node {
    pub id: i64,
    pub lat: f64,
    pub lon: f64,
    pub tags: Tags,
}

#[deriving(Show)]
pub struct Way {
    pub id: i64,
    pub nodes: Vec<i64>,
    pub tags: Tags,
}
