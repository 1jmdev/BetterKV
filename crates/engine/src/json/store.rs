use std::time::Duration;

use serde_json::{Map, Value as JsonValue};

use crate::helpers::{deadline_from_ttl, get_live_entry, monotonic_now_ms, purge_if_expired};
use crate::Store;

use super::{
    clamp_insert_index, clear_value, delete_exact, ensure_path_mut, get_matches, get_mut_exact,
    json_debug_memory, json_entry, json_entry_mut, json_len_bytes, merge_value, normalize_bounds,
    normalize_index, number_from_f64, to_f64, value_type, write_json_entry, JsonError, JsonPath,
    JsonSetMode, JsonSetResult, JsonType,
};

impl Store {
    pub fn json_get(
        &self,
        key: &[u8],
        path: &JsonPath,
    ) -> Result<Option<Vec<JsonValue>>, JsonError> {
        let idx = self.shard_index(key);
        let shard = self.shards[idx].read();
        let now_ms = monotonic_now_ms();
        let Some(entry) = get_live_entry(&shard, key, now_ms) else {
            return Ok(None);
        };
        let value = json_entry(&entry.entry)?;
        Ok(Some(
            get_matches(value, path).into_iter().cloned().collect(),
        ))
    }

    pub fn json_set(
        &self,
        key: &[u8],
        path: &JsonPath,
        value: JsonValue,
        mode: JsonSetMode,
        ttl: Option<Duration>,
    ) -> Result<JsonSetResult, JsonError> {
        let idx = self.shard_index(key);
        let mut shard = self.shards[idx].write();
        let now_ms = monotonic_now_ms();
        let expired = purge_if_expired(&mut shard, key, now_ms);
        let ttl_deadline = ttl
            .map(deadline_from_ttl)
            .or_else(|| shard.ttl_deadline(key));

        if path.tokens.is_empty() {
            if mode == JsonSetMode::Xx && (expired || !shard.entries.contains_key(key)) {
                return Ok(JsonSetResult {
                    applied: false,
                    ttl_preserved: false,
                });
            }
            if mode == JsonSetMode::Nx && !expired && shard.entries.contains_key(key) {
                return Ok(JsonSetResult {
                    applied: false,
                    ttl_preserved: false,
                });
            }
            write_json_entry(&mut shard, key, value, ttl_deadline);
            return Ok(JsonSetResult {
                applied: true,
                ttl_preserved: ttl.is_none(),
            });
        }

        let Some(stored) = shard.entries.get_mut::<[u8]>(key) else {
            if mode == JsonSetMode::Xx {
                return Ok(JsonSetResult {
                    applied: false,
                    ttl_preserved: false,
                });
            }
            let mut root = JsonValue::Object(Map::new());
            let target = ensure_path_mut(&mut root, &path.tokens)?;
            *target = value;
            write_json_entry(&mut shard, key, root, ttl_deadline);
            return Ok(JsonSetResult {
                applied: true,
                ttl_preserved: ttl.is_none(),
            });
        };

        let root = json_entry_mut(&mut stored.entry)?;
        match mode {
            JsonSetMode::Any => {
                let target = ensure_path_mut(root, &path.tokens)?;
                *target = value;
                Ok(JsonSetResult {
                    applied: true,
                    ttl_preserved: ttl.is_none(),
                })
            }
            JsonSetMode::Nx => {
                if get_mut_exact(root, &path.tokens)?.is_some() {
                    return Ok(JsonSetResult {
                        applied: false,
                        ttl_preserved: ttl.is_none(),
                    });
                }
                let target = ensure_path_mut(root, &path.tokens)?;
                *target = value;
                Ok(JsonSetResult {
                    applied: true,
                    ttl_preserved: ttl.is_none(),
                })
            }
            JsonSetMode::Xx => {
                let Some(target) = get_mut_exact(root, &path.tokens)? else {
                    return Ok(JsonSetResult {
                        applied: false,
                        ttl_preserved: ttl.is_none(),
                    });
                };
                *target = value;
                Ok(JsonSetResult {
                    applied: true,
                    ttl_preserved: ttl.is_none(),
                })
            }
        }
    }

    pub fn json_del(&self, key: &[u8], paths: &[JsonPath]) -> Result<i64, JsonError> {
        let idx = self.shard_index(key);
        let mut shard = self.shards[idx].write();
        let now_ms = monotonic_now_ms();
        if purge_if_expired(&mut shard, key, now_ms) {
            return Ok(0);
        }
        let Some(stored) = shard.entries.get_mut::<[u8]>(key) else {
            return Ok(0);
        };
        let root = json_entry_mut(&mut stored.entry)?;
        let mut removed = 0i64;
        if paths.is_empty() {
            if shard.remove_key(key).is_some() {
                return Ok(1);
            }
            return Ok(0);
        }
        for path in paths {
            if path.tokens.is_empty() {
                removed += i64::from(shard.remove_key(key).is_some());
                return Ok(removed);
            }
            removed += i64::from(delete_exact(root, &path.tokens)?);
        }
        Ok(removed)
    }

    pub fn json_clear(&self, key: &[u8], path: &JsonPath) -> Result<i64, JsonError> {
        let idx = self.shard_index(key);
        let mut shard = self.shards[idx].write();
        let now_ms = monotonic_now_ms();
        if purge_if_expired(&mut shard, key, now_ms) {
            return Ok(0);
        }
        let Some(stored) = shard.entries.get_mut::<[u8]>(key) else {
            return Ok(0);
        };
        let root = json_entry_mut(&mut stored.entry)?;
        let mut changed = 0i64;
        if path.tokens.is_empty() {
            return Ok(i64::from(clear_value(root)));
        }
        if let Some(target) = get_mut_exact(root, &path.tokens)? {
            changed += i64::from(clear_value(target));
        }
        Ok(changed)
    }

    pub fn json_merge(
        &self,
        key: &[u8],
        path: &JsonPath,
        patch: JsonValue,
    ) -> Result<bool, JsonError> {
        let idx = self.shard_index(key);
        let mut shard = self.shards[idx].write();
        let now_ms = monotonic_now_ms();
        if purge_if_expired(&mut shard, key, now_ms) {
            return Err(JsonError::KeyMissing);
        }
        let Some(stored) = shard.entries.get_mut::<[u8]>(key) else {
            return Err(JsonError::KeyMissing);
        };
        let root = json_entry_mut(&mut stored.entry)?;
        let Some(target) = get_mut_exact(root, &path.tokens)? else {
            return Ok(false);
        };
        merge_value(target, patch);
        Ok(true)
    }

    pub fn json_arrappend(
        &self,
        key: &[u8],
        path: &JsonPath,
        values: Vec<JsonValue>,
    ) -> Result<Option<usize>, JsonError> {
        let idx = self.shard_index(key);
        let mut shard = self.shards[idx].write();
        let now_ms = monotonic_now_ms();
        if purge_if_expired(&mut shard, key, now_ms) {
            return Ok(None);
        }
        let Some(stored) = shard.entries.get_mut::<[u8]>(key) else {
            return Ok(None);
        };
        let root = json_entry_mut(&mut stored.entry)?;
        let Some(target) = get_mut_exact(root, &path.tokens)? else {
            return Ok(None);
        };
        let array = target.as_array_mut().ok_or(JsonError::Path)?;
        array.extend(values);
        Ok(Some(array.len()))
    }

    pub fn json_arrindex(
        &self,
        key: &[u8],
        path: &JsonPath,
        needle: &JsonValue,
        start: i64,
        stop: Option<i64>,
    ) -> Result<Option<i64>, JsonError> {
        let values = match self.json_get(key, path)? {
            Some(values) => values,
            None => return Ok(None),
        };
        let Some(JsonValue::Array(array)) = values.into_iter().next() else {
            return Ok(None);
        };
        let len = array.len();
        if len == 0 {
            return Ok(Some(-1));
        }
        let len_i64 = i64::try_from(len).unwrap_or(i64::MAX);
        let mut from = if start < 0 { len_i64 + start } else { start };
        let mut to = stop.unwrap_or(len_i64 - 1);
        if to < 0 {
            to += len_i64;
        }
        if from < 0 {
            from = 0;
        }
        if to >= len_i64 {
            to = len_i64 - 1;
        }
        if from > to || from >= len_i64 {
            return Ok(Some(-1));
        }
        let from_usize = usize::try_from(from).map_err(|_| JsonError::Path)?;
        let to_usize = usize::try_from(to).map_err(|_| JsonError::Path)?;
        for (index, value) in array
            .iter()
            .enumerate()
            .skip(from_usize)
            .take(to_usize - from_usize + 1)
        {
            if value == needle {
                return Ok(Some(i64::try_from(index).unwrap_or(i64::MAX)));
            }
        }
        Ok(Some(-1))
    }

    pub fn json_arrinsert(
        &self,
        key: &[u8],
        path: &JsonPath,
        index: i64,
        values: Vec<JsonValue>,
    ) -> Result<Option<usize>, JsonError> {
        let idx = self.shard_index(key);
        let mut shard = self.shards[idx].write();
        let now_ms = monotonic_now_ms();
        if purge_if_expired(&mut shard, key, now_ms) {
            return Ok(None);
        }
        let Some(stored) = shard.entries.get_mut::<[u8]>(key) else {
            return Ok(None);
        };
        let root = json_entry_mut(&mut stored.entry)?;
        let Some(target) = get_mut_exact(root, &path.tokens)? else {
            return Ok(None);
        };
        let array = target.as_array_mut().ok_or(JsonError::Path)?;
        let insert_at = clamp_insert_index(array.len(), index);
        for (offset, value) in values.into_iter().enumerate() {
            array.insert(insert_at + offset, value);
        }
        Ok(Some(array.len()))
    }

    pub fn json_arrlen(&self, key: &[u8], path: &JsonPath) -> Result<Option<usize>, JsonError> {
        let values = match self.json_get(key, path)? {
            Some(values) => values,
            None => return Ok(None),
        };
        let Some(JsonValue::Array(array)) = values.into_iter().next() else {
            return Ok(None);
        };
        Ok(Some(array.len()))
    }

    pub fn json_arrpop(
        &self,
        key: &[u8],
        path: &JsonPath,
        index: i64,
    ) -> Result<Option<JsonValue>, JsonError> {
        let idx = self.shard_index(key);
        let mut shard = self.shards[idx].write();
        let now_ms = monotonic_now_ms();
        if purge_if_expired(&mut shard, key, now_ms) {
            return Ok(None);
        }
        let Some(stored) = shard.entries.get_mut::<[u8]>(key) else {
            return Ok(None);
        };
        let root = json_entry_mut(&mut stored.entry)?;
        let Some(target) = get_mut_exact(root, &path.tokens)? else {
            return Ok(None);
        };
        let array = target.as_array_mut().ok_or(JsonError::Path)?;
        if array.is_empty() {
            return Ok(None);
        }
        let normalized = if index == -1 {
            array.len() - 1
        } else {
            normalize_index(array.len(), index).ok_or(JsonError::Path)?
        };
        Ok(Some(array.remove(normalized)))
    }

    pub fn json_arrtrim(
        &self,
        key: &[u8],
        path: &JsonPath,
        start: i64,
        stop: i64,
    ) -> Result<Option<usize>, JsonError> {
        let idx = self.shard_index(key);
        let mut shard = self.shards[idx].write();
        let now_ms = monotonic_now_ms();
        if purge_if_expired(&mut shard, key, now_ms) {
            return Ok(None);
        }
        let Some(stored) = shard.entries.get_mut::<[u8]>(key) else {
            return Ok(None);
        };
        let root = json_entry_mut(&mut stored.entry)?;
        let Some(target) = get_mut_exact(root, &path.tokens)? else {
            return Ok(None);
        };
        let array = target.as_array_mut().ok_or(JsonError::Path)?;
        let Some((from, to)) = normalize_bounds(array.len(), start, stop) else {
            array.clear();
            return Ok(Some(0));
        };
        let replacement: Vec<JsonValue> = array.drain(from..=to).collect();
        *array = replacement;
        Ok(Some(array.len()))
    }

    pub fn json_objkeys(
        &self,
        key: &[u8],
        path: &JsonPath,
    ) -> Result<Option<Vec<String>>, JsonError> {
        let values = match self.json_get(key, path)? {
            Some(values) => values,
            None => return Ok(None),
        };
        let Some(JsonValue::Object(object)) = values.into_iter().next() else {
            return Ok(None);
        };
        Ok(Some(object.keys().cloned().collect()))
    }

    pub fn json_objlen(&self, key: &[u8], path: &JsonPath) -> Result<Option<usize>, JsonError> {
        let values = match self.json_get(key, path)? {
            Some(values) => values,
            None => return Ok(None),
        };
        let Some(JsonValue::Object(object)) = values.into_iter().next() else {
            return Ok(None);
        };
        Ok(Some(object.len()))
    }

    pub fn json_strlen(&self, key: &[u8], path: &JsonPath) -> Result<Option<usize>, JsonError> {
        let values = match self.json_get(key, path)? {
            Some(values) => values,
            None => return Ok(None),
        };
        let Some(JsonValue::String(value)) = values.into_iter().next() else {
            return Ok(None);
        };
        Ok(Some(value.chars().count()))
    }

    pub fn json_strappend(
        &self,
        key: &[u8],
        path: &JsonPath,
        suffixes: &[String],
    ) -> Result<Option<usize>, JsonError> {
        let idx = self.shard_index(key);
        let mut shard = self.shards[idx].write();
        let now_ms = monotonic_now_ms();
        if purge_if_expired(&mut shard, key, now_ms) {
            return Ok(None);
        }
        let Some(stored) = shard.entries.get_mut::<[u8]>(key) else {
            return Ok(None);
        };
        let root = json_entry_mut(&mut stored.entry)?;
        let Some(target) = get_mut_exact(root, &path.tokens)? else {
            return Ok(None);
        };
        let text = target.as_str().ok_or(JsonError::Path)?;
        let mut output =
            String::with_capacity(text.len() + suffixes.iter().map(String::len).sum::<usize>());
        output.push_str(text);
        for suffix in suffixes {
            output.push_str(suffix);
        }
        let len = output.chars().count();
        *target = JsonValue::String(output);
        Ok(Some(len))
    }

    pub fn json_numincrby(
        &self,
        key: &[u8],
        path: &JsonPath,
        delta: f64,
    ) -> Result<Option<JsonValue>, JsonError> {
        let idx = self.shard_index(key);
        let mut shard = self.shards[idx].write();
        let now_ms = monotonic_now_ms();
        if purge_if_expired(&mut shard, key, now_ms) {
            return Ok(None);
        }
        let Some(stored) = shard.entries.get_mut::<[u8]>(key) else {
            return Ok(None);
        };
        let root = json_entry_mut(&mut stored.entry)?;
        let Some(target) = get_mut_exact(root, &path.tokens)? else {
            return Ok(None);
        };
        let number = to_f64(target).ok_or(JsonError::Path)? + delta;
        let result = JsonValue::Number(number_from_f64(number)?);
        *target = result.clone();
        Ok(Some(result))
    }

    pub fn json_nummultby(
        &self,
        key: &[u8],
        path: &JsonPath,
        factor: f64,
    ) -> Result<Option<JsonValue>, JsonError> {
        let idx = self.shard_index(key);
        let mut shard = self.shards[idx].write();
        let now_ms = monotonic_now_ms();
        if purge_if_expired(&mut shard, key, now_ms) {
            return Ok(None);
        }
        let Some(stored) = shard.entries.get_mut::<[u8]>(key) else {
            return Ok(None);
        };
        let root = json_entry_mut(&mut stored.entry)?;
        let Some(target) = get_mut_exact(root, &path.tokens)? else {
            return Ok(None);
        };
        let number = to_f64(target).ok_or(JsonError::Path)? * factor;
        let result = JsonValue::Number(number_from_f64(number)?);
        *target = result.clone();
        Ok(Some(result))
    }

    pub fn json_toggle(&self, key: &[u8], path: &JsonPath) -> Result<Option<bool>, JsonError> {
        let idx = self.shard_index(key);
        let mut shard = self.shards[idx].write();
        let now_ms = monotonic_now_ms();
        if purge_if_expired(&mut shard, key, now_ms) {
            return Ok(None);
        }
        let Some(stored) = shard.entries.get_mut::<[u8]>(key) else {
            return Ok(None);
        };
        let root = json_entry_mut(&mut stored.entry)?;
        let Some(target) = get_mut_exact(root, &path.tokens)? else {
            return Ok(None);
        };
        let value = target.as_bool().ok_or(JsonError::Path)?;
        *target = JsonValue::Bool(!value);
        Ok(Some(!value))
    }

    pub fn json_type(&self, key: &[u8], path: &JsonPath) -> Result<Option<JsonType>, JsonError> {
        let values = match self.json_get(key, path)? {
            Some(values) => values,
            None => return Ok(None),
        };
        let Some(value) = values.into_iter().next() else {
            return Ok(None);
        };
        Ok(Some(value_type(&value)))
    }

    pub fn json_debug_memory(
        &self,
        key: &[u8],
        path: &JsonPath,
    ) -> Result<Option<usize>, JsonError> {
        let values = match self.json_get(key, path)? {
            Some(values) => values,
            None => return Ok(None),
        };
        let Some(value) = values.into_iter().next() else {
            return Ok(None);
        };
        Ok(Some(json_debug_memory(&value)))
    }

    pub fn json_mget(
        &self,
        keys: &[Vec<u8>],
        path: &JsonPath,
    ) -> Result<Vec<Option<JsonValue>>, JsonError> {
        let mut out = Vec::with_capacity(keys.len());
        for key in keys {
            let value = self
                .json_get(key, path)?
                .and_then(|mut values| values.drain(..).next());
            out.push(value);
        }
        Ok(out)
    }

    pub fn json_mset(&self, items: &[(Vec<u8>, JsonPath, JsonValue)]) -> Result<(), JsonError> {
        for (key, path, value) in items {
            self.json_set(key, path, value.clone(), JsonSetMode::Any, None)?;
        }
        Ok(())
    }

    pub fn json_resp(&self, key: &[u8], path: &JsonPath) -> Result<Option<JsonValue>, JsonError> {
        Ok(self
            .json_get(key, path)?
            .and_then(|mut values| values.drain(..).next()))
    }

    pub fn json_value_len_bytes(
        &self,
        key: &[u8],
        path: &JsonPath,
    ) -> Result<Option<usize>, JsonError> {
        let value = self.json_resp(key, path)?;
        Ok(value.as_ref().map(json_len_bytes))
    }
}
