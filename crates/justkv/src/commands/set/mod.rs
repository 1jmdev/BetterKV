mod algebra_ops;
mod core_ops;
mod random_ops;
mod scan_ops;

use crate::commands::util::{Args, CommandId};
use crate::engine::store::Store;
use crate::protocol::types::RespFrame;

pub fn handle(store: &Store, cmd: CommandId, args: &Args) -> RespFrame {
    match cmd {
        CommandId::Sadd => core_ops::sadd(store, args),
        CommandId::Srem => core_ops::srem(store, args),
        CommandId::Smembers => core_ops::smembers(store, args),
        CommandId::Sismember => core_ops::sismember(store, args),
        CommandId::Scard => core_ops::scard(store, args),
        CommandId::Smove => core_ops::smove(store, args),
        CommandId::Spop => random_ops::spop(store, args),
        CommandId::Srandmember => random_ops::srandmember(store, args),
        CommandId::Sinter => algebra_ops::sinter(store, args),
        CommandId::Sinterstore => algebra_ops::sinterstore(store, args),
        CommandId::Sunion => algebra_ops::sunion(store, args),
        CommandId::Sunionstore => algebra_ops::sunionstore(store, args),
        CommandId::Sdiff => algebra_ops::sdiff(store, args),
        CommandId::Sdiffstore => algebra_ops::sdiffstore(store, args),
        CommandId::Sintercard => algebra_ops::sintercard(store, args),
        CommandId::Sscan => scan_ops::sscan(store, args),
        _ => unreachable!("set::handle called with non-set command"),
    }
}
