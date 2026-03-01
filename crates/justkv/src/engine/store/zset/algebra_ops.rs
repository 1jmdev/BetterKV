use ahash::RandomState;
use hashbrown::HashMap;

use crate::engine::store::Store;
use crate::engine::value::{CompactArg, CompactKey, ZSetValueMap};

use super::super::helpers::{is_expired, monotonic_now_ms};
use super::{get_zset, sorted_by_score};

impl Store {
    pub fn zinter(&self, keys: &[CompactArg]) -> Result<Vec<(CompactKey, f64)>, ()> {
        let snapshots = self.zset_snapshots(keys)?;
        if snapshots.is_empty() || snapshots.iter().any(|set| set.is_empty()) {
            return Ok(Vec::new());
        }

        let mut out = HashMap::with_hasher(RandomState::new());
        let (first, rest) = snapshots.split_first().expect("checked non-empty");
        for (member, score) in first {
            if rest.iter().all(|set| set.contains_key(member.as_slice())) {
                let total = rest.iter().fold(*score, |acc, set| {
                    acc + set.get(member.as_slice()).copied().unwrap_or(0.0)
                });
                out.insert(member.clone(), total);
            }
        }
        Ok(sorted_by_score(&out, false))
    }

    pub fn zunion(&self, keys: &[CompactArg]) -> Result<Vec<(CompactKey, f64)>, ()> {
        let snapshots = self.zset_snapshots(keys)?;
        let mut out = HashMap::with_hasher(RandomState::new());
        for set in snapshots {
            for (member, score) in set {
                let next = out.get(member.as_slice()).copied().unwrap_or(0.0) + score;
                out.insert(member, next);
            }
        }
        Ok(sorted_by_score(&out, false))
    }

    pub fn zdiff(&self, keys: &[CompactArg]) -> Result<Vec<(CompactKey, f64)>, ()> {
        let snapshots = self.zset_snapshots(keys)?;
        let Some((first, rest)) = snapshots.split_first() else {
            return Ok(Vec::new());
        };

        let mut out = HashMap::with_hasher(RandomState::new());
        for (member, score) in first {
            if rest.iter().all(|set| !set.contains_key(member.as_slice())) {
                out.insert(member.clone(), *score);
            }
        }
        Ok(sorted_by_score(&out, false))
    }

    fn zset_snapshots(&self, keys: &[CompactArg]) -> Result<Vec<ZSetValueMap>, ()> {
        let mut snapshots = Vec::with_capacity(keys.len());
        let now_ms = monotonic_now_ms();
        for key in keys {
            let idx = self.shard_index(key.as_slice());
            let shard = self.shards[idx].read();
            if is_expired(&shard, key.as_slice(), now_ms) {
                snapshots.push(HashMap::with_hasher(RandomState::new()));
                continue;
            }

            match shard.entries.get(key.as_slice()) {
                None => snapshots.push(HashMap::with_hasher(RandomState::new())),
                Some(entry) => {
                    let zset = get_zset(entry).ok_or(())?;
                    snapshots.push(zset.clone());
                }
            }
        }
        Ok(snapshots)
    }
}
