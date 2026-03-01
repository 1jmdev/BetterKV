use crate::commands::util::{parse_command_id, CommandId};
use crate::commands::{connection, hash, keyspace, list, set, string, ttl, zset};
use crate::engine::store::Store;
use crate::engine::value::CompactArg;
use crate::protocol::types::{BulkData, RespFrame};

pub fn dispatch(store: &Store, frame: RespFrame) -> RespFrame {
    let args = match parse_command(frame) {
        Ok(args) => args,
        Err(err) => return RespFrame::Error(err),
    };

    dispatch_args(store, &args)
}

pub fn dispatch_args(store: &Store, args: &[CompactArg]) -> RespFrame {
    if args.is_empty() {
        return RespFrame::Error("ERR empty command".to_string());
    }

    // The command was already uppercased in parse_command; resolve it to an
    // enum variant exactly once so that no further string comparisons are
    // needed on the hot path.
    let cmd = parse_command_id(args[0].as_slice());

    match cmd {
        // ── Connection ────────────────────────────────────────────────────
        CommandId::Auth
        | CommandId::Hello
        | CommandId::Client
        | CommandId::Command
        | CommandId::Select
        | CommandId::Quit
        | CommandId::Ping
        | CommandId::Echo => connection::handle(cmd, args),

        // ── Strings ───────────────────────────────────────────────────────
        CommandId::Get
        | CommandId::Set
        | CommandId::Setnx
        | CommandId::Getset
        | CommandId::Getdel
        | CommandId::Setex
        | CommandId::Psetex
        | CommandId::Getex
        | CommandId::Append
        | CommandId::Strlen
        | CommandId::Setrange
        | CommandId::Getrange
        | CommandId::Mget
        | CommandId::Mset
        | CommandId::Msetnx
        | CommandId::Incr
        | CommandId::Incrby
        | CommandId::Decr
        | CommandId::Decrby => string::handle(store, cmd, args),

        // ── Hashes ────────────────────────────────────────────────────────
        CommandId::Hset
        | CommandId::Hmset
        | CommandId::Hsetnx
        | CommandId::Hget
        | CommandId::Hmget
        | CommandId::Hgetall
        | CommandId::Hdel
        | CommandId::Hexists
        | CommandId::Hkeys
        | CommandId::Hvals
        | CommandId::Hlen
        | CommandId::Hstrlen
        | CommandId::Hincrby
        | CommandId::Hincrbyfloat
        | CommandId::Hrandfield
        | CommandId::Hscan => hash::handle(store, cmd, args),

        // ── Lists ─────────────────────────────────────────────────────────
        CommandId::Lpush
        | CommandId::Rpush
        | CommandId::Lpop
        | CommandId::Rpop
        | CommandId::Llen
        | CommandId::Lindex
        | CommandId::Lrange
        | CommandId::Lset
        | CommandId::Ltrim
        | CommandId::Linsert
        | CommandId::Lpos
        | CommandId::Lmove
        | CommandId::Brpoplpush
        | CommandId::Lmpop
        | CommandId::Blpop
        | CommandId::Brpop
        | CommandId::Blmpop => list::handle(store, cmd, args),

        // ── Sets ──────────────────────────────────────────────────────────
        CommandId::Sadd
        | CommandId::Srem
        | CommandId::Smembers
        | CommandId::Sismember
        | CommandId::Scard
        | CommandId::Smove
        | CommandId::Spop
        | CommandId::Srandmember
        | CommandId::Sinter
        | CommandId::Sinterstore
        | CommandId::Sunion
        | CommandId::Sunionstore
        | CommandId::Sdiff
        | CommandId::Sdiffstore
        | CommandId::Sintercard
        | CommandId::Sscan => set::handle(store, cmd, args),

        // ── Sorted sets ───────────────────────────────────────────────────
        CommandId::Zadd
        | CommandId::Zrem
        | CommandId::Zcard
        | CommandId::Zcount
        | CommandId::Zscore
        | CommandId::Zrank
        | CommandId::Zrevrank
        | CommandId::Zincrby
        | CommandId::Zmscore
        | CommandId::Zrange
        | CommandId::Zrevrange
        | CommandId::Zrangebyscore
        | CommandId::Zrevrangebyscore
        | CommandId::Zpopmin
        | CommandId::Zpopmax
        | CommandId::Bzpopmin
        | CommandId::Bzpopmax
        | CommandId::Zmpop
        | CommandId::Bzmpop
        | CommandId::Zrandmember
        | CommandId::Zinter
        | CommandId::Zunion
        | CommandId::Zdiff
        | CommandId::Zscan => zset::handle(store, cmd, args),

        // ── Keyspace ──────────────────────────────────────────────────────
        CommandId::Del
        | CommandId::Exists
        | CommandId::Touch
        | CommandId::Unlink
        | CommandId::Type
        | CommandId::Rename
        | CommandId::Renamenx
        | CommandId::Dbsize
        | CommandId::Keys
        | CommandId::Scan
        | CommandId::Move
        | CommandId::Dump
        | CommandId::Restore
        | CommandId::Sort
        | CommandId::Copy
        | CommandId::Flushdb
        | CommandId::Flushall => keyspace::handle(store, cmd, args),

        // ── TTL ───────────────────────────────────────────────────────────
        CommandId::Expire
        | CommandId::Pexpire
        | CommandId::Expireat
        | CommandId::Pexpireat
        | CommandId::Persist
        | CommandId::Ttl
        | CommandId::Pttl => ttl::handle(store, cmd, args),

        CommandId::Unknown => RespFrame::Error("ERR unknown command".to_string()),
    }
}

pub fn parse_command(frame: RespFrame) -> Result<Vec<CompactArg>, String> {
    let RespFrame::Array(Some(items)) = frame else {
        return Err("ERR protocol error".to_string());
    };

    let mut args = Vec::with_capacity(items.len());
    for item in items {
        match item {
            RespFrame::Bulk(Some(BulkData::Arg(bytes))) => args.push(bytes),
            RespFrame::Bulk(Some(BulkData::Value(bytes))) => {
                args.push(CompactArg::from_vec(bytes.into_vec()))
            }
            RespFrame::Simple(value) => args.push(CompactArg::from_vec(value.into_bytes())),
            _ => return Err("ERR invalid argument type".to_string()),
        }
    }

    if let Some(command) = args.first_mut() {
        let mut normalized = command.to_vec();
        normalized.make_ascii_uppercase();
        *command = CompactArg::from_vec(normalized);
    }

    Ok(args)
}
