use std::borrow::Borrow;
use std::hash::Hash;

use super::index::{bucket_index_from_hash, hash_key};
use super::types::RehashingMap;

impl<K, V> RehashingMap<K, V>
where
    K: Eq + Hash,
{
    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let _trace = profiler::scope("rehash::remove::remove");
        let hash = hash_key(&self.hash_builder, key);
        let idx = self.find_index_hashed(key, hash)?;
        let bucket = bucket_index_from_hash(hash, self.table.len());
        let _ = self.unlink_index_from_bucket(bucket, idx);

        let removed = self.nodes.swap_remove(idx as usize);
        if (idx as usize) < self.nodes.len() {
            let old_last_idx = self.nodes.len() as u32;
            self.patch_moved_index(old_last_idx, idx);
        }

        Some(removed.value)
    }
}
