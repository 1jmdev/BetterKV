use super::constants::{INITIAL_BUCKETS, MAX_LOAD_FACTOR, NIL};
use super::iter::Iter;
use super::node::Node;
use super::table::Table;

pub struct RehashingMap<K, V> {
    pub(super) seed: u64,
    pub(super) table: Table,
    pub(super) nodes: Vec<Node<K, V>>,
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
            nodes: Vec::with_capacity(INITIAL_BUCKETS),
        }
    }

    pub fn len(&self) -> usize {
        let _trace = profiler::scope("rehash::types::len");
        self.nodes.len()
    }

    pub fn clear(&mut self) {
        let _trace = profiler::scope("rehash::types::clear");
        self.table = Table::with_buckets(INITIAL_BUCKETS);
        self.nodes.clear();
    }

    pub fn iter(&self) -> Iter<'_, K, V> {
        let _trace = profiler::scope("rehash::types::iter");
        Iter::new(&self.nodes)
    }

    #[inline(always)]
    pub(super) fn maybe_grow(&mut self) {
        let _trace = profiler::scope("rehash::types::maybe_grow");
        if self.nodes.len() < self.table.len() * MAX_LOAD_FACTOR {
            return;
        }
        self.resize_to(self.table.len() * 2);
    }

    pub(super) fn reserve_for_batch(&mut self, additional: usize) {
        if additional == 0 { return; }

        let required = self.nodes.len().saturating_add(additional);
        let bucket_need = required.div_ceil(MAX_LOAD_FACTOR).next_power_of_two();
        if bucket_need > self.table.len() {
            self.resize_to(bucket_need);
        }

        self.nodes.reserve(additional);
    }

    fn resize_to(&mut self, new_bucket_count: usize) {
        let _trace = profiler::scope("rehash::types::resize_to");
        let new_bucket_count = new_bucket_count.next_power_of_two();
        if new_bucket_count <= self.table.len() {
            return;
        }

        let mut new_table = Table::with_buckets(new_bucket_count);
        for idx in 0..self.nodes.len() {
            let node = &mut self.nodes[idx];
            let bucket = (node.hash as usize) & new_table.mask;
            node.next = new_table.heads[bucket];
            new_table.heads[bucket] = idx as u32;
        }
        self.table = new_table;
    }

    pub(super) fn patch_swapped(&mut self, old_idx: u32, new_idx: u32) {
        let _trace = profiler::scope("rehash::types::patch_swapped");
        if old_idx == new_idx {
            return;
        }

        let hash = self.nodes[new_idx as usize].hash;
        let bucket = (hash as usize) & self.table.mask;
        let mut cur = self.table.heads[bucket];
        let mut prev = NIL;

        while cur != NIL {
            if cur == old_idx {
                if prev == NIL {
                    self.table.heads[bucket] = new_idx;
                } else {
                    self.nodes[prev as usize].next = new_idx;
                }
                return;
            }
            prev = cur;
            cur = self.nodes[cur as usize].next;
        }
    }
}

fn random_seed() -> u64 {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};
    let s = RandomState::new();
    let mut h = s.build_hasher();
    h.write_u64(0xdeadbeefcafe1234);
    h.finish()
}
