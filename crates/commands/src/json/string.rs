use engine::store::Store;
use protocol::types::RespFrame;
use serde_json::Value as JsonValue;

use crate::util::{wrong_args, Args};

use super::{json_error, optional_path, parse_json, parse_path};

pub(crate) fn json_strlen(store: &Store, args: &Args) -> RespFrame {
    if args.len() < 2 || args.len() > 3 {
        return wrong_args("JSON.STRLEN");
    }
    let path = match optional_path(args, 2) {
        Ok(path) => path,
        Err(err) => return err,
    };
    match store.json_strlen(&args[1], &path) {
        Ok(Some(length)) => RespFrame::Integer(length as i64),
        Ok(None) => RespFrame::Bulk(None),
        Err(err) => json_error(err),
    }
}

pub(crate) fn json_strappend(store: &Store, args: &Args) -> RespFrame {
    if args.len() < 4 {
        return wrong_args("JSON.STRAPPEND");
    }
    let path = match parse_path(&args[2]) {
        Ok(path) => path,
        Err(err) => return err,
    };
    let mut suffixes = Vec::with_capacity(args.len() - 3);
    for value in &args[3..] {
        match parse_json(value) {
            Ok(JsonValue::String(value)) => suffixes.push(value),
            Ok(_) => return RespFrame::error_static("ERR expected JSON string"),
            Err(err) => return err,
        }
    }
    match store.json_strappend(&args[1], &path, &suffixes) {
        Ok(Some(length)) => RespFrame::Integer(length as i64),
        Ok(None) => RespFrame::Bulk(None),
        Err(err) => json_error(err),
    }
}
