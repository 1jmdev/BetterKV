use std::time::Duration;

use crate::store::Store;
use types::value::{CompactValue, Entry};

use super::super::helpers::{
    deadline_from_ttl, get_live_entry, monotonic_now_ms, purge_if_expired,
};
use super::write_entry;

impl Store {
    pub fn get(&self, key: &[u8]) -> Result<Option<CompactValue>, ()> {
        let idx = self.shard_index(key);
        let shard = self.shards[idx].read();
        let entry = if shard.has_ttls() {
            let now_ms = monotonic_now_ms();
            get_live_entry(&shard, key, now_ms)
        } else {
            shard.entries.get(key)
        };
        let Some(entry) = entry else {
            return Ok(None);
        };
        match entry.as_string() {
            Some(value) => Ok(Some(value.clone())),
            None => Err(()),
        }
    }

    pub fn set(&self, key: &[u8], value: &[u8], ttl: Option<Duration>) {
        let idx = self.shard_index(key);
        let mut shard = self.shards[idx].write();
        write_entry(
            &mut shard,
            key,
            Entry::from_slice(value),
            ttl.map(deadline_from_ttl),
        );
    }

    pub fn setnx(&self, key: &[u8], value: &[u8], ttl: Option<Duration>) -> bool {
        let idx = self.shard_index(key);
        let mut shard = self.shards[idx].write();
        if !shard.has_ttls() {
            if shard.entries.contains_key(key) {
                return false;
            }
        } else {
            let now_ms = monotonic_now_ms();
            if !purge_if_expired(&mut shard, key, now_ms) && shard.entries.contains_key(key) {
                return false;
            }
        }

        write_entry(
            &mut shard,
            key,
            Entry::from_slice(value),
            ttl.map(deadline_from_ttl),
        );
        true
    }

    pub fn setxx(&self, key: &[u8], value: &[u8], ttl: Option<Duration>) -> bool {
        let idx = self.shard_index(key);
        let mut shard = self.shards[idx].write();
        if !shard.has_ttls() {
            if !shard.entries.contains_key(key) {
                return false;
            }
        } else {
            let now_ms = monotonic_now_ms();
            if purge_if_expired(&mut shard, key, now_ms) || !shard.entries.contains_key(key) {
                return false;
            }
        }

        write_entry(
            &mut shard,
            key,
            Entry::from_slice(value),
            ttl.map(deadline_from_ttl),
        );
        true
    }

    pub fn getset(&self, key: &[u8], value: &[u8]) -> Result<Option<Vec<u8>>, ()> {
        let idx = self.shard_index(key);
        let mut shard = self.shards[idx].write();
        if shard.has_ttls() {
            let now_ms = monotonic_now_ms();
            if purge_if_expired(&mut shard, key, now_ms) {
                write_entry(&mut shard, key, Entry::from_slice(value), None);
                return Ok(None);
            }
        }

        if let Some(entry) = shard.entries.get_mut::<[u8]>(key) {
            let Some(current) = entry.as_string() else {
                return Err(());
            };

            let old_value = current.to_vec();
            entry.entry = Entry::from_slice(value);
            let _ = shard.clear_ttl(key);
            return Ok(Some(old_value));
        }

        write_entry(&mut shard, key, Entry::from_slice(value), None);
        Ok(None)
    }

    pub fn getdel(&self, key: &[u8]) -> Result<Option<Vec<u8>>, ()> {
        let idx = self.shard_index(key);
        let mut shard = self.shards[idx].write();
        if shard.has_ttls() {
            let now_ms = monotonic_now_ms();
            if purge_if_expired(&mut shard, key, now_ms) {
                return Ok(None);
            }
        }

        match shard.remove_key(key) {
            Some(entry) => match entry.into_string() {
                Some(value) => Ok(Some(value.into_vec())),
                None => Err(()),
            },
            None => Ok(None),
        }
    }

    pub fn append(&self, key: &[u8], suffix: &[u8]) -> Result<usize, ()> {
        let idx = self.shard_index(key);
        let mut shard = self.shards[idx].write();

        let mut base = if !shard.has_ttls() {
            match shard.entries.get::<[u8]>(key) {
                Some(entry) => match entry.as_string() {
                    Some(value) => value.to_vec(),
                    None => return Err(()),
                },
                None => Vec::new(),
            }
        } else {
            let now_ms = monotonic_now_ms();
            if purge_if_expired(&mut shard, key, now_ms) {
                Vec::new()
            } else {
                match shard.entries.get::<[u8]>(key) {
                    Some(entry) => match entry.as_string() {
                        Some(value) => value.to_vec(),
                        None => return Err(()),
                    },
                    None => Vec::new(),
                }
            }
        };
        let ttl_deadline = shard.ttl_deadline(key);

        base.extend_from_slice(suffix);
        let size = base.len();
        write_entry(&mut shard, key, Entry::new(base), ttl_deadline);
        Ok(size)
    }

    pub fn strlen(&self, key: &[u8]) -> Result<usize, ()> {
        let idx = self.shard_index(key);
        let shard = self.shards[idx].read();
        let entry = if shard.has_ttls() {
            let now_ms = monotonic_now_ms();
            get_live_entry(&shard, key, now_ms)
        } else {
            shard.entries.get(key)
        };
        let Some(entry) = entry else {
            return Ok(0);
        };
        match entry.as_string() {
            Some(value) => Ok(value.len()),
            None => Err(()),
        }
    }
}
