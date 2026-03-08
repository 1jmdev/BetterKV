use crate::util::{cmd, pack_runtime};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandId {
    Unknown,
    Auth,
    Hello,
    Client,
    Command,
    Select,
    Quit,
    Ping,
    Echo,
    Eval,
    EvalRo,
    EvalSha,
    EvalShaRo,
    Script,
    Del,
    Exists,
    Touch,
    Unlink,
    Type,
    Rename,
    RenameNx,
    DbSize,
    Keys,
    Scan,
    Move,
    Dump,
    Restore,
    Sort,
    Copy,
    FlushDb,
    FlushAll,
    Expire,
    PExpire,
    ExpireAt,
    PExpireAt,
    Persist,
    Ttl,
    PTtl,
    Get,
    Set,
    SetNx,
    GetSet,
    GetDel,
    SetEx,
    PSetEx,
    GetEx,
    Append,
    StrLen,
    SetRange,
    GetRange,
    MGet,
    MSet,
    MSetNx,
    Incr,
    IncrBy,
    Decr,
    DecrBy,
    SetBit,
    GetBit,
    BitCount,
    BitPos,
    BitOp,
    BitField,
    BitFieldRo,
    PfAdd,
    PfCount,
    PfMerge,
    HSet,
    HMSet,
    HSetNx,
    HGet,
    HMGet,
    HGetAll,
    HDel,
    HExists,
    HKeys,
    HVals,
    HLen,
    HStrLen,
    HIncrBy,
    HIncrByFloat,
    HScan,
    HRandField,
    LPush,
    RPush,
    LPop,
    RPop,
    LLen,
    LIndex,
    LRange,
    LSet,
    LTrim,
    LInsert,
    LPos,
    LMove,
    LMPop,
    BLPop,
    BRPop,
    BLMPop,
    BRPopLPush,
    SAdd,
    SRem,
    SMembers,
    SCard,
    SMove,
    SPop,
    SInter,
    SUnion,
    SDiff,
    SScan,
    SIsMember,
    SDiffStore,
    SInterCard,
    SInterStore,
    SUnionStore,
    SRandMember,
    ZAdd,
    ZRem,
    ZCard,
    ZCount,
    ZScore,
    ZRank,
    ZRevRank,
    ZIncrBy,
    ZMScore,
    ZRange,
    ZRevRange,
    ZPopMin,
    ZPopMax,
    BZPopMin,
    BZPopMax,
    ZMPop,
    BZMPop,
    ZInter,
    ZUnion,
    ZDiff,
    ZScan,
    ZRandMember,
    ZRangeByScore,
    ZRevRangeByScore,
    ZRemRangeByRank,
    GeoAdd,
    GeoPos,
    GeoDist,
    GeoHash,
    GeoRadius,
    GeoRadiusRo,
    GeoSearch,
    GeoSearchStore,
    GeoRadiusByMember,
    GeoRadiusByMemberRo,
    XAdd,
    XLen,
    XDel,
    XRange,
    XRevRange,
    XTrim,
    XRead,
    XGroup,
    XAck,
    XClaim,
    XPending,
    XReadGroup,
    XAutoClaim,
    Acl,
    Config,
    Publish,
    Subscribe,
    Unsubscribe,
    PSubscribe,
    PUnsubscribe,
    PubSub,
    Multi,
    Exec,
    Discard,
    Watch,
    Unwatch,
}

impl CommandId {
    #[inline(always)]
    pub fn name(self) -> Option<&'static str> {
        Some(match self {
            Self::Unknown => return None,
            Self::Auth => "AUTH",
            Self::Hello => "HELLO",
            Self::Client => "CLIENT",
            Self::Command => "COMMAND",
            Self::Select => "SELECT",
            Self::Quit => "QUIT",
            Self::Ping => "PING",
            Self::Echo => "ECHO",
            Self::Eval => "EVAL",
            Self::EvalRo => "EVAL_RO",
            Self::EvalSha => "EVALSHA",
            Self::EvalShaRo => "EVALSHA_RO",
            Self::Script => "SCRIPT",
            Self::Del => "DEL",
            Self::Exists => "EXISTS",
            Self::Touch => "TOUCH",
            Self::Unlink => "UNLINK",
            Self::Type => "TYPE",
            Self::Rename => "RENAME",
            Self::RenameNx => "RENAMENX",
            Self::DbSize => "DBSIZE",
            Self::Keys => "KEYS",
            Self::Scan => "SCAN",
            Self::Move => "MOVE",
            Self::Dump => "DUMP",
            Self::Restore => "RESTORE",
            Self::Sort => "SORT",
            Self::Copy => "COPY",
            Self::FlushDb => "FLUSHDB",
            Self::FlushAll => "FLUSHALL",
            Self::Expire => "EXPIRE",
            Self::PExpire => "PEXPIRE",
            Self::ExpireAt => "EXPIREAT",
            Self::PExpireAt => "PEXPIREAT",
            Self::Persist => "PERSIST",
            Self::Ttl => "TTL",
            Self::PTtl => "PTTL",
            Self::Get => "GET",
            Self::Set => "SET",
            Self::SetNx => "SETNX",
            Self::GetSet => "GETSET",
            Self::GetDel => "GETDEL",
            Self::SetEx => "SETEX",
            Self::PSetEx => "PSETEX",
            Self::GetEx => "GETEX",
            Self::Append => "APPEND",
            Self::StrLen => "STRLEN",
            Self::SetRange => "SETRANGE",
            Self::GetRange => "GETRANGE",
            Self::MGet => "MGET",
            Self::MSet => "MSET",
            Self::MSetNx => "MSETNX",
            Self::Incr => "INCR",
            Self::IncrBy => "INCRBY",
            Self::Decr => "DECR",
            Self::DecrBy => "DECRBY",
            Self::SetBit => "SETBIT",
            Self::GetBit => "GETBIT",
            Self::BitCount => "BITCOUNT",
            Self::BitPos => "BITPOS",
            Self::BitOp => "BITOP",
            Self::BitField => "BITFIELD",
            Self::BitFieldRo => "BITFIELD_RO",
            Self::PfAdd => "PFADD",
            Self::PfCount => "PFCOUNT",
            Self::PfMerge => "PFMERGE",
            Self::HSet => "HSET",
            Self::HMSet => "HMSET",
            Self::HSetNx => "HSETNX",
            Self::HGet => "HGET",
            Self::HMGet => "HMGET",
            Self::HGetAll => "HGETALL",
            Self::HDel => "HDEL",
            Self::HExists => "HEXISTS",
            Self::HKeys => "HKEYS",
            Self::HVals => "HVALS",
            Self::HLen => "HLEN",
            Self::HStrLen => "HSTRLEN",
            Self::HIncrBy => "HINCRBY",
            Self::HIncrByFloat => "HINCRBYFLOAT",
            Self::HScan => "HSCAN",
            Self::HRandField => "HRANDFIELD",
            Self::LPush => "LPUSH",
            Self::RPush => "RPUSH",
            Self::LPop => "LPOP",
            Self::RPop => "RPOP",
            Self::LLen => "LLEN",
            Self::LIndex => "LINDEX",
            Self::LRange => "LRANGE",
            Self::LSet => "LSET",
            Self::LTrim => "LTRIM",
            Self::LInsert => "LINSERT",
            Self::LPos => "LPOS",
            Self::LMove => "LMOVE",
            Self::LMPop => "LMPOP",
            Self::BLPop => "BLPOP",
            Self::BRPop => "BRPOP",
            Self::BLMPop => "BLMPOP",
            Self::BRPopLPush => "BRPOPLPUSH",
            Self::SAdd => "SADD",
            Self::SRem => "SREM",
            Self::SMembers => "SMEMBERS",
            Self::SCard => "SCARD",
            Self::SMove => "SMOVE",
            Self::SPop => "SPOP",
            Self::SInter => "SINTER",
            Self::SUnion => "SUNION",
            Self::SDiff => "SDIFF",
            Self::SScan => "SSCAN",
            Self::SIsMember => "SISMEMBER",
            Self::SDiffStore => "SDIFFSTORE",
            Self::SInterCard => "SINTERCARD",
            Self::SInterStore => "SINTERSTORE",
            Self::SUnionStore => "SUNIONSTORE",
            Self::SRandMember => "SRANDMEMBER",
            Self::ZAdd => "ZADD",
            Self::ZRem => "ZREM",
            Self::ZCard => "ZCARD",
            Self::ZCount => "ZCOUNT",
            Self::ZScore => "ZSCORE",
            Self::ZRank => "ZRANK",
            Self::ZRevRank => "ZREVRANK",
            Self::ZIncrBy => "ZINCRBY",
            Self::ZMScore => "ZMSCORE",
            Self::ZRange => "ZRANGE",
            Self::ZRevRange => "ZREVRANGE",
            Self::ZPopMin => "ZPOPMIN",
            Self::ZPopMax => "ZPOPMAX",
            Self::BZPopMin => "BZPOPMIN",
            Self::BZPopMax => "BZPOPMAX",
            Self::ZMPop => "ZMPOP",
            Self::BZMPop => "BZMPOP",
            Self::ZInter => "ZINTER",
            Self::ZUnion => "ZUNION",
            Self::ZDiff => "ZDIFF",
            Self::ZScan => "ZSCAN",
            Self::ZRandMember => "ZRANDMEMBER",
            Self::ZRangeByScore => "ZRANGEBYSCORE",
            Self::ZRevRangeByScore => "ZREVRANGEBYSCORE",
            Self::ZRemRangeByRank => "ZREMRANGEBYRANK",
            Self::GeoAdd => "GEOADD",
            Self::GeoPos => "GEOPOS",
            Self::GeoDist => "GEODIST",
            Self::GeoHash => "GEOHASH",
            Self::GeoRadius => "GEORADIUS",
            Self::GeoRadiusRo => "GEORADIUS_RO",
            Self::GeoSearch => "GEOSEARCH",
            Self::GeoSearchStore => "GEOSEARCHSTORE",
            Self::GeoRadiusByMember => "GEORADIUSBYMEMBER",
            Self::GeoRadiusByMemberRo => "GEORADIUSBYMEMBER_RO",
            Self::XAdd => "XADD",
            Self::XLen => "XLEN",
            Self::XDel => "XDEL",
            Self::XRange => "XRANGE",
            Self::XRevRange => "XREVRANGE",
            Self::XTrim => "XTRIM",
            Self::XRead => "XREAD",
            Self::XGroup => "XGROUP",
            Self::XAck => "XACK",
            Self::XClaim => "XCLAIM",
            Self::XPending => "XPENDING",
            Self::XReadGroup => "XREADGROUP",
            Self::XAutoClaim => "XAUTOCLAIM",
            Self::Acl => "ACL",
            Self::Config => "CONFIG",
            Self::Publish => "PUBLISH",
            Self::Subscribe => "SUBSCRIBE",
            Self::Unsubscribe => "UNSUBSCRIBE",
            Self::PSubscribe => "PSUBSCRIBE",
            Self::PUnsubscribe => "PUNSUBSCRIBE",
            Self::PubSub => "PUBSUB",
            Self::Multi => "MULTI",
            Self::Exec => "EXEC",
            Self::Discard => "DISCARD",
            Self::Watch => "WATCH",
            Self::Unwatch => "UNWATCH",
        })
    }
}

#[inline(always)]
pub fn identify(command: &[u8]) -> CommandId {
    if command.len() <= 8 {
        return identify_short(pack_runtime(command));
    }

    identify_long(command)
}

#[inline(always)]
fn identify_short(key: u64) -> CommandId {
    match key {
        cmd::AUTH => CommandId::Auth,
        cmd::HELLO => CommandId::Hello,
        cmd::CLIENT => CommandId::Client,
        cmd::COMMAND => CommandId::Command,
        cmd::SELECT => CommandId::Select,
        cmd::QUIT => CommandId::Quit,
        cmd::PING => CommandId::Ping,
        cmd::ECHO => CommandId::Echo,
        cmd::EVAL => CommandId::Eval,
        cmd::EVAL_RO => CommandId::EvalRo,
        cmd::EVALSHA => CommandId::EvalSha,
        cmd::SCRIPT => CommandId::Script,
        cmd::DEL => CommandId::Del,
        cmd::EXISTS => CommandId::Exists,
        cmd::TOUCH => CommandId::Touch,
        cmd::UNLINK => CommandId::Unlink,
        cmd::TYPE => CommandId::Type,
        cmd::RENAME => CommandId::Rename,
        cmd::RENAMENX => CommandId::RenameNx,
        cmd::DBSIZE => CommandId::DbSize,
        cmd::KEYS => CommandId::Keys,
        cmd::SCAN => CommandId::Scan,
        cmd::MOVE => CommandId::Move,
        cmd::DUMP => CommandId::Dump,
        cmd::RESTORE => CommandId::Restore,
        cmd::SORT => CommandId::Sort,
        cmd::COPY => CommandId::Copy,
        cmd::FLUSHDB => CommandId::FlushDb,
        cmd::FLUSHALL => CommandId::FlushAll,
        cmd::EXPIRE => CommandId::Expire,
        cmd::PEXPIRE => CommandId::PExpire,
        cmd::EXPIREAT => CommandId::ExpireAt,
        cmd::PERSIST => CommandId::Persist,
        cmd::TTL => CommandId::Ttl,
        cmd::PTTL => CommandId::PTtl,
        cmd::GET => CommandId::Get,
        cmd::SET => CommandId::Set,
        cmd::SETNX => CommandId::SetNx,
        cmd::GETSET => CommandId::GetSet,
        cmd::GETDEL => CommandId::GetDel,
        cmd::SETEX => CommandId::SetEx,
        cmd::PSETEX => CommandId::PSetEx,
        cmd::GETEX => CommandId::GetEx,
        cmd::APPEND => CommandId::Append,
        cmd::STRLEN => CommandId::StrLen,
        cmd::SETRANGE => CommandId::SetRange,
        cmd::GETRANGE => CommandId::GetRange,
        cmd::MGET => CommandId::MGet,
        cmd::MSET => CommandId::MSet,
        cmd::MSETNX => CommandId::MSetNx,
        cmd::INCR => CommandId::Incr,
        cmd::INCRBY => CommandId::IncrBy,
        cmd::DECR => CommandId::Decr,
        cmd::DECRBY => CommandId::DecrBy,
        cmd::SETBIT => CommandId::SetBit,
        cmd::GETBIT => CommandId::GetBit,
        cmd::BITCOUNT => CommandId::BitCount,
        cmd::BITPOS => CommandId::BitPos,
        cmd::BITOP => CommandId::BitOp,
        cmd::BITFIELD => CommandId::BitField,
        cmd::PFADD => CommandId::PfAdd,
        cmd::PFCOUNT => CommandId::PfCount,
        cmd::PFMERGE => CommandId::PfMerge,
        cmd::HSET => CommandId::HSet,
        cmd::HMSET => CommandId::HMSet,
        cmd::HSETNX => CommandId::HSetNx,
        cmd::HGET => CommandId::HGet,
        cmd::HMGET => CommandId::HMGet,
        cmd::HGETALL => CommandId::HGetAll,
        cmd::HDEL => CommandId::HDel,
        cmd::HEXISTS => CommandId::HExists,
        cmd::HKEYS => CommandId::HKeys,
        cmd::HVALS => CommandId::HVals,
        cmd::HLEN => CommandId::HLen,
        cmd::HSTRLEN => CommandId::HStrLen,
        cmd::HINCRBY => CommandId::HIncrBy,
        cmd::HSCAN => CommandId::HScan,
        cmd::LPUSH => CommandId::LPush,
        cmd::RPUSH => CommandId::RPush,
        cmd::LPOP => CommandId::LPop,
        cmd::RPOP => CommandId::RPop,
        cmd::LLEN => CommandId::LLen,
        cmd::LINDEX => CommandId::LIndex,
        cmd::LRANGE => CommandId::LRange,
        cmd::LSET => CommandId::LSet,
        cmd::LTRIM => CommandId::LTrim,
        cmd::LINSERT => CommandId::LInsert,
        cmd::LPOS => CommandId::LPos,
        cmd::LMOVE => CommandId::LMove,
        cmd::LMPOP => CommandId::LMPop,
        cmd::BLPOP => CommandId::BLPop,
        cmd::BRPOP => CommandId::BRPop,
        cmd::BLMPOP => CommandId::BLMPop,
        cmd::SADD => CommandId::SAdd,
        cmd::SREM => CommandId::SRem,
        cmd::SMEMBERS => CommandId::SMembers,
        cmd::SCARD => CommandId::SCard,
        cmd::SMOVE => CommandId::SMove,
        cmd::SPOP => CommandId::SPop,
        cmd::SINTER => CommandId::SInter,
        cmd::SUNION => CommandId::SUnion,
        cmd::SDIFF => CommandId::SDiff,
        cmd::SSCAN => CommandId::SScan,
        cmd::ZADD => CommandId::ZAdd,
        cmd::ZREM => CommandId::ZRem,
        cmd::ZCARD => CommandId::ZCard,
        cmd::ZCOUNT => CommandId::ZCount,
        cmd::ZSCORE => CommandId::ZScore,
        cmd::ZRANK => CommandId::ZRank,
        cmd::ZREVRANK => CommandId::ZRevRank,
        cmd::ZINCRBY => CommandId::ZIncrBy,
        cmd::ZMSCORE => CommandId::ZMScore,
        cmd::ZRANGE => CommandId::ZRange,
        cmd::ZPOPMIN => CommandId::ZPopMin,
        cmd::ZPOPMAX => CommandId::ZPopMax,
        cmd::BZPOPMIN => CommandId::BZPopMin,
        cmd::BZPOPMAX => CommandId::BZPopMax,
        cmd::ZMPOP => CommandId::ZMPop,
        cmd::BZMPOP => CommandId::BZMPop,
        cmd::ZINTER => CommandId::ZInter,
        cmd::ZUNION => CommandId::ZUnion,
        cmd::ZDIFF => CommandId::ZDiff,
        cmd::ZSCAN => CommandId::ZScan,
        cmd::GEOADD => CommandId::GeoAdd,
        cmd::GEOPOS => CommandId::GeoPos,
        cmd::GEODIST => CommandId::GeoDist,
        cmd::GEOHASH => CommandId::GeoHash,
        cmd::XADD => CommandId::XAdd,
        cmd::XLEN => CommandId::XLen,
        cmd::XDEL => CommandId::XDel,
        cmd::XRANGE => CommandId::XRange,
        cmd::XTRIM => CommandId::XTrim,
        cmd::XREAD => CommandId::XRead,
        cmd::XGROUP => CommandId::XGroup,
        cmd::XACK => CommandId::XAck,
        cmd::XCLAIM => CommandId::XClaim,
        cmd::XPENDING => CommandId::XPending,
        cmd::MULTI => CommandId::Multi,
        cmd::EXEC => CommandId::Exec,
        cmd::WATCH => CommandId::Watch,
        cmd::PUBLISH => CommandId::Publish,
        cmd::PUBSUB => CommandId::PubSub,
        cmd::ACL => CommandId::Acl,
        cmd::CONFIG => CommandId::Config,
        cmd::DISCARD => CommandId::Discard,
        cmd::UNWATCH => CommandId::Unwatch,
        _ => CommandId::Unknown,
    }
}

#[cold]
fn identify_long(command: &[u8]) -> CommandId {
    match command.len() {
        9 => {
            if command.eq_ignore_ascii_case(b"PEXPIREAT") {
                return CommandId::PExpireAt;
            }
            if command.eq_ignore_ascii_case(b"SISMEMBER") {
                return CommandId::SIsMember;
            }
            if command.eq_ignore_ascii_case(b"ZREVRANGE") {
                return CommandId::ZRevRange;
            }
            if command.eq_ignore_ascii_case(b"GEORADIUS") {
                return CommandId::GeoRadius;
            }
            if command.eq_ignore_ascii_case(b"GEOSEARCH") {
                return CommandId::GeoSearch;
            }
            if command.eq_ignore_ascii_case(b"XREVRANGE") {
                return CommandId::XRevRange;
            }
            if command.eq_ignore_ascii_case(b"SUBSCRIBE") {
                return CommandId::Subscribe;
            }
        }
        10 => {
            if command.eq_ignore_ascii_case(b"HRANDFIELD") {
                return CommandId::HRandField;
            }
            if command.eq_ignore_ascii_case(b"BRPOPLPUSH") {
                return CommandId::BRPopLPush;
            }
            if command.eq_ignore_ascii_case(b"SDIFFSTORE") {
                return CommandId::SDiffStore;
            }
            if command.eq_ignore_ascii_case(b"SINTERCARD") {
                return CommandId::SInterCard;
            }
            if command.eq_ignore_ascii_case(b"EVALSHA_RO") {
                return CommandId::EvalShaRo;
            }
            if command.eq_ignore_ascii_case(b"XREADGROUP") {
                return CommandId::XReadGroup;
            }
            if command.eq_ignore_ascii_case(b"XAUTOCLAIM") {
                return CommandId::XAutoClaim;
            }
            if command.eq_ignore_ascii_case(b"PSUBSCRIBE") {
                return CommandId::PSubscribe;
            }
        }
        11 => {
            if command.eq_ignore_ascii_case(b"BITFIELD_RO") {
                return CommandId::BitFieldRo;
            }
            if command.eq_ignore_ascii_case(b"SINTERSTORE") {
                return CommandId::SInterStore;
            }
            if command.eq_ignore_ascii_case(b"SUNIONSTORE") {
                return CommandId::SUnionStore;
            }
            if command.eq_ignore_ascii_case(b"SRANDMEMBER") {
                return CommandId::SRandMember;
            }
            if command.eq_ignore_ascii_case(b"ZRANDMEMBER") {
                return CommandId::ZRandMember;
            }
            if command.eq_ignore_ascii_case(b"UNSUBSCRIBE") {
                return CommandId::Unsubscribe;
            }
        }
        12 => {
            if command.eq_ignore_ascii_case(b"HINCRBYFLOAT") {
                return CommandId::HIncrByFloat;
            }
            if command.eq_ignore_ascii_case(b"GEORADIUS_RO") {
                return CommandId::GeoRadiusRo;
            }
            if command.eq_ignore_ascii_case(b"PUNSUBSCRIBE") {
                return CommandId::PUnsubscribe;
            }
        }
        13 => {
            if command.eq_ignore_ascii_case(b"ZRANGEBYSCORE") {
                return CommandId::ZRangeByScore;
            }
            if command.eq_ignore_ascii_case(b"GEOSEARCHSTORE") {
                return CommandId::GeoSearchStore;
            }
        }
        15 => {
            if command.eq_ignore_ascii_case(b"ZREMRANGEBYRANK") {
                return CommandId::ZRemRangeByRank;
            }
        }
        16 => {
            if command.eq_ignore_ascii_case(b"ZREVRANGEBYSCORE") {
                return CommandId::ZRevRangeByScore;
            }
        }
        17 => {
            if command.eq_ignore_ascii_case(b"GEORADIUSBYMEMBER") {
                return CommandId::GeoRadiusByMember;
            }
        }
        20 => {
            if command.eq_ignore_ascii_case(b"GEORADIUSBYMEMBER_RO") {
                return CommandId::GeoRadiusByMemberRo;
            }
        }
        _ => {}
    }

    CommandId::Unknown
}
