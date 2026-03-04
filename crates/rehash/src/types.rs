use std::hash::Hash;

use ahash::RandomState;

use super::constants::{INITIAL_BUCKETS, MAX_LOAD_FACTOR, NIL};
use super::iter::Iter;
use super::node::Node;
use super::table::Table;

pub struct RehashingMap<K, V> {
    pub(super) hash_builder: RandomState,
    pub(super) table: Table,
    pub(super) nodes: Vec<Node<K, V>>,
}

impl<K, V> RehashingMap<K, V>
where
    K: Eq + Hash,
{
    pub fn new() -> Self {
        let _trace = profiler::scope("rehash::types::new");
        Self {
            hash_builder: RandomState::new(),
            table: Table::with_buckets(INITIAL_BUCKETS),
            nodes: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        let _trace = profiler::scope("rehash::types::len");
        self.nodes.len()
    }

    pub fn clear(&mut self) {
        let _trace = profiler::scope("rehash::types::clear");
        self.table = Table::with_buckets(INITIAL_BUCKETS);
        self.nodes = Vec::new();
    }

    pub fn iter(&self) -> Iter<'_, K, V> {
        let _trace = profiler::scope("rehash::types::iter");
        Iter::new(&self.nodes)
    }

    #[inline(always)]
    pub(super) fn maybe_resize_for_insert(&mut self) {
        let _trace = profiler::scope("rehash::types::maybe_resize_for_insert");
        let next_len = self.nodes.len() + 1;
        if next_len <= self.table.len() * MAX_LOAD_FACTOR {
            return;
        }
        let target = next_len.div_ceil(MAX_LOAD_FACTOR);
        self.resize_table(target);
    }

    pub(super) fn reserve_for_batch(&mut self, additional: usize) {
        let _trace = profiler::scope("rehash::types::reserve_for_batch");
        if additional == 0 {
            return;
        }

        let required_len = self.nodes.len().saturating_add(additional);
        if required_len > self.table.len() * MAX_LOAD_FACTOR {
            let target_buckets = required_len.div_ceil(MAX_LOAD_FACTOR);
            self.resize_table(target_buckets);
        }

        if required_len > self.nodes.capacity() {
            let grow_by = (self.nodes.capacity() / 2).max(1024);
            let target = required_len.min(self.nodes.capacity().saturating_add(grow_by));
            if target > self.nodes.capacity() {
                self.nodes.reserve_exact(target - self.nodes.capacity());
            }
        }
    }

    pub(super) fn resize_table(&mut self, target_buckets: usize) {
        let _trace = profiler::scope("rehash::types::resize_table");
        let current = self.table.len();
        let mut new_count = current;
        while new_count < target_buckets {
            new_count = new_count + (new_count / 2).max(8);
        }

        if new_count == current {
            return;
        }

        let mut new_table = Table::with_buckets(new_count);
        for idx in 0..self.nodes.len() {
            let node = &mut self.nodes[idx];
            let bucket = super::index::bucket_index_from_hash(node.hash, new_table.len());
            node.next = new_table.heads[bucket];
            new_table.heads[bucket] = idx as u32;
        }
        self.table = new_table;
    }

    #[inline(always)]
    pub(super) fn unlink_index_from_bucket(&mut self, bucket: usize, idx: u32) -> u32 {
        let _trace = profiler::scope("rehash::types::unlink_index_from_bucket");
        let mut cur = self.table.heads[bucket];
        let mut prev = NIL;

        while cur != NIL {
            if cur == idx {
                let next = self.nodes[idx as usize].next;
                if prev == NIL {
                    self.table.heads[bucket] = next;
                } else {
                    self.nodes[prev as usize].next = next;
                }
                return next;
            }
            prev = cur;
            cur = self.nodes[cur as usize].next;
        }

        unreachable!("index not found in bucket chain");
    }

    pub(super) fn patch_moved_index(&mut self, old_idx: u32, new_idx: u32) {
        let _trace = profiler::scope("rehash::types::patch_moved_index");
        if old_idx == new_idx {
            return;
        }

        let moved_hash = self.nodes[new_idx as usize].hash;
        let bucket = super::index::bucket_index_from_hash(moved_hash, self.table.len());
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

        unreachable!("moved index not found in chain");
    }
}
