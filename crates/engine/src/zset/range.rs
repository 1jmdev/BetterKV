use crate::store::Store;
use types::value::CompactKey;

use super::super::helpers::{is_expired, monotonic_now_ms, purge_if_expired};
use super::{get_zset, get_zset_mut, normalize_range, sorted_by_score};

impl Store {
    pub fn zrange(
        &self,
        key: &[u8],
        start: i64,
        stop: i64,
        reverse: bool,
    ) -> Result<Vec<(CompactKey, f64)>, ()> {
        let idx = self.shard_index(key);
        let shard = self.shards[idx].read();
        let now_ms = monotonic_now_ms();
        if is_expired(&shard, key, now_ms) {
            return Ok(Vec::new());
        }

        let Some(entry) = shard.entries.get(key) else {
            return Ok(Vec::new());
        };
        let zset = get_zset(entry).ok_or(())?;
        let ordered = sorted_by_score(zset, reverse);

        let Some((from, to_exclusive)) = normalize_range(start, stop, ordered.len()) else {
            return Ok(Vec::new());
        };
        Ok(ordered[from..to_exclusive].to_vec())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn zrange_by_score(
        &self,
        key: &[u8],
        min: f64,
        min_exclusive: bool,
        max: f64,
        max_exclusive: bool,
        reverse: bool,
        offset: usize,
        count: Option<usize>,
    ) -> Result<Vec<(CompactKey, f64)>, ()> {
        let idx = self.shard_index(key);
        let shard = self.shards[idx].read();
        let now_ms = monotonic_now_ms();
        if is_expired(&shard, key, now_ms) {
            return Ok(Vec::new());
        }

        let Some(entry) = shard.entries.get(key) else {
            return Ok(Vec::new());
        };
        let zset = get_zset(entry).ok_or(())?;

        let filtered: Vec<_> = zset
            .iter_ordered(reverse)
            .filter(|(_, score)| {
                let above_min = if min_exclusive {
                    *score > min
                } else {
                    *score >= min
                };
                let below_max = if max_exclusive {
                    *score < max
                } else {
                    *score <= max
                };
                above_min && below_max
            })
            .map(|(member, score)| (member.clone(), score))
            .collect();

        if offset >= filtered.len() {
            return Ok(Vec::new());
        }

        let mut sliced = filtered.into_iter().skip(offset);
        let out = if let Some(limit) = count {
            sliced.by_ref().take(limit).collect()
        } else {
            sliced.collect()
        };
        Ok(out)
    }

    pub fn zrange_by_lex(
        &self,
        key: &[u8],
        min: LexBound<'_>,
        max: LexBound<'_>,
        reverse: bool,
        offset: usize,
        count: Option<usize>,
    ) -> Result<Vec<CompactKey>, ()> {
        let idx = self.shard_index(key);
        let shard = self.shards[idx].read();
        let now_ms = monotonic_now_ms();
        if is_expired(&shard, key, now_ms) {
            return Ok(Vec::new());
        }

        let Some(entry) = shard.entries.get(key) else {
            return Ok(Vec::new());
        };
        let zset = get_zset(entry).ok_or(())?;

        let mut members: Vec<_> = zset
            .iter_member_scores()
            .map(|(member, _)| member.clone())
            .collect();
        members.sort_by(|left, right| left.as_slice().cmp(right.as_slice()));
        if reverse {
            members.reverse();
        }

        let filtered: Vec<_> = members
            .into_iter()
            .filter(|member| in_lex_range(member.as_slice(), min, max))
            .collect();

        if offset >= filtered.len() {
            return Ok(Vec::new());
        }

        let mut sliced = filtered.into_iter().skip(offset);
        Ok(if let Some(limit) = count {
            sliced.by_ref().take(limit).collect()
        } else {
            sliced.collect()
        })
    }

    pub fn zlexcount(&self, key: &[u8], min: LexBound<'_>, max: LexBound<'_>) -> Result<i64, ()> {
        Ok(self.zrange_by_lex(key, min, max, false, 0, None)?.len() as i64)
    }

    pub fn zremrangebylex(
        &self,
        key: &[u8],
        min: LexBound<'_>,
        max: LexBound<'_>,
    ) -> Result<i64, ()> {
        let idx = self.shard_index(key);
        let mut shard = self.shards[idx].write();
        let now_ms = monotonic_now_ms();
        if purge_if_expired(&mut shard, key, now_ms) {
            return Ok(0);
        }

        let Some(entry) = shard.entries.get_mut(key) else {
            return Ok(0);
        };
        let zset = get_zset_mut(entry).ok_or(())?;
        let members: Vec<_> = zset
            .iter_member_scores()
            .map(|(member, _)| member.clone())
            .filter(|member| in_lex_range(member.as_slice(), min, max))
            .collect();

        for member in &members {
            let _ = zset.remove(member.as_slice());
        }
        if zset.is_empty() {
            let _ = shard.remove_key(key);
        }
        Ok(members.len() as i64)
    }
}

#[derive(Clone, Copy)]
pub struct LexBound<'a> {
    pub value: Option<&'a [u8]>,
    pub inclusive: bool,
}

fn in_lex_range(member: &[u8], min: LexBound<'_>, max: LexBound<'_>) -> bool {
    let above_min = match min.value {
        None => true,
        Some(value) if min.inclusive => member >= value,
        Some(value) => member > value,
    };
    let below_max = match max.value {
        None => true,
        Some(value) if max.inclusive => member <= value,
        Some(value) => member < value,
    };
    above_min && below_max
}
