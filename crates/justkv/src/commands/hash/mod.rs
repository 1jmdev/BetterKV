mod core_ops;
mod counter_ops;
mod random_ops;
mod scan_ops;

use crate::commands::util::{Args, CommandId};
use crate::engine::store::Store;
use crate::protocol::types::RespFrame;

pub fn handle(store: &Store, cmd: CommandId, args: &Args) -> RespFrame {
    match cmd {
        CommandId::Hset => core_ops::hset(store, args),
        CommandId::Hmset => core_ops::hmset(store, args),
        CommandId::Hsetnx => core_ops::hsetnx(store, args),
        CommandId::Hget => core_ops::hget(store, args),
        CommandId::Hmget => core_ops::hmget(store, args),
        CommandId::Hgetall => core_ops::hgetall(store, args),
        CommandId::Hdel => core_ops::hdel(store, args),
        CommandId::Hexists => core_ops::hexists(store, args),
        CommandId::Hkeys => core_ops::hkeys(store, args),
        CommandId::Hvals => core_ops::hvals(store, args),
        CommandId::Hlen => core_ops::hlen(store, args),
        CommandId::Hstrlen => core_ops::hstrlen(store, args),
        CommandId::Hincrby => counter_ops::hincrby(store, args),
        CommandId::Hincrbyfloat => counter_ops::hincrbyfloat(store, args),
        CommandId::Hrandfield => random_ops::hrandfield(store, args),
        CommandId::Hscan => scan_ops::hscan(store, args),
        _ => unreachable!("hash::handle called with non-hash command"),
    }
}
