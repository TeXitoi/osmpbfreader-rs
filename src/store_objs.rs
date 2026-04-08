use crate::objects::{OsmId, OsmObj};
use std::collections::BTreeMap;

/// Trait to allow generic objects (not just BTreeMap) in some methods.
pub trait StoreObjs {
    /// Insert given object at given key index.
    fn insert(&mut self, key: OsmId, value: OsmObj);
    /// Check if object contains the given key.
    fn contains_key(&self, key: &OsmId) -> bool;
}

impl StoreObjs for BTreeMap<OsmId, OsmObj> {
    fn insert(&mut self, key: OsmId, value: OsmObj) {
        self.insert(key, value);
    }

    fn contains_key(&self, key: &OsmId) -> bool {
        self.contains_key(key)
    }
}
