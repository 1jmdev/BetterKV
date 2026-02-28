use super::helpers::purge_if_expired;
use super::pattern::wildcard_match;
use super::Store;

impl Store {
    pub fn del(&self, keys: &[Vec<u8>]) -> i64 {
        let mut removed = 0;
        for key in keys {
            let idx = self.shard_index(key);
            if self.shards[idx].write().remove(key).is_some() {
                removed += 1;
            }
        }
        removed
    }

    pub fn exists(&self, keys: &[Vec<u8>]) -> i64 {
        let mut count = 0;
        for key in keys {
            let idx = self.shard_index(key);
            let mut shard = self.shards[idx].write();
            if !purge_if_expired(&mut shard, key) && shard.contains_key(key) {
                count += 1;
            }
        }
        count
    }

    pub fn touch(&self, keys: &[Vec<u8>]) -> i64 {
        self.exists(keys)
    }

    pub fn rename(&self, from: &[u8], to: Vec<u8>) -> bool {
        let from_idx = self.shard_index(from);
        let mut source = self.shards[from_idx].write();
        if purge_if_expired(&mut source, from) {
            return false;
        }

        let Some(entry) = source.remove(from) else {
            return false;
        };
        drop(source);

        let to_idx = self.shard_index(&to);
        self.shards[to_idx].write().insert(to, entry);
        true
    }

    pub fn renamenx(&self, from: &[u8], to: Vec<u8>) -> Result<i64, ()> {
        let from_idx = self.shard_index(from);
        let mut source = self.shards[from_idx].write();
        if purge_if_expired(&mut source, from) {
            return Err(());
        }

        let Some(entry) = source.get(from).cloned() else {
            return Err(());
        };
        drop(source);

        let to_idx = self.shard_index(&to);
        let mut destination = self.shards[to_idx].write();
        if !purge_if_expired(&mut destination, &to) && destination.contains_key(&to) {
            return Ok(0);
        }
        destination.insert(to, entry);
        drop(destination);

        self.shards[from_idx].write().remove(from);
        Ok(1)
    }

    pub fn key_type(&self, key: &[u8]) -> &'static str {
        if self.get(key).is_some() {
            "string"
        } else {
            "none"
        }
    }

    pub fn dbsize(&self) -> i64 {
        self.sweep_expired();
        let mut total = 0;
        for shard in self.shards.iter() {
            total += shard.read().len() as i64;
        }
        total
    }

    pub fn keys(&self, pattern: &[u8]) -> Vec<Vec<u8>> {
        let mut out = Vec::new();
        for shard in self.shards.iter() {
            let mut guard = shard.write();
            guard.retain(|_, entry| !entry.is_expired());
            for key in guard.keys() {
                if wildcard_match(pattern, key) {
                    out.push(key.clone());
                }
            }
        }
        out
    }

    pub fn flushdb(&self) -> i64 {
        let mut removed = 0;
        for shard in self.shards.iter() {
            let mut guard = shard.write();
            removed += guard.len() as i64;
            guard.clear();
        }
        removed
    }
}
