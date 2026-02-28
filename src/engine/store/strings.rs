use std::time::{Duration, Instant};

use crate::engine::value::Entry;

use super::helpers::purge_if_expired;
use super::Store;

impl Store {
    pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        let idx = self.shard_index(key);
        let mut shard = self.shards[idx].write();
        if purge_if_expired(&mut shard, key) {
            return None;
        }
        shard.get(key).map(|entry| entry.value.clone())
    }

    pub fn set(&self, key: Vec<u8>, value: Vec<u8>, ttl: Option<Duration>) {
        let idx = self.shard_index(&key);
        let entry = Entry {
            value,
            expires_at: ttl.map(|timeout| Instant::now() + timeout),
        };
        self.shards[idx].write().insert(key, entry);
    }

    pub fn setnx(&self, key: Vec<u8>, value: Vec<u8>, ttl: Option<Duration>) -> bool {
        let idx = self.shard_index(&key);
        let mut shard = self.shards[idx].write();
        if !purge_if_expired(&mut shard, &key) && shard.contains_key(&key) {
            return false;
        }

        shard.insert(
            key,
            Entry {
                value,
                expires_at: ttl.map(|timeout| Instant::now() + timeout),
            },
        );
        true
    }

    pub fn setxx(&self, key: Vec<u8>, value: Vec<u8>, ttl: Option<Duration>) -> bool {
        let idx = self.shard_index(&key);
        let mut shard = self.shards[idx].write();
        if purge_if_expired(&mut shard, &key) || !shard.contains_key(&key) {
            return false;
        }

        shard.insert(
            key,
            Entry {
                value,
                expires_at: ttl.map(|timeout| Instant::now() + timeout),
            },
        );
        true
    }

    pub fn getset(&self, key: Vec<u8>, value: Vec<u8>) -> Option<Vec<u8>> {
        let idx = self.shard_index(&key);
        let mut shard = self.shards[idx].write();
        let old_value = if purge_if_expired(&mut shard, &key) {
            None
        } else {
            shard.get(&key).map(|entry| entry.value.clone())
        };

        shard.insert(
            key,
            Entry {
                value,
                expires_at: None,
            },
        );

        old_value
    }

    pub fn getdel(&self, key: &[u8]) -> Option<Vec<u8>> {
        let idx = self.shard_index(key);
        let mut shard = self.shards[idx].write();
        if purge_if_expired(&mut shard, key) {
            return None;
        }

        shard.remove(key).map(|entry| entry.value)
    }

    pub fn append(&self, key: Vec<u8>, suffix: &[u8]) -> usize {
        let idx = self.shard_index(&key);
        let mut shard = self.shards[idx].write();
        if purge_if_expired(&mut shard, &key) {
            shard.insert(
                key,
                Entry {
                    value: suffix.to_vec(),
                    expires_at: None,
                },
            );
            return suffix.len();
        }

        match shard.get_mut(&key) {
            Some(entry) => {
                entry.value.extend_from_slice(suffix);
                entry.value.len()
            }
            None => {
                shard.insert(
                    key,
                    Entry {
                        value: suffix.to_vec(),
                        expires_at: None,
                    },
                );
                suffix.len()
            }
        }
    }

    pub fn strlen(&self, key: &[u8]) -> usize {
        self.get(key).map_or(0, |value| value.len())
    }

    pub fn incr(&self, key: &[u8]) -> Result<i64, ()> {
        self.incr_by(key, 1)
    }

    pub fn incr_by(&self, key: &[u8], delta: i64) -> Result<i64, ()> {
        let idx = self.shard_index(key);
        let mut shard = self.shards[idx].write();

        let current = if purge_if_expired(&mut shard, key) {
            0
        } else {
            match shard.get(key) {
                Some(entry) => {
                    let text = std::str::from_utf8(&entry.value).map_err(|_| ())?;
                    text.parse::<i64>().map_err(|_| ())?
                }
                None => 0,
            }
        };

        let next_value = current.checked_add(delta).ok_or(())?;
        shard.insert(
            key.to_vec(),
            Entry {
                value: next_value.to_string().into_bytes(),
                expires_at: None,
            },
        );

        Ok(next_value)
    }

    pub fn mget(&self, keys: &[Vec<u8>]) -> Vec<Option<Vec<u8>>> {
        keys.iter().map(|key| self.get(key)).collect()
    }

    pub fn mset(&self, pairs: &[(Vec<u8>, Vec<u8>)]) {
        for (key, value) in pairs {
            self.set(key.clone(), value.clone(), None);
        }
    }

    pub fn msetnx(&self, pairs: &[(Vec<u8>, Vec<u8>)]) -> bool {
        for (key, _) in pairs {
            let idx = self.shard_index(key);
            let mut shard = self.shards[idx].write();
            if !purge_if_expired(&mut shard, key) && shard.contains_key(key) {
                return false;
            }
        }

        for (key, value) in pairs {
            self.set(key.clone(), value.clone(), None);
        }
        true
    }
}
