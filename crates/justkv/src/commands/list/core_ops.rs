use crate::commands::util::{Args, eq_ascii, int_error, wrong_args, wrong_type};
use crate::engine::store::Store;
use crate::protocol::types::{BulkData, RespFrame};

pub(super) fn handle(store: &Store, command: &[u8], args: &Args) -> Option<RespFrame> {
    if eq_ascii(command, b"LPUSH") {
        return Some(lpush(store, args));
    }
    if eq_ascii(command, b"RPUSH") {
        return Some(rpush(store, args));
    }
    if eq_ascii(command, b"LPOP") {
        return Some(lpop(store, args));
    }
    if eq_ascii(command, b"RPOP") {
        return Some(rpop(store, args));
    }
    if eq_ascii(command, b"LLEN") {
        return Some(llen(store, args));
    }
    None
}

fn lpush(store: &Store, args: &Args) -> RespFrame {
    if args.len() < 3 {
        return wrong_args("LPUSH");
    }
    match store.lpush(&args[1], &args[2..]) {
        Ok(len) => RespFrame::Integer(len),
        Err(_) => wrong_type(),
    }
}

fn rpush(store: &Store, args: &Args) -> RespFrame {
    if args.len() < 3 {
        return wrong_args("RPUSH");
    }
    match store.rpush(&args[1], &args[2..]) {
        Ok(len) => RespFrame::Integer(len),
        Err(_) => wrong_type(),
    }
}

fn lpop(store: &Store, args: &Args) -> RespFrame {
    pop(store, args, true)
}

fn rpop(store: &Store, args: &Args) -> RespFrame {
    pop(store, args, false)
}

fn pop(store: &Store, args: &Args, left: bool) -> RespFrame {
    if args.len() != 2 && args.len() != 3 {
        return wrong_args(if left { "LPOP" } else { "RPOP" });
    }

    let has_count = args.len() == 3;
    let count = if has_count {
        match parse_usize(&args[2]) {
            Ok(value) => value,
            Err(response) => return response,
        }
    } else {
        1
    };

    let result = if left {
        store.lpop(&args[1], count)
    } else {
        store.rpop(&args[1], count)
    };

    match result {
        Ok(Some(values)) if has_count => RespFrame::Array(Some(
            values
                .into_iter()
                .map(|value| RespFrame::Bulk(Some(BulkData::Value(value))))
                .collect(),
        )),
        Ok(Some(mut values)) => RespFrame::Bulk(values.pop().map(BulkData::Value)),
        Ok(None) => RespFrame::Bulk(None),
        Err(_) => wrong_type(),
    }
}

fn llen(store: &Store, args: &Args) -> RespFrame {
    if args.len() != 2 {
        return wrong_args("LLEN");
    }
    match store.llen(&args[1]) {
        Ok(len) => RespFrame::Integer(len),
        Err(_) => wrong_type(),
    }
}

fn parse_usize(raw: &[u8]) -> Result<usize, RespFrame> {
    match std::str::from_utf8(raw) {
        Ok(value) => value
            .parse::<u64>()
            .map_err(|_| int_error())
            .and_then(|value| usize::try_from(value).map_err(|_| int_error())),
        Err(_) => Err(int_error()),
    }
}
