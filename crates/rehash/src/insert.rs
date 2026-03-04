use std::hash::Hash;

use super::constants::BULK_RESERVE_CAP;
use super::index::{bucket_index_from_hash, hash_key};
use super::node::Node;
use super::types::RehashingMap;

impl<K, V> RehashingMap<K, V>
where
    K: Eq + Hash,
{
    pub fn insert_batch<I>(&mut self, entries: I)
    where
        I: IntoIterator<Item = (K, V)>,
    {
        let _trace = profiler::scope("rehash::insert::insert_batch");
        let iter = entries.into_iter();
        let (lower_bound, _) = iter.size_hint();
        self.reserve_for_batch(lower_bound.min(BULK_RESERVE_CAP));

        for (key, value) in iter {
            self.insert(key, value);
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let _trace = profiler::scope("rehash::insert::insert");
        let hash = hash_key(&self.hash_builder, &key);
        if let Some(idx) = self.find_index_hashed(&key, hash) {
            let node = &mut self.nodes[idx as usize];
            return Some(std::mem::replace(&mut node.value, value));
        }

        self.maybe_resize_for_insert();
        let bucket = bucket_index_from_hash(hash, self.table.len());
        let head = self.table.heads[bucket];
        let idx = self.nodes.len() as u32;
        self.nodes.push(Node {
            hash,
            next: head,
            key,
            value,
        });
        self.table.heads[bucket] = idx;
        None
    }

    pub fn get_or_insert_with<F>(&mut self, key: K, default: F) -> &mut V
    where
        F: FnOnce() -> V,
    {
        let _trace = profiler::scope("rehash::insert::get_or_insert_with");
        let hash = hash_key(&self.hash_builder, &key);
        if let Some(idx) = self.find_index_hashed(&key, hash) {
            return &mut self.nodes[idx as usize].value;
        }

        self.maybe_resize_for_insert();
        let bucket = bucket_index_from_hash(hash, self.table.len());
        let head = self.table.heads[bucket];
        let idx = self.nodes.len() as u32;
        self.nodes.push(Node {
            hash,
            next: head,
            key,
            value: default(),
        });
        self.table.heads[bucket] = idx;
        &mut self.nodes[idx as usize].value
    }
}
