use crate::engine::store::Store;
use crate::engine::value::CompactKey;

use super::super::helpers::{monotonic_now_ms, purge_if_expired};
use super::super::pattern::wildcard_match;
use super::{collect_members, get_set};

impl Store {
    pub fn sscan(
        &self,
        key: &[u8],
        cursor: u64,
        pattern: Option<&[u8]>,
        count: usize,
    ) -> Result<(u64, Vec<CompactKey>), ()> {
        let idx = self.shard_index(key);
        let mut shard = self.shards[idx].write();
        let now_ms = monotonic_now_ms();
        if purge_if_expired(&mut shard, key, now_ms) {
            return Ok((0, Vec::new()));
        }

        let Some(entry) = shard.entries.get(key) else {
            return Ok((0, Vec::new()));
        };
        let set = get_set(entry).ok_or(())?;

        let members = collect_members(set);
        if members.is_empty() {
            return Ok((0, Vec::new()));
        }

        let mut index = usize::try_from(cursor)
            .unwrap_or(usize::MAX)
            .min(members.len());
        let target = count.max(1);
        let mut out = Vec::with_capacity(target);
        while index < members.len() && out.len() < target {
            let member = &members[index];
            if pattern.is_none_or(|matcher| wildcard_match(matcher, member.as_slice())) {
                out.push(member.clone());
            }
            index += 1;
        }

        let next = if index >= members.len() {
            0
        } else {
            index as u64
        };
        Ok((next, out))
    }
}
