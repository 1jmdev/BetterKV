use std::mem;

use serde_json::{Number, Value as JsonValue};

use crate::{Shard, StoredEntry};
use types::value::{CompactKey, Entry};

use super::{JsonError, JsonType};

pub(crate) fn normalize_index(len: usize, index: i64) -> Option<usize> {
    let len_i64 = i64::try_from(len).ok()?;
    let normalized = if index < 0 { len_i64 + index } else { index };
    if !(0..len_i64).contains(&normalized) {
        return None;
    }
    usize::try_from(normalized).ok()
}

pub(crate) fn clamp_insert_index(len: usize, index: i64) -> usize {
    if index <= 0 {
        return 0;
    }
    match usize::try_from(index) {
        Ok(value) => value.min(len),
        Err(_) => len,
    }
}

pub(crate) fn normalize_bounds(len: usize, start: i64, stop: i64) -> Option<(usize, usize)> {
    if len == 0 {
        return None;
    }
    let len_i64 = i64::try_from(len).ok()?;
    let mut from = if start < 0 { len_i64 + start } else { start };
    let mut to = if stop < 0 { len_i64 + stop } else { stop };
    if from < 0 {
        from = 0;
    }
    if to < 0 {
        return None;
    }
    if from >= len_i64 {
        return None;
    }
    if to >= len_i64 {
        to = len_i64 - 1;
    }
    if from > to {
        return None;
    }
    Some((usize::try_from(from).ok()?, usize::try_from(to).ok()?))
}

pub(crate) fn value_type(value: &JsonValue) -> JsonType {
    match value {
        JsonValue::Null => JsonType::Null,
        JsonValue::Bool(_) => JsonType::Boolean,
        JsonValue::Number(number) => {
            if number.is_i64() || number.is_u64() {
                JsonType::Integer
            } else {
                JsonType::Number
            }
        }
        JsonValue::String(_) => JsonType::String,
        JsonValue::Array(_) => JsonType::Array,
        JsonValue::Object(_) => JsonType::Object,
    }
}

pub(crate) fn json_len_bytes(value: &JsonValue) -> usize {
    serde_json::to_vec(value).map_or(0, |value| value.len())
}

pub(crate) fn write_json_entry(
    shard: &mut Shard,
    key: &[u8],
    value: JsonValue,
    ttl_deadline: Option<u64>,
) {
    let compact_key = CompactKey::from_slice(key);
    shard.insert_entry(compact_key, Entry::Json(Box::new(value)), ttl_deadline);
}

pub(crate) fn number_from_f64(value: f64) -> Result<Number, JsonError> {
    if !value.is_finite() {
        return Err(JsonError::Path);
    }
    if value.fract() == 0.0 {
        if value >= 0.0 && value <= u64::MAX as f64 {
            let integer = value as u64;
            if integer as f64 == value {
                return Ok(Number::from(integer));
            }
        }
        if value >= i64::MIN as f64 && value <= i64::MAX as f64 {
            let integer = value as i64;
            if integer as f64 == value {
                return Ok(Number::from(integer));
            }
        }
    }
    Number::from_f64(value).ok_or(JsonError::Path)
}

pub(crate) fn to_f64(value: &JsonValue) -> Option<f64> {
    match value {
        JsonValue::Number(number) => number
            .as_f64()
            .or_else(|| number.as_i64().map(|value| value as f64))
            .or_else(|| number.as_u64().map(|value| value as f64)),
        _ => None,
    }
}

pub(crate) fn json_debug_memory(value: &JsonValue) -> usize {
    match value {
        JsonValue::Null => 0,
        JsonValue::Bool(_) => 1,
        JsonValue::Number(_) => mem::size_of::<Number>(),
        JsonValue::String(text) => text.len(),
        JsonValue::Array(values) => {
            values.iter().map(json_debug_memory).sum::<usize>()
                + values.len() * mem::size_of::<JsonValue>()
        }
        JsonValue::Object(values) => values
            .iter()
            .map(|(key, value)| key.len() + json_debug_memory(value))
            .sum::<usize>(),
    }
}

#[allow(dead_code)]
fn _stored_entry_reference(_: &StoredEntry) {}
