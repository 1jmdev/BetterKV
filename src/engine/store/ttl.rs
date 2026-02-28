use std::time::{Duration, Instant};

use super::helpers::{purge_if_expired, unix_time_ms};
use super::Store;

impl Store {
    pub fn expire(&self, key: &[u8], seconds: u64) -> i64 {
        self.pexpire(key, seconds.saturating_mul(1000))
    }

    pub fn pexpire(&self, key: &[u8], milliseconds: u64) -> i64 {
        let idx = self.shard_index(key);
        let mut shard = self.shards[idx].write();
        if purge_if_expired(&mut shard, key) {
            return 0;
        }

        if let Some(entry) = shard.get_mut(key) {
            entry.expires_at = Some(Instant::now() + Duration::from_millis(milliseconds));
            return 1;
        }

        0
    }

    pub fn expire_at(&self, key: &[u8], timestamp_sec: u64) -> i64 {
        self.pexpire_at(key, timestamp_sec.saturating_mul(1000))
    }

    pub fn pexpire_at(&self, key: &[u8], timestamp_ms: u64) -> i64 {
        let now_ms = unix_time_ms();
        if timestamp_ms <= now_ms {
            return self.del(&[key.to_vec()]);
        }
        self.pexpire(key, timestamp_ms - now_ms)
    }

    pub fn persist(&self, key: &[u8]) -> i64 {
        let idx = self.shard_index(key);
        let mut shard = self.shards[idx].write();
        if purge_if_expired(&mut shard, key) {
            return 0;
        }

        match shard.get_mut(key) {
            Some(entry) => {
                if entry.expires_at.take().is_some() {
                    1
                } else {
                    0
                }
            }
            None => 0,
        }
    }

    pub fn ttl(&self, key: &[u8]) -> i64 {
        let pttl = self.pttl(key);
        if pttl < 0 {
            pttl
        } else {
            pttl / 1000
        }
    }

    pub fn pttl(&self, key: &[u8]) -> i64 {
        let idx = self.shard_index(key);
        let mut shard = self.shards[idx].write();
        if purge_if_expired(&mut shard, key) {
            return -2;
        }

        match shard.get(key) {
            Some(entry) => match entry.expires_at {
                Some(deadline) => deadline
                    .saturating_duration_since(Instant::now())
                    .as_millis() as i64,
                None => -1,
            },
            None => -2,
        }
    }
}
