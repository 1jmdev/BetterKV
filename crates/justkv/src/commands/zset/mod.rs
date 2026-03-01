mod algebra_ops;
mod core_ops;
mod pop_ops;
mod random_ops;
mod range_ops;
mod scan_ops;

use crate::commands::util::{Args, CommandId};
use crate::engine::store::Store;
use crate::protocol::types::RespFrame;

pub fn handle(store: &Store, cmd: CommandId, args: &Args) -> RespFrame {
    match cmd {
        CommandId::Zadd => core_ops::zadd(store, args),
        CommandId::Zrem => core_ops::zrem(store, args),
        CommandId::Zcard => core_ops::zcard(store, args),
        CommandId::Zcount => core_ops::zcount(store, args),
        CommandId::Zscore => core_ops::zscore(store, args),
        CommandId::Zrank => core_ops::zrank(store, args, false),
        CommandId::Zrevrank => core_ops::zrank(store, args, true),
        CommandId::Zincrby => core_ops::zincrby(store, args),
        CommandId::Zmscore => core_ops::zmscore(store, args),
        CommandId::Zrange => range_ops::zrange(store, args, false),
        CommandId::Zrevrange => range_ops::zrange(store, args, true),
        CommandId::Zrangebyscore => range_ops::zrange_by_score(store, args, false),
        CommandId::Zrevrangebyscore => range_ops::zrange_by_score(store, args, true),
        CommandId::Zpopmin => pop_ops::zpop(store, args, false),
        CommandId::Zpopmax => pop_ops::zpop(store, args, true),
        CommandId::Bzpopmin => pop_ops::bzpop(store, args, false),
        CommandId::Bzpopmax => pop_ops::bzpop(store, args, true),
        CommandId::Zmpop => pop_ops::zmpop(store, args),
        CommandId::Bzmpop => pop_ops::bzmpop(store, args),
        CommandId::Zrandmember => random_ops::zrandmember(store, args),
        CommandId::Zinter => algebra_ops::zop(store, args, "ZINTER"),
        CommandId::Zunion => algebra_ops::zop(store, args, "ZUNION"),
        CommandId::Zdiff => algebra_ops::zop(store, args, "ZDIFF"),
        CommandId::Zscan => scan_ops::zscan(store, args),
        _ => unreachable!("zset::handle called with non-zset command"),
    }
}
