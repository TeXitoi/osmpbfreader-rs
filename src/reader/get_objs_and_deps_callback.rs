use std::collections::{HashMap, HashSet};
use std::ops::Deref;
use std::rc::Rc;

use super::*;

struct PendingObj {
    obj: Rc<OsmObj>,
    found_deps: HashSet<Rc<OsmObj>>,
    missing_ids: HashSet<OsmId>,
}
impl PendingObj {
    fn new(obj: Rc<OsmObj>) -> Self {
        Self {
            found_deps: HashSet::new(),
            missing_ids: obj.dependencies().into_iter().collect(),
            obj,
        }
    }
    fn to_obj_and_dependencies(&self) -> ObjAndDependencies {
        ObjAndDependencies {
            dependencies: self
                .found_deps
                .iter()
                .map(|obj| (obj.id(), obj.deref().clone()))
                .collect(),
            obj: (*self.obj).clone(),
        }
    }
    fn insert_if_missing(&mut self, other: Rc<OsmObj>) {
        // TODO: the flamechart shows this method does _horribly_.
        // Must be optimized.
        if !self.missing_ids.remove(&other.id()) {
            // Object was not wanted: noop
            return;
        }
        // Object wanted, insert and update dependencies
        for id in other.dependencies() {
            self.missing_ids.insert(id);
        }
        for dep in self.found_deps.iter() {
            self.missing_ids.remove(&dep.id());
        }
    }
    fn complete(&self) -> bool {
        self.missing_ids.is_empty()
    }
}

trait ObjDependencyIds {
    fn dependencies(&self) -> Vec<OsmId>;
}
impl ObjDependencyIds for OsmObj {
    /// Get IDs of first level dependencies
    fn dependencies(&self) -> Vec<OsmId> {
        let it_way = self
            .way()
            .into_iter()
            .flat_map(|w| &w.nodes)
            .cloned()
            .map(OsmId::Node);
        let it_rel = self
            .relation()
            .into_iter()
            .flat_map(|w| &w.refs)
            .map(|r| r.member);
        it_way.chain(it_rel).collect()
    }
}

fn insert_into_pending(pending: &mut HashMap<OsmId, PendingObj>, obj: Rc<OsmObj>) {
    for (_id, pending) in pending.iter_mut() {
        pending.insert_if_missing(obj.clone());
    }
}

fn pop_finish(
    pending: &mut HashMap<OsmId, PendingObj>,
) -> impl Iterator<Item = ObjAndDependencies> {
    let result: Vec<ObjAndDependencies> = pending
        .values()
        .filter(|p| p.complete())
        .map(PendingObj::to_obj_and_dependencies)
        .collect();
    pending.retain(|_key, p| !p.complete());
    result.into_iter()
}

/// Object fulfilling a search predicate,
/// containing all recursive dependencies.
/// It will be the only argument for the provided callback in [get_objs_and_deps_callback].
pub struct ObjAndDependencies {
    /// The main object fulfilling the search predicate.
    pub obj: OsmObj,
    /// All recusrive dependencies of obj.
    pub dependencies: HashMap<OsmId, OsmObj>,
}

/// Tell [get_objs_and_deps_callback] whether to continue
/// it's search or not.
#[derive(PartialEq, Eq)]
pub enum GetObjsContinue {
    /// Instructs get_objs_and_deps_callback to continue.
    Continue,
    /// Instructs get_objs_and_deps_callback to stop and return.
    Stop,
}

impl<R: io::Read> OsmPbfReader<R> {
    /// If files get too big and the memory is limited,
    /// this method allows to process all objects in an
    /// efficient fashion. max_loaded_object is an upper bound, of how many
    /// predicate matching objects are to be kept in memory.
    /// Keep in mind, that every predicate matching object also keeps
    /// their dependencies until they are complete.
    /// Higher values require more memory, but less passes over the file.
    pub fn get_objs_and_deps_callback<F, C>(
        &mut self,
        mut pred: F,
        mut callback: C,
        max_loaded_objects: usize,
    ) -> Result<()>
    where
        R: io::Seek,
        F: FnMut(&OsmObj) -> bool,
        C: FnMut(ObjAndDependencies) -> GetObjsContinue,
    {
        let mut last_id_of_marked_as_pending: Option<OsmId> = None;
        // For pending object we want something like a vector,
        // but to pop in O(1). We can't take a hash map, because we
        // might alter the object.
        let mut pending_objects: HashMap<OsmId, PendingObj> = HashMap::new();
        let mut processed_counter: usize = 0;

        // How often to pop the completed objects.
        const PROCESSED_MODULO: usize = 128;

        'outer: loop {
            self.rewind()?;
            let mut obj_iter = self.iter();

            // Phase 1: Only fetch dependencies.
            // We could, for example, be in the second round, starting from the beginning,
            // only completing pending objects, because there were too many in memory.
            for obj in obj_iter.by_ref() {
                let obj = Rc::new(obj?);
                insert_into_pending(&mut pending_objects, obj.clone());
                if processed_counter % PROCESSED_MODULO == 0 {
                    for obj in pop_finish(&mut pending_objects) {
                        if callback(obj) == GetObjsContinue::Stop {
                            break 'outer;
                        }
                    }
                }
                processed_counter += 1;
                if last_id_of_marked_as_pending.is_none() {
                    break;
                }
                if Some(obj.id()) == last_id_of_marked_as_pending {
                    break;
                }
            }

            // Phase 2: Continue collecting new pending objects matching the predicate, until too many pending.
            // The first phase is complete and potentially freed space to add more pending objects.
            // We continue to complete pending objects.
            for obj in obj_iter.by_ref() {
                let obj = Rc::new(obj?);
                insert_into_pending(&mut pending_objects, obj.clone());
                if processed_counter % PROCESSED_MODULO == 0 {
                    for obj in pop_finish(&mut pending_objects) {
                        if callback(obj) == GetObjsContinue::Stop {
                            break 'outer;
                        }
                    }
                }
                processed_counter += 1;
                last_id_of_marked_as_pending = Some(obj.id());
                if pred(&obj) {
                    pending_objects.insert(obj.id(), PendingObj::new(obj));
                }
                if pending_objects.len() >= max_loaded_objects {
                    break;
                }
            }

            // Phase 3: Continue while only completing pending objects.
            // There are many pending objects again, and we only want to complete them  /
            // reduce the count.
            // TODO: is this phase necessary, or are dependencies _always_ listed before
            // there parent?
            for obj in obj_iter.by_ref() {
                let obj = Rc::new(obj?);
                insert_into_pending(&mut pending_objects, obj.clone());
                if processed_counter % PROCESSED_MODULO == 0 {
                    for obj in pop_finish(&mut pending_objects) {
                        if callback(obj) == GetObjsContinue::Stop {
                            break 'outer;
                        }
                    }
                }
                processed_counter += 1;
            }

            for obj in pop_finish(&mut pending_objects) {
                if callback(obj) == GetObjsContinue::Stop {
                    break 'outer;
                }
            }
            if pending_objects.is_empty() {
                break;
            }
        }
        Ok(())
    }
}
