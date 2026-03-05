use crate::store::Store;
use types::value::CompactKey;

use super::super::helpers::{is_expired, monotonic_now_ms};
use super::super::pattern::{CompiledPattern, wildcard_match};
use super::{get_zset, sorted_by_score_refs};

impl Store {
    pub fn zscan(
        &self,
        key: &[u8],
        cursor: u64,
        pattern: Option<&[u8]>,
        count: usize,
    ) -> Result<(u64, Vec<(CompactKey, f64)>), ()> {
        let _trace = profiler::scope("engine::zset::scan::zscan");
        let idx = self.shard_index(key);
        let shard = self.shards[idx].read();
        let now_ms = monotonic_now_ms();
        if is_expired(&shard, key, now_ms) {
            return Ok((0, Vec::new()));
        }

        let Some(entry) = shard.entries.get(key) else {
            return Ok((0, Vec::new()));
        };
        let zset = get_zset(entry).ok_or(())?;
        let ordered = sorted_by_score_refs(zset, false);
        let pattern = CompiledPattern::new(pattern);
        if ordered.is_empty() {
            return Ok((0, Vec::new()));
        }

        let mut index = usize::try_from(cursor)
            .unwrap_or(usize::MAX)
            .min(ordered.len());
        let target = count.max(1);
        let mut out = Vec::with_capacity(target);
        while index < ordered.len() && out.len() < target {
            let item = &ordered[index];
            let member_bytes = item.0.as_slice();
            let pattern_matches = match &pattern {
                CompiledPattern::Any => true,
                CompiledPattern::Exact(pattern) => member_bytes == *pattern,
                CompiledPattern::Prefix(prefix) => member_bytes.starts_with(prefix),
                CompiledPattern::Suffix(suffix) => member_bytes.ends_with(suffix),
                CompiledPattern::Contains(needle) => {
                    needle.is_empty()
                        || member_bytes
                            .windows(needle.len())
                            .any(|window| window == *needle)
                }
                CompiledPattern::PrefixSuffix { prefix, suffix } => {
                    member_bytes.len() >= prefix.len() + suffix.len()
                        && member_bytes.starts_with(prefix)
                        && member_bytes.ends_with(suffix)
                }
                CompiledPattern::Wildcard(pattern) => wildcard_match(pattern, member_bytes),
            };
            if pattern_matches {
                out.push((item.0.clone(), item.1));
            }
            index += 1;
        }

        let next = if index >= ordered.len() {
            0
        } else {
            index as u64
        };
        Ok((next, out))
    }
}
