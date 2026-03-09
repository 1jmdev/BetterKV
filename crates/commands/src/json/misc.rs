use engine::store::Store;
use protocol::types::RespFrame;

use crate::util::{Args, wrong_args};

use super::{bulk_json, json_error, parse_number, parse_path};

pub(crate) fn json_numincrby(store: &Store, args: &Args) -> RespFrame {
    if args.len() != 4 {
        return wrong_args("JSON.NUMINCRBY");
    }
    let path = match parse_path(&args[2]) {
        Ok(path) => path,
        Err(err) => return err,
    };
    let delta = match parse_number(&args[3]) {
        Ok(value) => value,
        Err(err) => return err,
    };
    match store.json_numincrby(&args[1], &path, delta) {
        Ok(Some(value)) => bulk_json(value),
        Ok(None) => RespFrame::Bulk(None),
        Err(err) => json_error(err),
    }
}

pub(crate) fn json_nummultby(store: &Store, args: &Args) -> RespFrame {
    if args.len() != 4 {
        return wrong_args("JSON.NUMMULTBY");
    }
    let path = match parse_path(&args[2]) {
        Ok(path) => path,
        Err(err) => return err,
    };
    let factor = match parse_number(&args[3]) {
        Ok(value) => value,
        Err(err) => return err,
    };
    match store.json_nummultby(&args[1], &path, factor) {
        Ok(Some(value)) => bulk_json(value),
        Ok(None) => RespFrame::Bulk(None),
        Err(err) => json_error(err),
    }
}
