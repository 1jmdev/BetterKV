use std::mem;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use super::constants::{
    INITIAL_BUCKETS, MAX_LOAD_FACTOR, NIL, REHASH_BUCKETS_PER_STEP, SMALL_REHASH_THRESHOLD,
};
use super::iter::Iter;
use super::node::NodeMeta;
use super::table::Table;

pub struct RehashingMap<K, V> {
    pub(super) seed: u64,
    pub(super) table: Table,
    pub(super) old_table: Option<Table>,
    pub(super) rehash_cursor: usize,
    pub(super) metas: Vec<NodeMeta>,
    pub(super) keys: Vec<K>,
    pub(super) values: Vec<V>,
}

impl<K, V> RehashingMap<K, V>
where
    K: Eq + AsRef<[u8]>,
{
    pub fn new() -> Self {
        Self::with_capacity(INITIAL_BUCKETS)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let storage_capacity = capacity.max(INITIAL_BUCKETS);
        Self {
            seed: random_seed(),
            table: Table::with_buckets(bucket_count_for_entries(storage_capacity)),
            old_table: None,
            rehash_cursor: 0,
            metas: Vec::with_capacity(storage_capacity),
            keys: Vec::with_capacity(storage_capacity),
            values: Vec::with_capacity(storage_capacity),
        }
    }

    pub fn reserve(&mut self, additional: usize) {
        self.reserve_for_batch(additional);
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.metas.len()
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.metas.is_empty()
    }

    pub fn clear(&mut self) {
        self.table = Table::with_buckets(INITIAL_BUCKETS);
        self.old_table = None;
        self.rehash_cursor = 0;
        self.metas.clear();
        self.keys.clear();
        self.values.clear();
    }

    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter::new(&self.keys, &self.values)
    }

    pub fn slices(&self) -> (&[K], &[V]) {
        (&self.keys, &self.values)
    }

    #[inline(always)]
    pub(super) fn maybe_grow(&mut self) {
        if self.old_table.is_some() {
            return;
        }
        if self.metas.len() < self.table.len() * MAX_LOAD_FACTOR {
            return;
        }
        self.start_rehash(self.table.len() * 2);
    }

    pub(super) fn reserve_for_batch(&mut self, additional: usize) {
        if additional == 0 {
            return;
        }
        let required = self.metas.len().saturating_add(additional);
        let bucket_need = required.div_ceil(MAX_LOAD_FACTOR).next_power_of_two();
        if self.old_table.is_none() && bucket_need > self.table.len() {
            self.start_rehash(bucket_need);
        }
        self.metas.reserve(additional);
        self.keys.reserve(additional);
        self.values.reserve(additional);
    }

    pub(super) fn rehash_step(&mut self, steps: usize) {
        let Some(old_table) = self.old_table.as_mut() else {
            return;
        };

        let steps = steps.max(1);
        let old_len = old_table.len();
        let metas_ptr = self.metas.as_mut_ptr();
        let heads_ptr = self.table.heads.as_mut_ptr();

        for _ in 0..steps {
            if self.rehash_cursor >= old_len {
                self.old_table = None;
                self.rehash_cursor = 0;
                return;
            }

            let bucket = self.rehash_cursor;
            self.rehash_cursor += 1;

            unsafe {
                let mut idx = old_table.heads[bucket];
                old_table.heads[bucket] = NIL;

                while idx != NIL {
                    let meta = &mut *metas_ptr.add(idx as usize);
                    let next = meta.next;
                    let new_bucket = self.table.bucket(meta.hash);
                    meta.next = *heads_ptr.add(new_bucket);
                    *heads_ptr.add(new_bucket) = idx;
                    idx = next;
                }
            }
        }

        if self.rehash_cursor >= old_len {
            self.old_table = None;
            self.rehash_cursor = 0;
        }
    }

    #[inline(always)]
    pub(super) fn rehash_write_step(&mut self) {
        self.rehash_step(REHASH_BUCKETS_PER_STEP);
    }

    #[inline(always)]
    pub fn maintain(&mut self) {
        self.rehash_step(1);
    }

    fn start_rehash(&mut self, new_bucket_count: usize) {
        let new_bucket_count = new_bucket_count.next_power_of_two();
        if new_bucket_count <= self.table.len() {
            return;
        }

        reserve_storage_for_bucket_growth(
            &mut self.metas,
            &mut self.keys,
            &mut self.values,
            new_bucket_count,
        );

        let new_table = Table::with_buckets(new_bucket_count);
        let old_table = mem::replace(&mut self.table, new_table);

        if old_table.len() <= SMALL_REHASH_THRESHOLD {
            let heads_ptr = self.table.heads.as_mut_ptr();
            let metas_ptr = self.metas.as_mut_ptr();
            for idx in 0..self.metas.len() {
                unsafe {
                    let meta = &mut *metas_ptr.add(idx);
                    let new_bucket = self.table.bucket(meta.hash);
                    meta.next = *heads_ptr.add(new_bucket);
                    *heads_ptr.add(new_bucket) = idx as u32;
                }
            }
        } else {
            self.old_table = Some(old_table);
            self.rehash_cursor = 0;
        }
    }

    pub(super) fn patch_swapped(&mut self, old_idx: u32, new_idx: u32) {
        if old_idx == new_idx {
            return;
        }

        let hash = self.metas[new_idx as usize].hash;
        if Self::patch_swapped_in_table_impl(
            &mut self.table,
            &mut self.metas,
            hash,
            old_idx,
            new_idx,
        ) {
            return;
        }
        if let Some(old_table) = self.old_table.as_mut() {
            let _ = Self::patch_swapped_in_table_impl(
                old_table,
                &mut self.metas,
                hash,
                old_idx,
                new_idx,
            );
        }
    }

    fn patch_swapped_in_table_impl(
        table: &mut Table,
        metas: &mut [NodeMeta],
        hash: u32,
        old_idx: u32,
        new_idx: u32,
    ) -> bool {
        unsafe {
            let bucket = table.bucket(hash);
            let heads_ptr = table.heads.as_mut_ptr();
            let metas_ptr = metas.as_mut_ptr();

            let mut cur = *heads_ptr.add(bucket);
            let mut prev = NIL;

            while cur != NIL {
                if cur == old_idx {
                    if prev == NIL {
                        *heads_ptr.add(bucket) = new_idx;
                    } else {
                        (*metas_ptr.add(prev as usize)).next = new_idx;
                    }
                    return true;
                }
                prev = cur;
                cur = (*metas_ptr.add(cur as usize)).next;
            }
        }

        false
    }
}

impl<K, V> Default for RehashingMap<K, V>
where
    K: Eq + AsRef<[u8]>,
{
    fn default() -> Self {
        Self::new()
    }
}

fn random_seed() -> u64 {
    let mut buf = [0u8; 8];
    if getrandom::fill(&mut buf).is_ok() {
        return u64::from_ne_bytes(buf);
    }

    static FALLBACK_SEED: AtomicU64 = AtomicU64::new(0x9e37_79b9_7f4a_7c15);
    let sequence = FALLBACK_SEED.fetch_add(0xa076_1d64_78bd_642f, Ordering::Relaxed);
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos() as u64)
        .unwrap_or(sequence.rotate_left(17));
    now ^ sequence.rotate_left(23)
}

#[inline(always)]
fn bucket_count_for_entries(entry_capacity: usize) -> usize {
    entry_capacity
        .max(1)
        .div_ceil(MAX_LOAD_FACTOR)
        .next_power_of_two()
        .max(INITIAL_BUCKETS)
}

fn reserve_storage_for_bucket_growth<K, V>(
    metas: &mut Vec<NodeMeta>,
    keys: &mut Vec<K>,
    values: &mut Vec<V>,
    new_bucket_count: usize,
) {
    let target_capacity = new_bucket_count.saturating_mul(MAX_LOAD_FACTOR);
    reserve_vec_to_capacity(metas, target_capacity);
    reserve_vec_to_capacity(keys, target_capacity);
    reserve_vec_to_capacity(values, target_capacity);
}

#[inline(always)]
fn reserve_vec_to_capacity<T>(vec: &mut Vec<T>, target_capacity: usize) {
    if vec.capacity() >= target_capacity {
        return;
    }

    let additional = target_capacity.saturating_sub(vec.len());
    vec.reserve_exact(additional);
}
