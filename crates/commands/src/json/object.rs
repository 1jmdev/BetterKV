use engine::store::Store;
use protocol::types::{BulkData, RespFrame};

use crate::util::{Args, wrong_args};

use super::{json_error, optional_path};

pub(crate) fn json_objkeys(store: &Store, args: &Args) -> RespFrame {
    if args.len() < 2 || args.len() > 3 {
        return wrong_args("JSON.OBJKEYS");
    }
    let path = match optional_path(args, 2) {
        Ok(path) => path,
        Err(err) => return err,
    };
    match store.json_objkeys(&args[1], &path) {
        Ok(Some(keys)) => RespFrame::Array(Some(
            keys.into_iter()
                .map(|key| RespFrame::Bulk(Some(BulkData::from_vec(key.into_bytes()))))
                .collect(),
        )),
        Ok(None) => RespFrame::Bulk(None),
        Err(err) => json_error(err),
    }
}

pub(crate) fn json_objlen(store: &Store, args: &Args) -> RespFrame {
    if args.len() < 2 || args.len() > 3 {
        return wrong_args("JSON.OBJLEN");
    }
    let path = match optional_path(args, 2) {
        Ok(path) => path,
        Err(err) => return err,
    };
    match store.json_objlen(&args[1], &path) {
        Ok(Some(length)) => RespFrame::Integer(length as i64),
        Ok(None) => RespFrame::Bulk(None),
        Err(err) => json_error(err),
    }
}
