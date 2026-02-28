mod algebra_ops;
mod core_ops;
mod pop_ops;
mod random_ops;
mod range_ops;
mod scan_ops;

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
    if let Some(response) = pop_ops::handle(store, command, args) {
        return Some(response);
    }
    if let Some(response) = random_ops::handle(store, command, args) {
        return Some(response);
    }
    if let Some(response) = algebra_ops::handle(store, command, args) {
        return Some(response);
    }
    scan_ops::handle(store, command, args)
}
