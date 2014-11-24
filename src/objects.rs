use std::collections::BTreeMap;

pub type Tags = BTreeMap<String, String>;

pub struct Node {
    pub id: i64,
    pub lat: f64,
    pub lon: f64,
    pub tags: Tags,
}
