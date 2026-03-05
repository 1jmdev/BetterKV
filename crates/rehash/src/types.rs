use super::constants::{INITIAL_BUCKETS, MAX_LOAD_FACTOR, NIL};
use super::iter::Iter;
use super::node::NodeMeta;
use super::table::Table;

pub struct RehashingMap<K, V> {
    pub(super) seed: u64,
    pub(super) table: Table,
    // SoA (Structure of Arrays) Layout:
    pub(super) metas: Vec<NodeMeta>,
    pub(super) keys: Vec<K>,
    pub(super) values: Vec<V>,
}

impl<K, V> RehashingMap<K, V>
where
    K: Eq + AsRef<[u8]>,
{
    pub fn new() -> Self {
        let _trace = profiler::scope("rehash::types::new");
        Self {
            seed: random_seed(),
            table: Table::with_buckets(INITIAL_BUCKETS),
            metas: Vec::with_capacity(INITIAL_BUCKETS),
            keys: Vec::with_capacity(INITIAL_BUCKETS),
            values: Vec::with_capacity(INITIAL_BUCKETS),
        }
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        let _trace = profiler::scope("rehash::types::len");
        self.metas.len()
    }

    pub fn clear(&mut self) {
        let _trace = profiler::scope("rehash::types::clear");
        self.table = Table::with_buckets(INITIAL_BUCKETS);
        self.metas.clear();
        self.keys.clear();
        self.values.clear();
    }

    pub fn iter(&self) -> Iter<'_, K, V> {
        let _trace = profiler::scope("rehash::types::iter");
        Iter::new(&self.keys, &self.values)
    }

    #[inline(always)]
    pub(super) fn maybe_grow(&mut self) {
        let _trace = profiler::scope("rehash::types::maybe_grow");
        if self.metas.len() < self.table.len() * MAX_LOAD_FACTOR {
            return;
        }
        self.resize_to(self.table.len() * 2);
    }

    pub(super) fn reserve_for_batch(&mut self, additional: usize) {
        let _trace = profiler::scope("rehash::types::reserve_for_batch");
        if additional == 0 {
            return;
        }
        let required = self.metas.len().saturating_add(additional);
        let bucket_need = required.div_ceil(MAX_LOAD_FACTOR).next_power_of_two();
        if bucket_need > self.table.len() {
            self.resize_to(bucket_need);
        }
        self.metas.reserve(additional);
        self.keys.reserve(additional);
        self.values.reserve(additional);
    }

    fn resize_to(&mut self, new_bucket_count: usize) {
        let _trace = profiler::scope("rehash::types::resize_to");
        let new_bucket_count = new_bucket_count.next_power_of_two();
        if new_bucket_count <= self.table.len() {
            return;
        }

        let mut new_table = Table::with_buckets(new_bucket_count);
        // Using unchecked accesses for extremely fast rehashing
        unsafe {
            let heads_ptr = new_table.heads.as_mut_ptr();
            let metas_ptr = self.metas.as_mut_ptr();
            let mask = new_table.mask;

            for idx in 0..self.metas.len() {
                let meta = &mut *metas_ptr.add(idx);
                let bucket = (meta.hash as usize) & mask;
                meta.next = *heads_ptr.add(bucket);
                *heads_ptr.add(bucket) = idx as u32;
            }
        }
        self.table = new_table;
    }

    pub(super) fn patch_swapped(&mut self, old_idx: u32, new_idx: u32) {
        let _trace = profiler::scope("rehash::types::patch_swapped");
        if old_idx == new_idx {
            return;
        }

        unsafe {
            let hash = (*self.metas.as_ptr().add(new_idx as usize)).hash;
            let bucket = (hash as usize) & self.table.mask;
            let heads_ptr = self.table.heads.as_mut_ptr();
            let metas_ptr = self.metas.as_mut_ptr();

            let mut cur = *heads_ptr.add(bucket);
            let mut prev = NIL;

            while cur != NIL {
                if cur == old_idx {
                    if prev == NIL {
                        *heads_ptr.add(bucket) = new_idx;
                    } else {
                        (*metas_ptr.add(prev as usize)).next = new_idx;
                    }
                    return;
                }
                prev = cur;
                cur = (*metas_ptr.add(cur as usize)).next;
            }
        }
    }
}

fn random_seed() -> u64 {
    let _trace = profiler::scope("rehash::types::random_seed");
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};
    RandomState::new().build_hasher().finish()
}
