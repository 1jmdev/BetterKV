mod blocking_ops;
mod core_ops;
mod move_ops;
mod range_ops;

use crate::commands::util::{Args, CommandId};
use crate::engine::store::Store;
use crate::protocol::types::RespFrame;

pub fn handle(store: &Store, cmd: CommandId, args: &Args) -> RespFrame {
    match cmd {
        CommandId::Lpush => core_ops::lpush(store, args),
        CommandId::Rpush => core_ops::rpush(store, args),
        CommandId::Lpop => core_ops::lpop(store, args),
        CommandId::Rpop => core_ops::rpop(store, args),
        CommandId::Llen => core_ops::llen(store, args),
        CommandId::Lindex => range_ops::lindex(store, args),
        CommandId::Lrange => range_ops::lrange(store, args),
        CommandId::Lset => range_ops::lset(store, args),
        CommandId::Ltrim => range_ops::ltrim(store, args),
        CommandId::Linsert => range_ops::linsert(store, args),
        CommandId::Lpos => range_ops::lpos(store, args),
        CommandId::Lmove => move_ops::lmove(store, args),
        CommandId::Brpoplpush => move_ops::brpoplpush(store, args),
        CommandId::Lmpop => move_ops::lmpop(store, args),
        CommandId::Blpop => blocking_ops::blpop(store, args),
        CommandId::Brpop => blocking_ops::brpop(store, args),
        CommandId::Blmpop => blocking_ops::blmpop(store, args),
        _ => unreachable!("list::handle called with non-list command"),
    }
}
