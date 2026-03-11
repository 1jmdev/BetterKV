use crate::util::{Args, wrong_args, wrong_type};
use engine::store::Store;
use protocol::types::{BulkData, RespFrame};

pub(crate) fn digest(store: &Store, args: &Args) -> RespFrame {
    if args.len() != 2 {
        return wrong_args("DIGEST");
    }

    match store.digest(&args[1]) {
        Ok(value) => RespFrame::Bulk(value.map(BulkData::from_vec)),
        Err(()) => wrong_type(),
    }
}
