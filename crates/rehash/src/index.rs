use std::hash::Hash;

use ahash::RandomState;

#[inline(always)]
pub(super) fn hash_key<Q: Hash + ?Sized>(hash_builder: &RandomState, key: &Q) -> u64 {
    let _trace = profiler::scope("rehash::index::hash_key");
    hash_builder.hash_one(key)
}

#[inline(always)]
pub(super) fn bucket_index_from_hash(hash: u64, buckets: usize) -> usize {
    let _trace = profiler::scope("rehash::index::bucket_index_from_hash");
    (hash as usize) % buckets
}
