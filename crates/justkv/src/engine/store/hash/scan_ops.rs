use crate::engine::store::Store;
use crate::engine::value::{CompactKey, CompactValue};

use super::super::helpers::{monotonic_now_ms, purge_if_expired};
use super::super::pattern::wildcard_match;
use super::{collect_pairs, get_hash_map};

impl Store {
    pub fn hscan(
        &self,
        key: &[u8],
        _cursor: u64,
        pattern: Option<&[u8]>,
        count: usize,
    ) -> Result<(u64, Vec<(CompactKey, CompactValue)>), ()> {
        let idx = self.shard_index(key);
        let mut shard = self.shards[idx].write();
        let now_ms = monotonic_now_ms();
        if purge_if_expired(&mut shard, key, now_ms) {
            return Ok((0, Vec::new()));
        }

        let Some(entry) = shard.entries.get(key) else {
            return Ok((0, Vec::new()));
        };
        let map = get_hash_map(entry).ok_or(())?;

        let mut out = collect_pairs(map);
        if let Some(pattern) = pattern {
            out.retain(|(field, _)| wildcard_match(pattern, field.as_slice()));
        }
        if count > 0 && out.len() > count {
            out.truncate(count);
        }
        Ok((0, out))
    }
}
