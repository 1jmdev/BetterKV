use std::time::{SystemTime, UNIX_EPOCH};

use super::StoreMap;
use crate::engine::value::Entry;

pub(super) fn purge_if_expired(shard: &mut StoreMap, key: &[u8]) -> bool {
    let expired = shard.get(key).map(Entry::is_expired).unwrap_or(false);
    if expired {
        shard.remove(key);
    }
    expired
}

pub(super) fn unix_time_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|value| value.as_millis() as u64)
        .unwrap_or(0)
}
