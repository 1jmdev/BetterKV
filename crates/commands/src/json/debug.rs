use engine::store::Store;
use protocol::types::RespFrame;

use crate::util::{eq_ascii, syntax_error, wrong_args, Args};

use super::{json_error, optional_path};

pub(crate) fn json_debug(store: &Store, args: &Args) -> RespFrame {
    if args.len() < 3 {
        return wrong_args("JSON.DEBUG");
    }
    if !eq_ascii(&args[1], b"MEMORY") {
        return syntax_error();
    }
    let path = match optional_path(args, 3) {
        Ok(path) => path,
        Err(err) => return err,
    };
    match store.json_debug_memory(&args[2], &path) {
        Ok(Some(value)) => RespFrame::Integer(value as i64),
        Ok(None) => RespFrame::Bulk(None),
        Err(err) => json_error(err),
    }
}
