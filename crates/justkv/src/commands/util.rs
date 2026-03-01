use crate::engine::value::CompactArg;
use crate::protocol::types::RespFrame;

pub type Args = [CompactArg];

/// All known commands parsed from the uppercased command name.
/// The dispatcher matches on this enum to avoid repeated string comparisons
/// on the hot path.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandId {
    // Connection
    Auth,
    Hello,
    Client,
    Command,
    Select,
    Quit,
    Ping,
    Echo,
    // Keyspace
    Del,
    Exists,
    Touch,
    Unlink,
    Type,
    Rename,
    Renamenx,
    Dbsize,
    Keys,
    Scan,
    Move,
    Dump,
    Restore,
    Sort,
    Copy,
    Flushdb,
    Flushall,
    // TTL
    Expire,
    Pexpire,
    Expireat,
    Pexpireat,
    Persist,
    Ttl,
    Pttl,
    // String – get/set
    Get,
    Set,
    Setnx,
    Getset,
    Getdel,
    // String – expiry
    Setex,
    Psetex,
    Getex,
    // String – length
    Append,
    Strlen,
    Setrange,
    Getrange,
    // String – multi
    Mget,
    Mset,
    Msetnx,
    // String – counter
    Incr,
    Incrby,
    Decr,
    Decrby,
    // Hash
    Hset,
    Hmset,
    Hsetnx,
    Hget,
    Hmget,
    Hgetall,
    Hdel,
    Hexists,
    Hkeys,
    Hvals,
    Hlen,
    Hstrlen,
    Hincrby,
    Hincrbyfloat,
    Hrandfield,
    Hscan,
    // List
    Lpush,
    Rpush,
    Lpop,
    Rpop,
    Llen,
    Lindex,
    Lrange,
    Lset,
    Ltrim,
    Linsert,
    Lpos,
    Lmove,
    Brpoplpush,
    Lmpop,
    Blpop,
    Brpop,
    Blmpop,
    // Set
    Sadd,
    Srem,
    Smembers,
    Sismember,
    Scard,
    Smove,
    Spop,
    Srandmember,
    Sinter,
    Sinterstore,
    Sunion,
    Sunionstore,
    Sdiff,
    Sdiffstore,
    Sintercard,
    Sscan,
    // Sorted set
    Zadd,
    Zrem,
    Zcard,
    Zcount,
    Zscore,
    Zrank,
    Zrevrank,
    Zincrby,
    Zmscore,
    Zrange,
    Zrevrange,
    Zrangebyscore,
    Zrevrangebyscore,
    Zpopmin,
    Zpopmax,
    Bzpopmin,
    Bzpopmax,
    Zmpop,
    Bzmpop,
    Zrandmember,
    Zinter,
    Zunion,
    Zdiff,
    Zscan,
    // Unknown
    Unknown,
}

/// Parse the (already uppercased) command bytes into a `CommandId`.
/// This is called once per request and replaces the long chain of
/// `eq_ascii` comparisons that used to happen on every routing step.
pub fn parse_command_id(command: &[u8]) -> CommandId {
    match command {
        // Connection
        b"AUTH" => CommandId::Auth,
        b"HELLO" => CommandId::Hello,
        b"CLIENT" => CommandId::Client,
        b"COMMAND" => CommandId::Command,
        b"SELECT" => CommandId::Select,
        b"QUIT" => CommandId::Quit,
        b"PING" => CommandId::Ping,
        b"ECHO" => CommandId::Echo,
        // Keyspace
        b"DEL" => CommandId::Del,
        b"EXISTS" => CommandId::Exists,
        b"TOUCH" => CommandId::Touch,
        b"UNLINK" => CommandId::Unlink,
        b"TYPE" => CommandId::Type,
        b"RENAME" => CommandId::Rename,
        b"RENAMENX" => CommandId::Renamenx,
        b"DBSIZE" => CommandId::Dbsize,
        b"KEYS" => CommandId::Keys,
        b"SCAN" => CommandId::Scan,
        b"MOVE" => CommandId::Move,
        b"DUMP" => CommandId::Dump,
        b"RESTORE" => CommandId::Restore,
        b"SORT" => CommandId::Sort,
        b"COPY" => CommandId::Copy,
        b"FLUSHDB" => CommandId::Flushdb,
        b"FLUSHALL" | b"FLUSH" => CommandId::Flushall,
        // TTL
        b"EXPIRE" => CommandId::Expire,
        b"PEXPIRE" => CommandId::Pexpire,
        b"EXPIREAT" => CommandId::Expireat,
        b"PEXPIREAT" => CommandId::Pexpireat,
        b"PERSIST" => CommandId::Persist,
        b"TTL" => CommandId::Ttl,
        b"PTTL" => CommandId::Pttl,
        // String – get/set
        b"GET" => CommandId::Get,
        b"SET" => CommandId::Set,
        b"SETNX" => CommandId::Setnx,
        b"GETSET" => CommandId::Getset,
        b"GETDEL" => CommandId::Getdel,
        // String – expiry
        b"SETEX" => CommandId::Setex,
        b"PSETEX" => CommandId::Psetex,
        b"GETEX" => CommandId::Getex,
        // String – length
        b"APPEND" => CommandId::Append,
        b"STRLEN" => CommandId::Strlen,
        b"SETRANGE" => CommandId::Setrange,
        b"GETRANGE" => CommandId::Getrange,
        // String – multi
        b"MGET" => CommandId::Mget,
        b"MSET" => CommandId::Mset,
        b"MSETNX" => CommandId::Msetnx,
        // String – counter
        b"INCR" => CommandId::Incr,
        b"INCRBY" => CommandId::Incrby,
        b"DECR" => CommandId::Decr,
        b"DECRBY" => CommandId::Decrby,
        // Hash
        b"HSET" => CommandId::Hset,
        b"HMSET" => CommandId::Hmset,
        b"HSETNX" => CommandId::Hsetnx,
        b"HGET" => CommandId::Hget,
        b"HMGET" => CommandId::Hmget,
        b"HGETALL" => CommandId::Hgetall,
        b"HDEL" => CommandId::Hdel,
        b"HEXISTS" => CommandId::Hexists,
        b"HKEYS" => CommandId::Hkeys,
        b"HVALS" => CommandId::Hvals,
        b"HLEN" => CommandId::Hlen,
        b"HSTRLEN" => CommandId::Hstrlen,
        b"HINCRBY" => CommandId::Hincrby,
        b"HINCRBYFLOAT" => CommandId::Hincrbyfloat,
        b"HRANDFIELD" => CommandId::Hrandfield,
        b"HSCAN" => CommandId::Hscan,
        // List
        b"LPUSH" => CommandId::Lpush,
        b"RPUSH" => CommandId::Rpush,
        b"LPOP" => CommandId::Lpop,
        b"RPOP" => CommandId::Rpop,
        b"LLEN" => CommandId::Llen,
        b"LINDEX" => CommandId::Lindex,
        b"LRANGE" => CommandId::Lrange,
        b"LSET" => CommandId::Lset,
        b"LTRIM" => CommandId::Ltrim,
        b"LINSERT" => CommandId::Linsert,
        b"LPOS" => CommandId::Lpos,
        b"LMOVE" => CommandId::Lmove,
        b"BRPOPLPUSH" => CommandId::Brpoplpush,
        b"LMPOP" => CommandId::Lmpop,
        b"BLPOP" => CommandId::Blpop,
        b"BRPOP" => CommandId::Brpop,
        b"BLMPOP" => CommandId::Blmpop,
        // Set
        b"SADD" => CommandId::Sadd,
        b"SREM" => CommandId::Srem,
        b"SMEMBERS" => CommandId::Smembers,
        b"SISMEMBER" => CommandId::Sismember,
        b"SCARD" => CommandId::Scard,
        b"SMOVE" => CommandId::Smove,
        b"SPOP" => CommandId::Spop,
        b"SRANDMEMBER" => CommandId::Srandmember,
        b"SINTER" => CommandId::Sinter,
        b"SINTERSTORE" => CommandId::Sinterstore,
        b"SUNION" => CommandId::Sunion,
        b"SUNIONSTORE" => CommandId::Sunionstore,
        b"SDIFF" => CommandId::Sdiff,
        b"SDIFFSTORE" => CommandId::Sdiffstore,
        b"SINTERCARD" => CommandId::Sintercard,
        b"SSCAN" => CommandId::Sscan,
        // Sorted set
        b"ZADD" => CommandId::Zadd,
        b"ZREM" => CommandId::Zrem,
        b"ZCARD" => CommandId::Zcard,
        b"ZCOUNT" => CommandId::Zcount,
        b"ZSCORE" => CommandId::Zscore,
        b"ZRANK" => CommandId::Zrank,
        b"ZREVRANK" => CommandId::Zrevrank,
        b"ZINCRBY" => CommandId::Zincrby,
        b"ZMSCORE" => CommandId::Zmscore,
        b"ZRANGE" => CommandId::Zrange,
        b"ZREVRANGE" => CommandId::Zrevrange,
        b"ZRANGEBYSCORE" => CommandId::Zrangebyscore,
        b"ZREVRANGEBYSCORE" => CommandId::Zrevrangebyscore,
        b"ZPOPMIN" => CommandId::Zpopmin,
        b"ZPOPMAX" => CommandId::Zpopmax,
        b"BZPOPMIN" => CommandId::Bzpopmin,
        b"BZPOPMAX" => CommandId::Bzpopmax,
        b"ZMPOP" => CommandId::Zmpop,
        b"BZMPOP" => CommandId::Bzmpop,
        b"ZRANDMEMBER" => CommandId::Zrandmember,
        b"ZINTER" => CommandId::Zinter,
        b"ZUNION" => CommandId::Zunion,
        b"ZDIFF" => CommandId::Zdiff,
        b"ZSCAN" => CommandId::Zscan,
        _ => CommandId::Unknown,
    }
}

pub fn eq_ascii(command: &[u8], expected: &[u8]) -> bool {
    command == expected || command.eq_ignore_ascii_case(expected)
}

pub fn wrong_args(command: &str) -> RespFrame {
    RespFrame::Error(format!(
        "ERR wrong number of arguments for '{command}' command"
    ))
}

pub fn int_error() -> RespFrame {
    RespFrame::error_static("ERR value is not an integer or out of range")
}

pub fn wrong_type() -> RespFrame {
    RespFrame::error_static("WRONGTYPE Operation against a key holding the wrong kind of value")
}

pub fn u64_to_bytes(value: u64) -> Vec<u8> {
    let mut buffer = itoa::Buffer::new();
    buffer.format(value).as_bytes().to_vec()
}

pub fn f64_to_bytes(value: f64) -> Vec<u8> {
    let mut buffer = ryu::Buffer::new();
    buffer.format(value).as_bytes().to_vec()
}
