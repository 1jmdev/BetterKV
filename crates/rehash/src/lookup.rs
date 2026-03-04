use std::borrow::Borrow;
use std::hash::Hash;

use super::constants::NIL;
use super::index::{bucket_index_from_hash, hash_key};
use super::types::RehashingMap;

impl<K, V> RehashingMap<K, V>
where
    K: Eq + Hash,
{
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let _trace = profiler::scope("rehash::lookup::contains_key");
        self.find_index(key).is_some()
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let _trace = profiler::scope("rehash::lookup::get");
        let idx = self.find_index(key)?;
        Some(&self.nodes[idx as usize].value)
    }

    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let _trace = profiler::scope("rehash::lookup::get_mut");
        let idx = self.find_index(key)?;
        Some(&mut self.nodes[idx as usize].value)
    }

    #[inline(always)]
    pub fn find_index<Q>(&self, key: &Q) -> Option<u32>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let _trace = profiler::scope("rehash::lookup::find_index");
        let hash = hash_key(&self.hash_builder, key);
        self.find_index_hashed(key, hash)
    }

    #[inline(always)]
    pub fn find_index_hashed<Q>(&self, key: &Q, hash: u64) -> Option<u32>
    where
        K: Borrow<Q>,
        Q: Eq + ?Sized,
    {
        let _trace = profiler::scope("rehash::lookup::find_index_hashed");
        let bucket = bucket_index_from_hash(hash, self.table.len());
        let mut idx = self.table.heads[bucket];
        while idx != NIL {
            let node = &self.nodes[idx as usize];
            if node.hash == hash && node.key.borrow() == key {
                return Some(idx);
            }
            idx = node.next;
        }
        None
    }
}
