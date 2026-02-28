mod blocking_ops;
mod core_ops;
mod move_ops;
mod range_ops;

use crate::commands::util::Args;
use crate::engine::store::Store;
use crate::protocol::types::RespFrame;

pub fn handle(store: &Store, command: &[u8], args: &Args) -> Option<RespFrame> {
    if let Some(response) = core_ops::handle(store, command, args) {
        return Some(response);
    }
    if let Some(response) = range_ops::handle(store, command, args) {
        return Some(response);
    }
    if let Some(response) = move_ops::handle(store, command, args) {
        return Some(response);
    }
    blocking_ops::handle(store, command, args)
}
