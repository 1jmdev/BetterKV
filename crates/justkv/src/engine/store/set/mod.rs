mod algebra_ops;
mod core_ops;
mod random_ops;
mod scan_ops;

use ahash::RandomState;
use hashbrown::HashSet;

use crate::engine::value::{CompactKey, Entry, SetValue};

fn get_set(entry: &Entry) -> Option<&SetValue> {
    entry.as_set()
}

fn get_set_mut(entry: &mut Entry) -> Option<&mut SetValue> {
    entry.as_set_mut()
}

fn new_set() -> SetValue {
    HashSet::with_hasher(RandomState::new())
}

fn collect_members(set: &SetValue) -> Vec<CompactKey> {
    set.iter().cloned().collect()
}
