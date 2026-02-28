use crate::commands::util::{eq_ascii, wrong_args, wrong_type, Args};
use crate::engine::store::Store;
use crate::protocol::types::{BulkData, RespFrame};

pub(super) fn handle(store: &Store, command: &[u8], args: &Args) -> Option<RespFrame> {
    if eq_ascii(command, b"ZADD") {
        return Some(zadd(store, args));
    }
    if eq_ascii(command, b"ZREM") {
        return Some(zrem(store, args));
    }
    if eq_ascii(command, b"ZCARD") {
        return Some(zcard(store, args));
    }
    if eq_ascii(command, b"ZCOUNT") {
        return Some(zcount(store, args));
    }
    if eq_ascii(command, b"ZSCORE") {
        return Some(zscore(store, args));
    }
    if eq_ascii(command, b"ZRANK") {
        return Some(zrank(store, args, false));
    }
    if eq_ascii(command, b"ZREVRANK") {
        return Some(zrank(store, args, true));
    }
    if eq_ascii(command, b"ZINCRBY") {
        return Some(zincrby(store, args));
    }
    if eq_ascii(command, b"ZMSCORE") {
        return Some(zmscore(store, args));
    }
    None
}

fn zadd(store: &Store, args: &Args) -> RespFrame {
    if args.len() < 4 || args.len() % 2 != 0 {
        return wrong_args("ZADD");
    }

    let mut pairs = Vec::with_capacity((args.len() - 2) / 2);
    for chunk in args[2..].chunks(2) {
        let score = match parse_f64(&chunk[0]) {
            Ok(value) => value,
            Err(response) => return response,
        };
        pairs.push((score, chunk[1].clone()));
    }

    match store.zadd(&args[1], &pairs) {
        Ok(added) => RespFrame::Integer(added),
        Err(_) => wrong_type(),
    }
}

fn zrem(store: &Store, args: &Args) -> RespFrame {
    if args.len() < 3 {
        return wrong_args("ZREM");
    }
    match store.zrem(&args[1], &args[2..]) {
        Ok(removed) => RespFrame::Integer(removed),
        Err(_) => wrong_type(),
    }
}

fn zcard(store: &Store, args: &Args) -> RespFrame {
    if args.len() != 2 {
        return wrong_args("ZCARD");
    }
    match store.zcard(&args[1]) {
        Ok(size) => RespFrame::Integer(size),
        Err(_) => wrong_type(),
    }
}

fn zcount(store: &Store, args: &Args) -> RespFrame {
    if args.len() != 4 {
        return wrong_args("ZCOUNT");
    }
    let min = match parse_f64(&args[2]) {
        Ok(value) => value,
        Err(response) => return response,
    };
    let max = match parse_f64(&args[3]) {
        Ok(value) => value,
        Err(response) => return response,
    };
    match store.zcount(&args[1], min, max) {
        Ok(value) => RespFrame::Integer(value),
        Err(_) => wrong_type(),
    }
}

fn zscore(store: &Store, args: &Args) -> RespFrame {
    if args.len() != 3 {
        return wrong_args("ZSCORE");
    }
    match store.zscore(&args[1], &args[2]) {
        Ok(Some(score)) => {
            RespFrame::Bulk(Some(BulkData::from_vec(score.to_string().into_bytes())))
        }
        Ok(None) => RespFrame::Bulk(None),
        Err(_) => wrong_type(),
    }
}

fn zrank(store: &Store, args: &Args, reverse: bool) -> RespFrame {
    if args.len() != 3 {
        return wrong_args(if reverse { "ZREVRANK" } else { "ZRANK" });
    }
    match store.zrank(&args[1], &args[2], reverse) {
        Ok(Some(value)) => RespFrame::Integer(value),
        Ok(None) => RespFrame::Bulk(None),
        Err(_) => wrong_type(),
    }
}

fn zincrby(store: &Store, args: &Args) -> RespFrame {
    if args.len() != 4 {
        return wrong_args("ZINCRBY");
    }
    let increment = match parse_f64(&args[2]) {
        Ok(value) => value,
        Err(response) => return response,
    };
    match store.zincrby(&args[1], increment, &args[3]) {
        Ok(score) => RespFrame::Bulk(Some(BulkData::from_vec(score.to_string().into_bytes()))),
        Err(_) => wrong_type(),
    }
}

fn zmscore(store: &Store, args: &Args) -> RespFrame {
    if args.len() < 3 {
        return wrong_args("ZMSCORE");
    }
    match store.zmscore(&args[1], &args[2..]) {
        Ok(scores) => RespFrame::Array(Some(
            scores
                .into_iter()
                .map(|score| {
                    RespFrame::Bulk(
                        score.map(|value| BulkData::from_vec(value.to_string().into_bytes())),
                    )
                })
                .collect(),
        )),
        Err(_) => wrong_type(),
    }
}

fn parse_f64(raw: &[u8]) -> Result<f64, RespFrame> {
    match std::str::from_utf8(raw) {
        Ok(value) => value
            .parse::<f64>()
            .ok()
            .filter(|value| value.is_finite())
            .ok_or_else(|| RespFrame::Error("ERR value is not a valid float".to_string())),
        Err(_) => Err(RespFrame::Error(
            "ERR value is not a valid float".to_string(),
        )),
    }
}
