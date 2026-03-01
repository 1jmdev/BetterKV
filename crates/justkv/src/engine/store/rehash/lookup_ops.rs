use std::borrow::Borrow;
use std::hash::Hash;

use super::constants::REHASH_STEPS_PER_WRITE;
use super::index::{bucket_index, find_in_chain};
use super::types::RehashingMap;

impl<K, V> RehashingMap<K, V>
where
    K: Eq + Hash,
{
    pub(in crate::engine::store) fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.find_index(key).is_some()
    }

    pub(in crate::engine::store) fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let idx = self.find_index(key)?;
        self.nodes[idx as usize].as_ref().map(|node| &node.value)
    }

    pub(in crate::engine::store) fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.rehash_step(REHASH_STEPS_PER_WRITE);
        let idx = self.find_index(key)?;
        self.nodes[idx as usize]
            .as_mut()
            .map(|node| &mut node.value)
    }

    pub(in crate::engine::store::rehash) fn find_index<Q>(&self, key: &Q) -> Option<u32>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        if let Some(table) = self.rehash_table.as_ref() {
            let bucket = bucket_index(&self.hash_builder, key, table.len());
            if let Some(idx) = find_in_chain(&self.nodes, table.heads[bucket], key) {
                return Some(idx);
            }
        }

        let bucket = bucket_index(&self.hash_builder, key, self.table.len());
        find_in_chain(&self.nodes, self.table.heads[bucket], key)
    }
}
