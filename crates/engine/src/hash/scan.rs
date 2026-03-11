use crate::store::Store;
use types::value::{CompactKey, CompactValue};

use super::super::helpers::{is_expired, monotonic_now_ms};
use super::super::pattern::{CompiledPattern, wildcard_match};
use super::get_hash_map;

impl Store {
    pub fn hscan(
        &self,
        key: &[u8],
        cursor: u64,
        pattern: Option<&[u8]>,
        count: usize,
    ) -> Result<(u64, Vec<(CompactKey, CompactValue)>), ()> {
        let idx = self.shard_index(key);
        let shard = self.shards[idx].read();
        let now_ms = monotonic_now_ms();
        if is_expired(&shard, key, now_ms) {
            return Ok((0, Vec::new()));
        }

        let Some(entry) = shard.entries.get(key) else {
            return Ok((0, Vec::new()));
        };
        let map = get_hash_map(entry).ok_or(())?;
        let pattern = CompiledPattern::new(pattern);

        if map.is_empty() {
            return Ok((0, Vec::new()));
        }

        let total_len = map.len();
        let mut index = usize::try_from(cursor).unwrap_or(usize::MAX).min(total_len);
        let target = count.max(1);
        let mut out = Vec::with_capacity(target);
        let mut iter = map.iter().skip(index);
        while out.len() < target {
            let Some((field, value)) = iter.next() else {
                break;
            };
            let field_bytes = field.as_slice();
            let pattern_matches = match &pattern {
                CompiledPattern::Any => true,
                CompiledPattern::Exact(pattern) => field_bytes == *pattern,
                CompiledPattern::Prefix(prefix) => field_bytes.starts_with(prefix),
                CompiledPattern::Suffix(suffix) => field_bytes.ends_with(suffix),
                CompiledPattern::Contains(needle) => {
                    needle.is_empty()
                        || field_bytes
                            .windows(needle.len())
                            .any(|window| window == *needle)
                }
                CompiledPattern::PrefixSuffix { prefix, suffix } => {
                    field_bytes.len() >= prefix.len() + suffix.len()
                        && field_bytes.starts_with(prefix)
                        && field_bytes.ends_with(suffix)
                }
                CompiledPattern::Wildcard(pattern) => wildcard_match(pattern, field_bytes),
            };
            if pattern_matches {
                out.push((field.clone(), value.clone()));
            }
            index += 1;
        }

        let next_cursor = if index >= total_len { 0 } else { index as u64 };
        Ok((next_cursor, out))
    }
}
