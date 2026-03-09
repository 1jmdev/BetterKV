use engine::store::Store;
use protocol::types::RespFrame;

use crate::util::{Args, parse_i64_bytes, wrong_args};

use super::{bulk_json, json_error, parse_json, parse_path};

pub(crate) fn json_arrappend(store: &Store, args: &Args) -> RespFrame {
    if args.len() < 4 {
        return wrong_args("JSON.ARRAPPEND");
    }
    let path = match parse_path(&args[2]) {
        Ok(path) => path,
        Err(err) => return err,
    };
    let mut values = Vec::with_capacity(args.len() - 3);
    for value in &args[3..] {
        match parse_json(value) {
            Ok(value) => values.push(value),
            Err(err) => return err,
        }
    }
    match store.json_arrappend(&args[1], &path, values) {
        Ok(Some(length)) => RespFrame::Integer(length as i64),
        Ok(None) => RespFrame::Bulk(None),
        Err(err) => json_error(err),
    }
}

pub(crate) fn json_arrindex(store: &Store, args: &Args) -> RespFrame {
    if args.len() < 4 || args.len() > 6 {
        return wrong_args("JSON.ARRINDEX");
    }
    let path = match parse_path(&args[2]) {
        Ok(path) => path,
        Err(err) => return err,
    };
    let needle = match parse_json(&args[3]) {
        Ok(value) => value,
        Err(err) => return err,
    };
    let start = if args.len() >= 5 {
        match parse_i64_bytes(&args[4]) {
            Some(value) => value,
            None => return crate::util::int_error(),
        }
    } else {
        0
    };
    let stop = if args.len() == 6 {
        match parse_i64_bytes(&args[5]) {
            Some(value) => Some(value),
            None => return crate::util::int_error(),
        }
    } else {
        None
    };
    match store.json_arrindex(&args[1], &path, &needle, start, stop) {
        Ok(Some(index)) => RespFrame::Integer(index),
        Ok(None) => RespFrame::Bulk(None),
        Err(err) => json_error(err),
    }
}

pub(crate) fn json_arrinsert(store: &Store, args: &Args) -> RespFrame {
    if args.len() < 5 {
        return wrong_args("JSON.ARRINSERT");
    }
    let path = match parse_path(&args[2]) {
        Ok(path) => path,
        Err(err) => return err,
    };
    let index = match parse_i64_bytes(&args[3]) {
        Some(value) => value,
        None => return crate::util::int_error(),
    };
    let mut values = Vec::with_capacity(args.len() - 4);
    for value in &args[4..] {
        match parse_json(value) {
            Ok(value) => values.push(value),
            Err(err) => return err,
        }
    }
    match store.json_arrinsert(&args[1], &path, index, values) {
        Ok(Some(length)) => RespFrame::Integer(length as i64),
        Ok(None) => RespFrame::Bulk(None),
        Err(err) => json_error(err),
    }
}

pub(crate) fn json_arrlen(store: &Store, args: &Args) -> RespFrame {
    if args.len() < 2 || args.len() > 3 {
        return wrong_args("JSON.ARRLEN");
    }
    let path = match super::optional_path(args, 2) {
        Ok(path) => path,
        Err(err) => return err,
    };
    match store.json_arrlen(&args[1], &path) {
        Ok(Some(length)) => RespFrame::Integer(length as i64),
        Ok(None) => RespFrame::Bulk(None),
        Err(err) => json_error(err),
    }
}

pub(crate) fn json_arrpop(store: &Store, args: &Args) -> RespFrame {
    if args.len() < 2 || args.len() > 4 {
        return wrong_args("JSON.ARRPOP");
    }
    let path = if args.len() >= 3 {
        match parse_path(&args[2]) {
            Ok(path) => path,
            Err(err) => return err,
        }
    } else {
        engine::store::JsonPath::root()
    };
    let index = if args.len() == 4 {
        match parse_i64_bytes(&args[3]) {
            Some(value) => value,
            None => return crate::util::int_error(),
        }
    } else {
        -1
    };
    match store.json_arrpop(&args[1], &path, index) {
        Ok(Some(value)) => bulk_json(value),
        Ok(None) => RespFrame::Bulk(None),
        Err(err) => json_error(err),
    }
}

pub(crate) fn json_arrtrim(store: &Store, args: &Args) -> RespFrame {
    if args.len() != 5 {
        return wrong_args("JSON.ARRTRIM");
    }
    let path = match parse_path(&args[2]) {
        Ok(path) => path,
        Err(err) => return err,
    };
    let start = match parse_i64_bytes(&args[3]) {
        Some(value) => value,
        None => return crate::util::int_error(),
    };
    let stop = match parse_i64_bytes(&args[4]) {
        Some(value) => value,
        None => return crate::util::int_error(),
    };
    match store.json_arrtrim(&args[1], &path, start, stop) {
        Ok(Some(length)) => RespFrame::Integer(length as i64),
        Ok(None) => RespFrame::Bulk(None),
        Err(err) => json_error(err),
    }
}
