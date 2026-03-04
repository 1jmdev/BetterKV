use std::collections::{BTreeMap, VecDeque};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use ahash::RandomState;
use hashbrown::HashMap;
use indexmap::IndexSet;

use engine::store::Store;

const RDB_MAGIC_PREFIX: &[u8; 5] = b"REDIS";
const RDB_VERSION: &str = "0004";
const OP_EXPIRETIME: u8 = 0xFD;
const OP_SELECTDB: u8 = 0xFE;
const OP_EOF: u8 = 0xFF;
const TYPE_STRING: u8 = 0;
const TYPE_LIST: u8 = 1;
const TYPE_SET: u8 = 2;
const TYPE_ZSET: u8 = 3;
const TYPE_HASH: u8 = 4;
const EMBEDDED_PREFIX: &[u8] = b"__JUSTKV_EMBEDDED__";

#[derive(Debug, Clone, Copy)]
pub struct SnapshotStats {
    pub keys_written: u64,
    pub bytes_written: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct RestoreStats {
    pub keys_loaded: u64,
}

#[derive(Debug, Clone)]
enum Value {
    String(Vec<u8>),
    Hash(Vec<(Vec<u8>, Vec<u8>)>),
    List(Vec<Vec<u8>>),
    Set(Vec<Vec<u8>>),
    ZSet(Vec<(Vec<u8>, f64)>),
    Geo(Vec<(Vec<u8>, (f64, f64))>),
    Stream(Vec<StreamEntry>),
}

#[derive(Debug, Clone)]
struct StreamEntry {
    ms: u64,
    seq: u64,
    fields: Vec<(Vec<u8>, Vec<u8>)>,
}

#[derive(Debug)]
struct LoadedEntry {
    key: Vec<u8>,
    expire_at_s: Option<u32>,
    value: Value,
}

pub fn load_snapshot(store: &Store, path: &Path) -> Result<RestoreStats, String> {
    let _trace = profiler::scope("server::backup::load_snapshot");
    let bytes = std::fs::read(path)
        .map_err(|err| format!("failed to read snapshot {}: {err}", path.display()))?;
    let entries = parse_rdb(&bytes)?;

    let now_s = now_unix_seconds();
    let mut loaded = 0u64;
    for entry in entries {
        if let Some(expire_at) = entry.expire_at_s
            && u64::from(expire_at) <= now_s
        {
            continue;
        }

        let payload = encode_custom_entry(&entry.value);
        let ttl_ms = entry
            .expire_at_s
            .map(|ts| {
                let remaining_s = u64::from(ts).saturating_sub(now_s);
                remaining_s.saturating_mul(1000)
            })
            .unwrap_or(0);

        store
            .restore(&entry.key, ttl_ms, &payload, true)
            .map_err(|_| format!("failed to restore key from {}", path.display()))?;
        loaded += 1;
    }

    Ok(RestoreStats {
        keys_loaded: loaded,
    })
}

pub fn write_snapshot(store: &Store, path: &Path) -> Result<SnapshotStats, String> {
    let _trace = profiler::scope("server::backup::write_snapshot");
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|err| {
            format!(
                "failed to create snapshot directory {}: {err}",
                parent.display()
            )
        })?;
    }

    let mut out = Vec::with_capacity(64 * 1024);
    out.extend_from_slice(RDB_MAGIC_PREFIX);
    out.extend_from_slice(RDB_VERSION.as_bytes());

    out.push(OP_SELECTDB);
    encode_len(&mut out, 0)?;

    let now_s = now_unix_seconds();
    let mut written_keys = 0u64;
    for key in store.keys(b"*") {
        let Some(payload) = store.dump(&key) else {
            continue;
        };

        let ttl_ms = store.pttl(&key);
        let value = decode_custom_entry(&payload)?;
        let rdb_value = match value {
            Value::Geo(_) | Value::Stream(_) => {
                let mut bytes = EMBEDDED_PREFIX.to_vec();
                bytes.extend_from_slice(&payload);
                RdbValue::String(bytes)
            }
            Value::String(value) => RdbValue::String(value),
            Value::Hash(pairs) => RdbValue::Hash(pairs),
            Value::List(values) => RdbValue::List(values),
            Value::Set(values) => RdbValue::Set(values),
            Value::ZSet(values) => RdbValue::ZSet(values),
        };

        if ttl_ms >= 0 {
            let expire_at_s = now_s.saturating_add((ttl_ms as u64).div_ceil(1000));
            let expire_at_s_u32 = u32::try_from(expire_at_s)
                .map_err(|_| format!("ttl overflow while writing {}", path.display()))?;
            out.push(OP_EXPIRETIME);
            out.extend_from_slice(&expire_at_s_u32.to_le_bytes());
        }

        write_rdb_value(&mut out, &key, rdb_value)?;
        written_keys += 1;
    }

    out.push(OP_EOF);

    let temp_path = path.with_extension("tmp");
    let file = File::create(&temp_path)
        .map_err(|err| format!("failed to create snapshot {}: {err}", temp_path.display()))?;
    let mut writer = BufWriter::new(file);
    writer
        .write_all(&out)
        .map_err(|err| format!("failed to write snapshot {}: {err}", temp_path.display()))?;
    writer
        .flush()
        .map_err(|err| format!("failed to flush snapshot {}: {err}", temp_path.display()))?;
    std::fs::rename(&temp_path, path).map_err(|err| {
        format!(
            "failed to move snapshot {} to {}: {err}",
            temp_path.display(),
            path.display()
        )
    })?;

    Ok(SnapshotStats {
        keys_written: written_keys,
        bytes_written: out.len() as u64,
    })
}

enum RdbValue {
    String(Vec<u8>),
    List(Vec<Vec<u8>>),
    Set(Vec<Vec<u8>>),
    ZSet(Vec<(Vec<u8>, f64)>),
    Hash(Vec<(Vec<u8>, Vec<u8>)>),
}

fn write_rdb_value(out: &mut Vec<u8>, key: &[u8], value: RdbValue) -> Result<(), String> {
    let _trace = profiler::scope("server::backup::write_rdb_value");
    match value {
        RdbValue::String(v) => {
            out.push(TYPE_STRING);
            encode_string(out, key)?;
            encode_string(out, &v)?;
        }
        RdbValue::List(values) => {
            out.push(TYPE_LIST);
            encode_string(out, key)?;
            encode_len(out, values.len())?;
            for value in values {
                encode_string(out, &value)?;
            }
        }
        RdbValue::Set(values) => {
            out.push(TYPE_SET);
            encode_string(out, key)?;
            encode_len(out, values.len())?;
            for value in values {
                encode_string(out, &value)?;
            }
        }
        RdbValue::ZSet(values) => {
            out.push(TYPE_ZSET);
            encode_string(out, key)?;
            encode_len(out, values.len())?;
            for (member, score) in values {
                encode_string(out, &member)?;
                encode_zset_score(out, score);
            }
        }
        RdbValue::Hash(values) => {
            out.push(TYPE_HASH);
            encode_string(out, key)?;
            encode_len(out, values.len())?;
            for (field, value) in values {
                encode_string(out, &field)?;
                encode_string(out, &value)?;
            }
        }
    }

    Ok(())
}

fn parse_rdb(input: &[u8]) -> Result<Vec<LoadedEntry>, String> {
    let _trace = profiler::scope("server::backup::parse_rdb");
    if input.len() < 9 {
        return Err("snapshot file is too short".to_string());
    }
    if &input[..5] != RDB_MAGIC_PREFIX {
        return Err("snapshot is not a Redis/Valkey RDB file".to_string());
    }
    let version_raw = std::str::from_utf8(&input[5..9]).map_err(|_| "invalid RDB header")?;
    let _version = version_raw
        .parse::<u32>()
        .map_err(|_| format!("invalid RDB version '{version_raw}'"))?;

    let mut cursor = 9usize;
    let mut expire_at_s: Option<u32> = None;
    let mut entries = Vec::new();

    while cursor < input.len() {
        let op = read_u8(input, &mut cursor)?;
        match op {
            OP_EOF => break,
            OP_SELECTDB => {
                let _ = decode_len(input, &mut cursor)?;
                expire_at_s = None;
            }
            OP_EXPIRETIME => {
                let mut buf = [0u8; 4];
                read_exact(input, &mut cursor, &mut buf)?;
                expire_at_s = Some(u32::from_le_bytes(buf));
            }
            0xFC => {
                let mut buf = [0u8; 8];
                read_exact(input, &mut cursor, &mut buf)?;
                let expire_ms = u64::from_le_bytes(buf);
                let expire_s = expire_ms.div_ceil(1000);
                let expire_s_u32 = u32::try_from(expire_s)
                    .map_err(|_| "RDB millisecond expiration out of range".to_string())?;
                expire_at_s = Some(expire_s_u32);
            }
            0xFA => {
                let _ = decode_string(input, &mut cursor)?;
                let _ = decode_string(input, &mut cursor)?;
            }
            0xFB => {
                let _ = decode_len(input, &mut cursor)?;
                let _ = decode_len(input, &mut cursor)?;
            }
            TYPE_STRING | TYPE_LIST | TYPE_SET | TYPE_ZSET | TYPE_HASH => {
                let key = decode_string(input, &mut cursor)?;
                let value = match op {
                    TYPE_STRING => {
                        let raw = decode_string(input, &mut cursor)?;
                        if raw.starts_with(EMBEDDED_PREFIX) {
                            let payload = &raw[EMBEDDED_PREFIX.len()..];
                            decode_custom_entry(payload)?
                        } else {
                            Value::String(raw)
                        }
                    }
                    TYPE_LIST => {
                        let len = decode_len(input, &mut cursor)?;
                        let mut values = Vec::with_capacity(len);
                        for _ in 0..len {
                            values.push(decode_string(input, &mut cursor)?);
                        }
                        Value::List(values)
                    }
                    TYPE_SET => {
                        let len = decode_len(input, &mut cursor)?;
                        let mut values = Vec::with_capacity(len);
                        for _ in 0..len {
                            values.push(decode_string(input, &mut cursor)?);
                        }
                        Value::Set(values)
                    }
                    TYPE_ZSET => {
                        let len = decode_len(input, &mut cursor)?;
                        let mut values = Vec::with_capacity(len);
                        for _ in 0..len {
                            let member = decode_string(input, &mut cursor)?;
                            let score = decode_zset_score(input, &mut cursor)?;
                            values.push((member, score));
                        }
                        Value::ZSet(values)
                    }
                    TYPE_HASH => {
                        let len = decode_len(input, &mut cursor)?;
                        let mut values = Vec::with_capacity(len);
                        for _ in 0..len {
                            let field = decode_string(input, &mut cursor)?;
                            let value = decode_string(input, &mut cursor)?;
                            values.push((field, value));
                        }
                        Value::Hash(values)
                    }
                    _ => unreachable!(),
                };

                entries.push(LoadedEntry {
                    key,
                    expire_at_s,
                    value,
                });
                expire_at_s = None;
            }
            _ => return Err(format!("unsupported RDB opcode/type: {op}")),
        }
    }

    Ok(entries)
}

fn encode_len(out: &mut Vec<u8>, len: usize) -> Result<(), String> {
    let _trace = profiler::scope("server::backup::encode_len");
    if len < (1 << 6) {
        out.push(len as u8);
        return Ok(());
    }
    if len < (1 << 14) {
        out.push(((len >> 8) as u8 & 0x3F) | 0b0100_0000);
        out.push((len & 0xFF) as u8);
        return Ok(());
    }

    let len_u32 = u32::try_from(len).map_err(|_| "RDB length over u32 is unsupported")?;
    out.push(0b1000_0000);
    out.extend_from_slice(&len_u32.to_be_bytes());
    Ok(())
}

fn decode_len(input: &[u8], cursor: &mut usize) -> Result<usize, String> {
    let _trace = profiler::scope("server::backup::decode_len");
    let first = read_u8(input, cursor)?;
    decode_len_with_first(input, cursor, first)
}

fn encode_string(out: &mut Vec<u8>, value: &[u8]) -> Result<(), String> {
    let _trace = profiler::scope("server::backup::encode_string");
    encode_len(out, value.len())?;
    out.extend_from_slice(value);
    Ok(())
}

fn decode_string(input: &[u8], cursor: &mut usize) -> Result<Vec<u8>, String> {
    let _trace = profiler::scope("server::backup::decode_string");
    let first = read_u8(input, cursor)?;
    let mode = first >> 6;
    if mode != 0b11 {
        let len = decode_len_with_first(input, cursor, first)?;
        let mut out = vec![0u8; len];
        read_exact(input, cursor, &mut out)?;
        return Ok(out);
    }

    match first & 0x3F {
        0 => {
            let value = read_u8(input, cursor)? as i8;
            Ok(value.to_string().into_bytes())
        }
        1 => {
            let mut buf = [0u8; 2];
            read_exact(input, cursor, &mut buf)?;
            let value = i16::from_le_bytes(buf);
            Ok(value.to_string().into_bytes())
        }
        2 => {
            let mut buf = [0u8; 4];
            read_exact(input, cursor, &mut buf)?;
            let value = i32::from_le_bytes(buf);
            Ok(value.to_string().into_bytes())
        }
        3 => {
            let compressed_len = decode_len(input, cursor)?;
            let uncompressed_len = decode_len(input, cursor)?;
            let mut compressed = vec![0u8; compressed_len];
            read_exact(input, cursor, &mut compressed)?;
            lzf_decompress(&compressed, uncompressed_len)
        }
        _ => Err("unsupported RDB string encoding".to_string()),
    }
}

fn decode_len_with_first(input: &[u8], cursor: &mut usize, first: u8) -> Result<usize, String> {
    let _trace = profiler::scope("server::backup::decode_len_with_first");
    let mode = first >> 6;
    match mode {
        0b00 => Ok((first & 0x3F) as usize),
        0b01 => {
            let second = read_u8(input, cursor)?;
            Ok((((first & 0x3F) as usize) << 8) | second as usize)
        }
        0b10 => match first & 0x3F {
            0 => {
                let mut buf = [0u8; 4];
                read_exact(input, cursor, &mut buf)?;
                Ok(u32::from_be_bytes(buf) as usize)
            }
            1 => {
                let mut buf = [0u8; 8];
                read_exact(input, cursor, &mut buf)?;
                usize::try_from(u64::from_be_bytes(buf))
                    .map_err(|_| "RDB 64-bit length does not fit usize".to_string())
            }
            _ => Err("unsupported RDB length encoding".to_string()),
        },
        _ => Err("RDB encoded value is not a plain length".to_string()),
    }
}

fn lzf_decompress(input: &[u8], expected_len: usize) -> Result<Vec<u8>, String> {
    let _trace = profiler::scope("server::backup::lzf_decompress");
    let mut out = Vec::with_capacity(expected_len);
    let mut i = 0usize;

    while i < input.len() {
        let ctrl = input[i] as usize;
        i += 1;

        if ctrl < 32 {
            let len = ctrl + 1;
            if i + len > input.len() {
                return Err("invalid LZF literal length".to_string());
            }
            out.extend_from_slice(&input[i..i + len]);
            i += len;
            continue;
        }

        let mut len = (ctrl >> 5) + 2;
        let mut ref_offset = (ctrl & 0x1F) << 8;
        if i >= input.len() {
            return Err("invalid LZF back-reference".to_string());
        }
        ref_offset += input[i] as usize;
        i += 1;

        if len == 9 {
            if i >= input.len() {
                return Err("invalid LZF extended length".to_string());
            }
            len += input[i] as usize;
            i += 1;
        }

        let back = ref_offset + 1;
        if back > out.len() {
            return Err("invalid LZF match distance".to_string());
        }
        let start = out.len() - back;
        for j in 0..len {
            let b = out[start + j];
            out.push(b);
        }
    }

    if out.len() != expected_len {
        return Err(format!(
            "invalid LZF output length: expected {expected_len}, got {}",
            out.len()
        ));
    }

    Ok(out)
}

fn encode_zset_score(out: &mut Vec<u8>, score: f64) {
    let _trace = profiler::scope("server::backup::encode_zset_score");
    if score.is_nan() {
        out.push(253);
        return;
    }
    if score == f64::INFINITY {
        out.push(254);
        return;
    }
    if score == f64::NEG_INFINITY {
        out.push(255);
        return;
    }

    let encoded = score.to_string();
    out.push(encoded.len() as u8);
    out.extend_from_slice(encoded.as_bytes());
}

fn decode_zset_score(input: &[u8], cursor: &mut usize) -> Result<f64, String> {
    let _trace = profiler::scope("server::backup::decode_zset_score");
    let len = read_u8(input, cursor)?;
    match len {
        253 => Ok(f64::NAN),
        254 => Ok(f64::INFINITY),
        255 => Ok(f64::NEG_INFINITY),
        n => {
            let mut bytes = vec![0u8; n as usize];
            read_exact(input, cursor, &mut bytes)?;
            let raw = std::str::from_utf8(&bytes).map_err(|_| "invalid zset score utf8")?;
            raw.parse::<f64>()
                .map_err(|_| format!("invalid zset score '{raw}'"))
        }
    }
}

fn read_u8(input: &[u8], cursor: &mut usize) -> Result<u8, String> {
    let _trace = profiler::scope("server::backup::read_u8");
    if *cursor >= input.len() {
        return Err("unexpected EOF".to_string());
    }
    let b = input[*cursor];
    *cursor += 1;
    Ok(b)
}

fn read_exact(input: &[u8], cursor: &mut usize, out: &mut [u8]) -> Result<(), String> {
    let _trace = profiler::scope("server::backup::read_exact");
    let remaining = input.len().saturating_sub(*cursor);
    if out.len() > remaining {
        return Err("unexpected EOF".to_string());
    }
    out.copy_from_slice(&input[*cursor..*cursor + out.len()]);
    *cursor += out.len();
    Ok(())
}

fn now_unix_seconds() -> u64 {
    let _trace = profiler::scope("server::backup::now_unix_seconds");
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn encode_custom_entry(value: &Value) -> Vec<u8> {
    let _trace = profiler::scope("server::backup::encode_custom_entry");
    let mut out = vec![1u8];
    match value {
        Value::String(v) => {
            out.push(0);
            write_bytes(&mut out, v);
        }
        Value::Hash(values) => {
            out.push(1);
            write_u32(&mut out, values.len() as u32);
            for (field, value) in values {
                write_bytes(&mut out, field);
                write_bytes(&mut out, value);
            }
        }
        Value::List(values) => {
            out.push(2);
            write_u32(&mut out, values.len() as u32);
            for value in values {
                write_bytes(&mut out, value);
            }
        }
        Value::Set(values) => {
            out.push(3);
            write_u32(&mut out, values.len() as u32);
            for value in values {
                write_bytes(&mut out, value);
            }
        }
        Value::ZSet(values) => {
            out.push(4);
            write_u32(&mut out, values.len() as u32);
            for (member, score) in values {
                write_bytes(&mut out, member);
                out.extend_from_slice(&score.to_le_bytes());
            }
        }
        Value::Geo(values) => {
            out.push(5);
            write_u32(&mut out, values.len() as u32);
            for (member, (lon, lat)) in values {
                write_bytes(&mut out, member);
                out.extend_from_slice(&lon.to_le_bytes());
                out.extend_from_slice(&lat.to_le_bytes());
            }
        }
        Value::Stream(values) => {
            out.push(6);
            write_u32(&mut out, values.len() as u32);
            for entry in values {
                out.extend_from_slice(&entry.ms.to_le_bytes());
                out.extend_from_slice(&entry.seq.to_le_bytes());
                write_u32(&mut out, entry.fields.len() as u32);
                for (field, value) in &entry.fields {
                    write_bytes(&mut out, field);
                    write_bytes(&mut out, value);
                }
            }
        }
    }
    out
}

fn decode_custom_entry(payload: &[u8]) -> Result<Value, String> {
    let _trace = profiler::scope("server::backup::decode_custom_entry");
    if payload.len() < 2 || payload[0] != 1 {
        return Err("invalid payload".to_string());
    }

    let mut input = &payload[2..];
    let value = match payload[1] {
        0 => Value::String(read_bytes(&mut input)?),
        1 => {
            let count = read_u32(&mut input)? as usize;
            let mut map: HashMap<Vec<u8>, Vec<u8>, RandomState> =
                HashMap::with_capacity_and_hasher(count, RandomState::new());
            for _ in 0..count {
                let field = read_bytes(&mut input)?;
                let value = read_bytes(&mut input)?;
                map.insert(field, value);
            }
            Value::Hash(map.into_iter().collect())
        }
        2 => {
            let count = read_u32(&mut input)? as usize;
            let mut list = VecDeque::with_capacity(count);
            for _ in 0..count {
                list.push_back(read_bytes(&mut input)?);
            }
            Value::List(list.into_iter().collect())
        }
        3 => {
            let count = read_u32(&mut input)? as usize;
            let mut set: IndexSet<Vec<u8>, RandomState> =
                IndexSet::with_capacity_and_hasher(count, RandomState::new());
            for _ in 0..count {
                set.insert(read_bytes(&mut input)?);
            }
            Value::Set(set.into_iter().collect())
        }
        4 => {
            let count = read_u32(&mut input)? as usize;
            let mut values = Vec::with_capacity(count);
            for _ in 0..count {
                let member = read_bytes(&mut input)?;
                if input.len() < 8 {
                    return Err("invalid zset payload".to_string());
                }
                let mut bytes = [0u8; 8];
                bytes.copy_from_slice(&input[..8]);
                input = &input[8..];
                values.push((member, f64::from_le_bytes(bytes)));
            }
            Value::ZSet(values)
        }
        5 => {
            let count = read_u32(&mut input)? as usize;
            let mut values = Vec::with_capacity(count);
            for _ in 0..count {
                let member = read_bytes(&mut input)?;
                if input.len() < 16 {
                    return Err("invalid geo payload".to_string());
                }
                let mut lon = [0u8; 8];
                lon.copy_from_slice(&input[..8]);
                let mut lat = [0u8; 8];
                lat.copy_from_slice(&input[8..16]);
                input = &input[16..];
                values.push((member, (f64::from_le_bytes(lon), f64::from_le_bytes(lat))));
            }
            Value::Geo(values)
        }
        6 => {
            let count = read_u32(&mut input)? as usize;
            let mut entries = BTreeMap::new();
            for _ in 0..count {
                if input.len() < 16 {
                    return Err("invalid stream payload".to_string());
                }
                let mut ms = [0u8; 8];
                ms.copy_from_slice(&input[..8]);
                let mut seq = [0u8; 8];
                seq.copy_from_slice(&input[8..16]);
                input = &input[16..];
                let field_count = read_u32(&mut input)? as usize;
                let mut fields = Vec::with_capacity(field_count);
                for _ in 0..field_count {
                    fields.push((read_bytes(&mut input)?, read_bytes(&mut input)?));
                }
                entries.insert(
                    (u64::from_le_bytes(ms), u64::from_le_bytes(seq)),
                    StreamEntry {
                        ms: u64::from_le_bytes(ms),
                        seq: u64::from_le_bytes(seq),
                        fields,
                    },
                );
            }
            Value::Stream(entries.into_values().collect())
        }
        _ => return Err("unsupported payload type".to_string()),
    };

    if !input.is_empty() {
        return Err("payload has trailing bytes".to_string());
    }
    Ok(value)
}

fn write_u32(out: &mut Vec<u8>, value: u32) {
    let _trace = profiler::scope("server::backup::write_u32");
    out.extend_from_slice(&value.to_le_bytes());
}

fn read_u32(input: &mut &[u8]) -> Result<u32, String> {
    let _trace = profiler::scope("server::backup::read_u32");
    if input.len() < 4 {
        return Err("unexpected EOF".to_string());
    }
    let mut bytes = [0u8; 4];
    bytes.copy_from_slice(&input[..4]);
    *input = &input[4..];
    Ok(u32::from_le_bytes(bytes))
}

fn write_bytes(out: &mut Vec<u8>, value: &[u8]) {
    let _trace = profiler::scope("server::backup::write_bytes");
    write_u32(out, value.len() as u32);
    out.extend_from_slice(value);
}

fn read_bytes(input: &mut &[u8]) -> Result<Vec<u8>, String> {
    let _trace = profiler::scope("server::backup::read_bytes");
    let len = read_u32(input)? as usize;
    if input.len() < len {
        return Err("unexpected EOF".to_string());
    }
    let out = input[..len].to_vec();
    *input = &input[len..];
    Ok(out)
}
