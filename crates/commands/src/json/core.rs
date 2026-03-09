use std::time::{Duration, SystemTime, UNIX_EPOCH};

use engine::store::{JsonError, JsonPath, JsonSetMode, Store};
use protocol::types::{BulkData, RespFrame};
use serde_json::Value as JsonValue;

use crate::util::{eq_ascii, parse_u64_bytes, syntax_error, wrong_args, wrong_type, Args};

pub(crate) fn parse_path(raw: &[u8]) -> Result<JsonPath, RespFrame> {
    JsonPath::parse(raw).map_err(|_| RespFrame::error_static("ERR invalid JSONPath"))
}

pub(crate) fn parse_json(raw: &[u8]) -> Result<JsonValue, RespFrame> {
    match serde_json::from_slice(raw) {
        Ok(value) => Ok(value),
        Err(_) => {
            let Some(first) = raw.first().copied() else {
                return Err(RespFrame::error_static("ERR invalid JSON"));
            };
            let is_bare_string = !matches!(first, b'{' | b'[' | b'"' | b'-' | b'0'..=b'9');
            if is_bare_string {
                if let Ok(value) = std::str::from_utf8(raw) {
                    return Ok(JsonValue::String(value.to_owned()));
                }
            }
            Err(RespFrame::error_static("ERR invalid JSON"))
        }
    }
}

pub(crate) fn parse_number(raw: &[u8]) -> Result<f64, RespFrame> {
    std::str::from_utf8(raw)
        .ok()
        .and_then(|value| value.parse::<f64>().ok())
        .ok_or_else(crate::util::int_error)
}

pub(crate) fn serialize_json(value: &JsonValue) -> Result<Vec<u8>, RespFrame> {
    serde_json::to_vec(value).map_err(|_| RespFrame::error_static("ERR serialization failure"))
}

pub(crate) fn bulk_json(value: JsonValue) -> RespFrame {
    match serialize_json(&value) {
        Ok(bytes) => RespFrame::Bulk(Some(BulkData::from_vec(bytes))),
        Err(err) => err,
    }
}

pub(crate) fn optional_path(args: &Args, index: usize) -> Result<JsonPath, RespFrame> {
    if args.len() > index {
        parse_path(&args[index])
    } else {
        Ok(JsonPath::root())
    }
}

pub(crate) fn json_error(err: JsonError) -> RespFrame {
    match err {
        JsonError::WrongType => wrong_type(),
        JsonError::Syntax => RespFrame::error_static("ERR syntax error"),
        JsonError::Path => {
            RespFrame::error_static("ERR path does not exist or value type mismatch")
        }
        JsonError::KeyMissing => RespFrame::error_static("ERR key does not exist"),
    }
}

fn parse_set_mode(args: &Args, start: usize) -> Result<(JsonSetMode, Option<Duration>), RespFrame> {
    let mut mode = JsonSetMode::Any;
    let mut ttl = None;
    let mut index = start;
    while index < args.len() {
        let option = args[index].as_slice();
        if eq_ascii(option, b"NX") {
            if mode != JsonSetMode::Any {
                return Err(syntax_error());
            }
            mode = JsonSetMode::Nx;
        } else if eq_ascii(option, b"XX") {
            if mode != JsonSetMode::Any {
                return Err(syntax_error());
            }
            mode = JsonSetMode::Xx;
        } else if eq_ascii(option, b"EXAT") || eq_ascii(option, b"PXAT") {
            index += 1;
            if index >= args.len() {
                return Err(wrong_args("JSON.SET"));
            }
            let at = parse_u64_bytes(&args[index]).ok_or_else(crate::util::int_error)?;
            let now_ms = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|value| value.as_millis() as u64)
                .unwrap_or(0);
            let ttl_ms = if eq_ascii(option, b"EXAT") {
                at.saturating_mul(1000).saturating_sub(now_ms)
            } else {
                at.saturating_sub(now_ms)
            };
            ttl = Some(Duration::from_millis(ttl_ms));
        } else {
            return Err(syntax_error());
        }
        index += 1;
    }
    Ok((mode, ttl))
}

pub(crate) fn json_set(store: &Store, args: &Args) -> RespFrame {
    if args.len() < 4 {
        return wrong_args("JSON.SET");
    }
    let path = match parse_path(&args[2]) {
        Ok(path) => path,
        Err(err) => return err,
    };
    let value = match parse_json(&args[3]) {
        Ok(value) => value,
        Err(err) => return err,
    };
    let (mode, ttl) = match parse_set_mode(args, 4) {
        Ok(value) => value,
        Err(err) => return err,
    };
    match store.json_set(&args[1], &path, value, mode, ttl) {
        Ok(result) => {
            if result.applied {
                RespFrame::ok()
            } else {
                RespFrame::Bulk(None)
            }
        }
        Err(err) => json_error(err),
    }
}

pub(crate) fn json_get(store: &Store, args: &Args) -> RespFrame {
    if args.len() < 2 {
        return wrong_args("JSON.GET");
    }
    let paths = if args.len() > 2 {
        let mut parsed = Vec::with_capacity(args.len() - 2);
        for path in &args[2..] {
            match parse_path(path) {
                Ok(path) => parsed.push(path),
                Err(err) => return err,
            }
        }
        parsed
    } else {
        vec![JsonPath::root()]
    };

    let mut values = Vec::with_capacity(paths.len());
    for path in &paths {
        match store.json_get(&args[1], path) {
            Ok(Some(mut matched)) => {
                if matched.is_empty() {
                    return RespFrame::Bulk(None);
                }
                if paths.len() == 1 && matched.len() == 1 {
                    return bulk_json(matched.remove(0));
                }
                if paths.len() == 1 {
                    return bulk_json(JsonValue::Array(matched));
                }
                if matched.len() == 1 {
                    values.push(matched.remove(0));
                } else {
                    values.push(JsonValue::Array(matched));
                }
            }
            Ok(None) => return RespFrame::Bulk(None),
            Err(err) => return json_error(err),
        }
    }
    bulk_json(JsonValue::Array(values))
}

pub(crate) fn json_del(store: &Store, args: &Args) -> RespFrame {
    if args.len() < 2 {
        return wrong_args("JSON.DEL");
    }
    let mut paths = Vec::with_capacity(args.len().saturating_sub(2));
    for path in &args[2..] {
        match parse_path(path) {
            Ok(path) => paths.push(path),
            Err(err) => return err,
        }
    }
    match store.json_del(&args[1], &paths) {
        Ok(value) => RespFrame::Integer(value),
        Err(err) => json_error(err),
    }
}

pub(crate) fn json_forget(store: &Store, args: &Args) -> RespFrame {
    json_del(store, args)
}

pub(crate) fn json_clear(store: &Store, args: &Args) -> RespFrame {
    if args.len() < 2 || args.len() > 3 {
        return wrong_args("JSON.CLEAR");
    }
    let path = match optional_path(args, 2) {
        Ok(path) => path,
        Err(err) => return err,
    };
    match store.json_clear(&args[1], &path) {
        Ok(value) => RespFrame::Integer(value),
        Err(err) => json_error(err),
    }
}

pub(crate) fn json_merge(store: &Store, args: &Args) -> RespFrame {
    if args.len() != 4 {
        return wrong_args("JSON.MERGE");
    }
    let path = match parse_path(&args[2]) {
        Ok(path) => path,
        Err(err) => return err,
    };
    let patch = match parse_json(&args[3]) {
        Ok(value) => value,
        Err(err) => return err,
    };
    match store.json_merge(&args[1], &path, patch) {
        Ok(true) => RespFrame::ok(),
        Ok(false) => RespFrame::Bulk(None),
        Err(err) => json_error(err),
    }
}

pub(crate) fn json_mget(store: &Store, args: &Args) -> RespFrame {
    if args.len() < 3 {
        return wrong_args("JSON.MGET");
    }
    let path = match parse_path(&args[args.len() - 1]) {
        Ok(path) => path,
        Err(err) => return err,
    };
    let keys: Vec<Vec<u8>> = args[1..args.len() - 1]
        .iter()
        .map(|key| key.to_vec())
        .collect();
    match store.json_mget(&keys, &path) {
        Ok(values) => RespFrame::Array(Some(
            values
                .into_iter()
                .map(|value| match value {
                    Some(value) => match serialize_json(&value) {
                        Ok(bytes) => RespFrame::Bulk(Some(BulkData::from_vec(bytes))),
                        Err(err) => err,
                    },
                    None => RespFrame::Bulk(None),
                })
                .collect(),
        )),
        Err(err) => json_error(err),
    }
}

pub(crate) fn json_mset(store: &Store, args: &Args) -> RespFrame {
    if args.len() < 4 || !(args.len() - 1).is_multiple_of(3) {
        return wrong_args("JSON.MSET");
    }
    let mut items = Vec::with_capacity((args.len() - 1) / 3);
    let mut index = 1;
    while index < args.len() {
        let path = match parse_path(&args[index + 1]) {
            Ok(path) => path,
            Err(err) => return err,
        };
        let value = match parse_json(&args[index + 2]) {
            Ok(value) => value,
            Err(err) => return err,
        };
        items.push((args[index].to_vec(), path, value));
        index += 3;
    }
    match store.json_mset(&items) {
        Ok(()) => RespFrame::ok(),
        Err(err) => json_error(err),
    }
}

pub(crate) fn json_resp(store: &Store, args: &Args) -> RespFrame {
    if args.len() < 2 || args.len() > 3 {
        return wrong_args("JSON.RESP");
    }
    let path = match optional_path(args, 2) {
        Ok(path) => path,
        Err(err) => return err,
    };
    match store.json_resp(&args[1], &path) {
        Ok(Some(value)) => bulk_json(value),
        Ok(None) => RespFrame::Bulk(None),
        Err(err) => json_error(err),
    }
}

pub(crate) fn json_toggle(store: &Store, args: &Args) -> RespFrame {
    if args.len() != 3 {
        return wrong_args("JSON.TOGGLE");
    }
    let path = match parse_path(&args[2]) {
        Ok(path) => path,
        Err(err) => return err,
    };
    match store.json_toggle(&args[1], &path) {
        Ok(Some(value)) => RespFrame::Integer(i64::from(value)),
        Ok(None) => RespFrame::Bulk(None),
        Err(err) => json_error(err),
    }
}

pub(crate) fn json_type(store: &Store, args: &Args) -> RespFrame {
    if args.len() < 2 || args.len() > 3 {
        return wrong_args("JSON.TYPE");
    }
    let path = match optional_path(args, 2) {
        Ok(path) => path,
        Err(err) => return err,
    };
    match store.json_type(&args[1], &path) {
        Ok(Some(kind)) => RespFrame::Simple(kind.as_str().to_owned()),
        Ok(None) => RespFrame::Bulk(None),
        Err(err) => json_error(err),
    }
}
