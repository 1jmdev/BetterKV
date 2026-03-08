use commands::util::{cmd, pack_runtime};
use types::value::CompactArg;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum AclCategory {
    All,
    Admin,
    Bitmap,
    Blocking,
    Connection,
    Dangerous,
    Fast,
    Geo,
    Hash,
    HyperLogLog,
    Keyspace,
    List,
    PubSub,
    Read,
    Scripting,
    Set,
    Slow,
    SortedSet,
    Stream,
    String,
    Write,
}

#[derive(Clone, Copy)]
pub(super) struct CommandSpec {
    pub(super) name: &'static str,
    pub(super) categories: &'static [AclCategory],
    key_extractor: KeyExtractor,
    channel_extractor: ChannelExtractor,
}

impl CommandSpec {
    const fn new(
        name: &'static str,
        categories: &'static [AclCategory],
        key_extractor: KeyExtractor,
        channel_extractor: ChannelExtractor,
    ) -> Self {
        Self {
            name,
            categories,
            key_extractor,
            channel_extractor,
        }
    }

    pub(super) fn unknown() -> Self {
        Self::new(
            "UNKNOWN",
            UNKNOWN_CATEGORIES,
            KeyExtractor::None,
            ChannelExtractor::None,
        )
    }

    pub(super) fn keys<'a>(&self, args: &'a [CompactArg]) -> Vec<&'a [u8]> {
        extract_keys(self.key_extractor, args)
    }

    pub(super) fn channels<'a>(&self, args: &'a [CompactArg]) -> Vec<&'a [u8]> {
        extract_channels(self.channel_extractor, args)
    }
}

#[derive(Clone, Copy)]
enum KeyExtractor {
    None,
    Single,
    Pair,
    AllFrom(usize),
    AllExceptLastFrom(usize),
    EveryOtherFrom(usize),
    Counted {
        count_index: usize,
        first_key: usize,
    },
    EvalStyle,
    XReadStyle,
    SortStore,
}

#[derive(Clone, Copy)]
enum ChannelExtractor {
    None,
    First,
    AllFrom(usize),
}

impl AclCategory {
    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::All => "all",
            Self::Admin => "admin",
            Self::Bitmap => "bitmap",
            Self::Blocking => "blocking",
            Self::Connection => "connection",
            Self::Dangerous => "dangerous",
            Self::Fast => "fast",
            Self::Geo => "geo",
            Self::Hash => "hash",
            Self::HyperLogLog => "hyperloglog",
            Self::Keyspace => "keyspace",
            Self::List => "list",
            Self::PubSub => "pubsub",
            Self::Read => "read",
            Self::Scripting => "scripting",
            Self::Set => "set",
            Self::Slow => "slow",
            Self::SortedSet => "sortedset",
            Self::Stream => "stream",
            Self::String => "string",
            Self::Write => "write",
        }
    }

    pub(super) fn parse(value: &str) -> Result<Self, String> {
        let _trace = profiler::scope("server::auth::parse_acl_category");
        match value.to_ascii_lowercase().as_str() {
            "all" => Ok(Self::All),
            "admin" => Ok(Self::Admin),
            "bitmap" => Ok(Self::Bitmap),
            "blocking" => Ok(Self::Blocking),
            "connection" => Ok(Self::Connection),
            "dangerous" => Ok(Self::Dangerous),
            "fast" => Ok(Self::Fast),
            "geo" => Ok(Self::Geo),
            "hash" => Ok(Self::Hash),
            "hyperloglog" => Ok(Self::HyperLogLog),
            "keyspace" => Ok(Self::Keyspace),
            "list" => Ok(Self::List),
            "pubsub" => Ok(Self::PubSub),
            "read" => Ok(Self::Read),
            "scripting" => Ok(Self::Scripting),
            "set" => Ok(Self::Set),
            "slow" => Ok(Self::Slow),
            "sortedset" => Ok(Self::SortedSet),
            "stream" => Ok(Self::Stream),
            "string" => Ok(Self::String),
            "write" => Ok(Self::Write),
            _ => Err(format!("ERR Unknown ACL category '{value}'")),
        }
    }
}

pub(super) fn acl_categories() -> &'static [AclCategory] {
    &[
        AclCategory::Admin,
        AclCategory::Bitmap,
        AclCategory::Blocking,
        AclCategory::Connection,
        AclCategory::Dangerous,
        AclCategory::Fast,
        AclCategory::Geo,
        AclCategory::Hash,
        AclCategory::HyperLogLog,
        AclCategory::Keyspace,
        AclCategory::List,
        AclCategory::PubSub,
        AclCategory::Read,
        AclCategory::Scripting,
        AclCategory::Set,
        AclCategory::Slow,
        AclCategory::SortedSet,
        AclCategory::Stream,
        AclCategory::String,
        AclCategory::Write,
    ]
}

pub(super) fn commands_in_category(category: AclCategory) -> Vec<&'static str> {
    all_specs()
        .iter()
        .copied()
        .filter(|spec| spec.categories.contains(&category))
        .map(|spec| spec.name)
        .collect()
}

pub(super) fn command_spec(command: &[u8]) -> Option<CommandSpec> {
    if command.len() <= 8 {
        let key = pack_runtime(command);
        return match key {
            cmd::AUTH => spec_by_name("AUTH"),
            cmd::HELLO => spec_by_name("HELLO"),
            cmd::CLIENT => spec_by_name("CLIENT"),
            cmd::COMMAND => spec_by_name("COMMAND"),
            cmd::SELECT => spec_by_name("SELECT"),
            cmd::QUIT => spec_by_name("QUIT"),
            cmd::PING => spec_by_name("PING"),
            cmd::ECHO => spec_by_name("ECHO"),
            cmd::GET => spec_by_name("GET"),
            cmd::SET => spec_by_name("SET"),
            cmd::INCR => spec_by_name("INCR"),
            cmd::DEL => spec_by_name("DEL"),
            cmd::EXPIRE => spec_by_name("EXPIRE"),
            cmd::PEXPIRE => spec_by_name("PEXPIRE"),
            cmd::EXPIREAT => spec_by_name("EXPIREAT"),
            cmd::PERSIST => spec_by_name("PERSIST"),
            cmd::TTL => spec_by_name("TTL"),
            cmd::PTTL => spec_by_name("PTTL"),
            cmd::SETNX => spec_by_name("SETNX"),
            cmd::GETSET => spec_by_name("GETSET"),
            cmd::GETDEL => spec_by_name("GETDEL"),
            cmd::SETEX => spec_by_name("SETEX"),
            cmd::PSETEX => spec_by_name("PSETEX"),
            cmd::GETEX => spec_by_name("GETEX"),
            cmd::APPEND => spec_by_name("APPEND"),
            cmd::STRLEN => spec_by_name("STRLEN"),
            cmd::SETRANGE => spec_by_name("SETRANGE"),
            cmd::GETRANGE => spec_by_name("GETRANGE"),
            cmd::MGET => spec_by_name("MGET"),
            cmd::MSET => spec_by_name("MSET"),
            cmd::MSETNX => spec_by_name("MSETNX"),
            cmd::INCRBY => spec_by_name("INCRBY"),
            cmd::DECR => spec_by_name("DECR"),
            cmd::DECRBY => spec_by_name("DECRBY"),
            cmd::SETBIT => spec_by_name("SETBIT"),
            cmd::GETBIT => spec_by_name("GETBIT"),
            cmd::BITCOUNT => spec_by_name("BITCOUNT"),
            cmd::BITPOS => spec_by_name("BITPOS"),
            cmd::BITOP => spec_by_name("BITOP"),
            cmd::BITFIELD => spec_by_name("BITFIELD"),
            cmd::PFADD => spec_by_name("PFADD"),
            cmd::PFCOUNT => spec_by_name("PFCOUNT"),
            cmd::PFMERGE => spec_by_name("PFMERGE"),
            cmd::HSET => spec_by_name("HSET"),
            cmd::HMSET => spec_by_name("HMSET"),
            cmd::HSETNX => spec_by_name("HSETNX"),
            cmd::HGET => spec_by_name("HGET"),
            cmd::HMGET => spec_by_name("HMGET"),
            cmd::HGETALL => spec_by_name("HGETALL"),
            cmd::HDEL => spec_by_name("HDEL"),
            cmd::HEXISTS => spec_by_name("HEXISTS"),
            cmd::HKEYS => spec_by_name("HKEYS"),
            cmd::HVALS => spec_by_name("HVALS"),
            cmd::HLEN => spec_by_name("HLEN"),
            cmd::HSTRLEN => spec_by_name("HSTRLEN"),
            cmd::HINCRBY => spec_by_name("HINCRBY"),
            cmd::HSCAN => spec_by_name("HSCAN"),
            cmd::LPUSH => spec_by_name("LPUSH"),
            cmd::RPUSH => spec_by_name("RPUSH"),
            cmd::LPOP => spec_by_name("LPOP"),
            cmd::RPOP => spec_by_name("RPOP"),
            cmd::LLEN => spec_by_name("LLEN"),
            cmd::LINDEX => spec_by_name("LINDEX"),
            cmd::LRANGE => spec_by_name("LRANGE"),
            cmd::LSET => spec_by_name("LSET"),
            cmd::LTRIM => spec_by_name("LTRIM"),
            cmd::LINSERT => spec_by_name("LINSERT"),
            cmd::LPOS => spec_by_name("LPOS"),
            cmd::LMOVE => spec_by_name("LMOVE"),
            cmd::LMPOP => spec_by_name("LMPOP"),
            cmd::BLPOP => spec_by_name("BLPOP"),
            cmd::BRPOP => spec_by_name("BRPOP"),
            cmd::BLMPOP => spec_by_name("BLMPOP"),
            cmd::SADD => spec_by_name("SADD"),
            cmd::SREM => spec_by_name("SREM"),
            cmd::SCARD => spec_by_name("SCARD"),
            cmd::SMOVE => spec_by_name("SMOVE"),
            cmd::SPOP => spec_by_name("SPOP"),
            cmd::SINTER => spec_by_name("SINTER"),
            cmd::SDIFF => spec_by_name("SDIFF"),
            cmd::SSCAN => spec_by_name("SSCAN"),
            cmd::ZADD => spec_by_name("ZADD"),
            cmd::ZREM => spec_by_name("ZREM"),
            cmd::ZCARD => spec_by_name("ZCARD"),
            cmd::ZCOUNT => spec_by_name("ZCOUNT"),
            cmd::ZSCORE => spec_by_name("ZSCORE"),
            cmd::ZRANK => spec_by_name("ZRANK"),
            cmd::ZREVRANK => spec_by_name("ZREVRANK"),
            cmd::ZINCRBY => spec_by_name("ZINCRBY"),
            cmd::ZMSCORE => spec_by_name("ZMSCORE"),
            cmd::ZRANGE => spec_by_name("ZRANGE"),
            cmd::ZPOPMIN => spec_by_name("ZPOPMIN"),
            cmd::ZPOPMAX => spec_by_name("ZPOPMAX"),
            cmd::BZPOPMIN => spec_by_name("BZPOPMIN"),
            cmd::BZPOPMAX => spec_by_name("BZPOPMAX"),
            cmd::ZMPOP => spec_by_name("ZMPOP"),
            cmd::BZMPOP => spec_by_name("BZMPOP"),
            cmd::ZINTER => spec_by_name("ZINTER"),
            cmd::ZUNION => spec_by_name("ZUNION"),
            cmd::ZDIFF => spec_by_name("ZDIFF"),
            cmd::ZSCAN => spec_by_name("ZSCAN"),
            cmd::GEOADD => spec_by_name("GEOADD"),
            cmd::GEOPOS => spec_by_name("GEOPOS"),
            cmd::GEODIST => spec_by_name("GEODIST"),
            cmd::GEOHASH => spec_by_name("GEOHASH"),
            cmd::XADD => spec_by_name("XADD"),
            cmd::XLEN => spec_by_name("XLEN"),
            cmd::XDEL => spec_by_name("XDEL"),
            cmd::XRANGE => spec_by_name("XRANGE"),
            cmd::XTRIM => spec_by_name("XTRIM"),
            cmd::XREAD => spec_by_name("XREAD"),
            cmd::XGROUP => spec_by_name("XGROUP"),
            cmd::XACK => spec_by_name("XACK"),
            cmd::XCLAIM => spec_by_name("XCLAIM"),
            cmd::XPENDING => spec_by_name("XPENDING"),
            cmd::EXISTS => spec_by_name("EXISTS"),
            cmd::TOUCH => spec_by_name("TOUCH"),
            cmd::UNLINK => spec_by_name("UNLINK"),
            cmd::TYPE => spec_by_name("TYPE"),
            cmd::RENAME => spec_by_name("RENAME"),
            cmd::RENAMENX => spec_by_name("RENAMENX"),
            cmd::DBSIZE => spec_by_name("DBSIZE"),
            cmd::KEYS => spec_by_name("KEYS"),
            cmd::SCAN => spec_by_name("SCAN"),
            cmd::MOVE => spec_by_name("MOVE"),
            cmd::DUMP => spec_by_name("DUMP"),
            cmd::RESTORE => spec_by_name("RESTORE"),
            cmd::SORT => spec_by_name("SORT"),
            cmd::COPY => spec_by_name("COPY"),
            cmd::FLUSHDB => spec_by_name("FLUSHDB"),
            cmd::FLUSHALL => spec_by_name("FLUSHALL"),
            cmd::EVAL => spec_by_name("EVAL"),
            cmd::EVAL_RO => spec_by_name("EVAL_RO"),
            cmd::EVALSHA => spec_by_name("EVALSHA"),
            cmd::SCRIPT => spec_by_name("SCRIPT"),
            _ => None,
        };
    }

    spec_by_bytes(command)
}

#[inline(always)]
fn spec_by_name(name: &str) -> Option<CommandSpec> {
    all_specs().iter().copied().find(|spec| spec.name == name)
}

fn spec_by_bytes(command: &[u8]) -> Option<CommandSpec> {
    match command.len() {
        9 => {
            if command.eq_ignore_ascii_case(b"PEXPIREAT") {
                return spec_by_name("PEXPIREAT");
            }
            if command.eq_ignore_ascii_case(b"SISMEMBER") {
                return spec_by_name("SISMEMBER");
            }
            if command.eq_ignore_ascii_case(b"ZREVRANGE") {
                return spec_by_name("ZREVRANGE");
            }
            if command.eq_ignore_ascii_case(b"GEORADIUS") {
                return spec_by_name("GEORADIUS");
            }
            if command.eq_ignore_ascii_case(b"GEOSEARCH") {
                return spec_by_name("GEOSEARCH");
            }
            if command.eq_ignore_ascii_case(b"XREVRANGE") {
                return spec_by_name("XREVRANGE");
            }
            if command.eq_ignore_ascii_case(b"XREADGROUP") {
                return spec_by_name("XREADGROUP");
            }
            if command.eq_ignore_ascii_case(b"XAUTOCLAIM") {
                return spec_by_name("XAUTOCLAIM");
            }
        }
        10 => {
            if command.eq_ignore_ascii_case(b"HRANDFIELD") {
                return spec_by_name("HRANDFIELD");
            }
            if command.eq_ignore_ascii_case(b"BRPOPLPUSH") {
                return spec_by_name("BRPOPLPUSH");
            }
            if command.eq_ignore_ascii_case(b"SDIFFSTORE") {
                return spec_by_name("SDIFFSTORE");
            }
            if command.eq_ignore_ascii_case(b"SINTERCARD") {
                return spec_by_name("SINTERCARD");
            }
            if command.eq_ignore_ascii_case(b"EVALSHA_RO") {
                return spec_by_name("EVALSHA_RO");
            }
        }
        11 => {
            if command.eq_ignore_ascii_case(b"BITFIELD_RO") {
                return spec_by_name("BITFIELD_RO");
            }
            if command.eq_ignore_ascii_case(b"SINTERSTORE") {
                return spec_by_name("SINTERSTORE");
            }
            if command.eq_ignore_ascii_case(b"SUNIONSTORE") {
                return spec_by_name("SUNIONSTORE");
            }
            if command.eq_ignore_ascii_case(b"SRANDMEMBER") {
                return spec_by_name("SRANDMEMBER");
            }
            if command.eq_ignore_ascii_case(b"ZRANDMEMBER") {
                return spec_by_name("ZRANDMEMBER");
            }
        }
        12 => {
            if command.eq_ignore_ascii_case(b"HINCRBYFLOAT") {
                return spec_by_name("HINCRBYFLOAT");
            }
            if command.eq_ignore_ascii_case(b"GEORADIUS_RO") {
                return spec_by_name("GEORADIUS_RO");
            }
        }
        13 => {
            if command.eq_ignore_ascii_case(b"ZRANGEBYSCORE") {
                return spec_by_name("ZRANGEBYSCORE");
            }
            if command.eq_ignore_ascii_case(b"GEOSEARCHSTORE") {
                return spec_by_name("GEOSEARCHSTORE");
            }
        }
        14 => {
            if command.eq_ignore_ascii_case(b"ZREMRANGEBYRANK") {
                return spec_by_name("ZREMRANGEBYRANK");
            }
        }
        16 => {
            if command.eq_ignore_ascii_case(b"ZREVRANGEBYSCORE") {
                return spec_by_name("ZREVRANGEBYSCORE");
            }
        }
        17 => {
            if command.eq_ignore_ascii_case(b"GEORADIUSBYMEMBER") {
                return spec_by_name("GEORADIUSBYMEMBER");
            }
        }
        20 => {
            if command.eq_ignore_ascii_case(b"GEORADIUSBYMEMBER_RO") {
                return spec_by_name("GEORADIUSBYMEMBER_RO");
            }
        }
        _ => {}
    }

    None
}

const CONN: &[AclCategory] = &[AclCategory::Connection, AclCategory::Fast];
const ADMIN: &[AclCategory] = &[AclCategory::Admin, AclCategory::Slow];
const ADMIN_DANGEROUS: &[AclCategory] = &[
    AclCategory::Admin,
    AclCategory::Dangerous,
    AclCategory::Slow,
];
const PUBSUB_READ: &[AclCategory] = &[AclCategory::PubSub, AclCategory::Read, AclCategory::Fast];
const PUBSUB_WRITE: &[AclCategory] = &[AclCategory::PubSub, AclCategory::Write, AclCategory::Fast];
const UNKNOWN_CATEGORIES: &[AclCategory] = &[AclCategory::Slow];

fn all_specs() -> &'static [CommandSpec] {
    static ALL_SPECS: &[CommandSpec] = &[
        CommandSpec::new("AUTH", CONN, KeyExtractor::None, ChannelExtractor::None),
        CommandSpec::new("HELLO", CONN, KeyExtractor::None, ChannelExtractor::None),
        CommandSpec::new("CLIENT", CONN, KeyExtractor::None, ChannelExtractor::None),
        CommandSpec::new("COMMAND", CONN, KeyExtractor::None, ChannelExtractor::None),
        CommandSpec::new("SELECT", CONN, KeyExtractor::None, ChannelExtractor::None),
        CommandSpec::new("QUIT", CONN, KeyExtractor::None, ChannelExtractor::None),
        CommandSpec::new("PING", CONN, KeyExtractor::None, ChannelExtractor::None),
        CommandSpec::new("ECHO", CONN, KeyExtractor::None, ChannelExtractor::None),
        CommandSpec::new("ACL", ADMIN, KeyExtractor::None, ChannelExtractor::None),
        CommandSpec::new("CONFIG", ADMIN, KeyExtractor::None, ChannelExtractor::None),
        CommandSpec::new(
            "GET",
            &[AclCategory::Read, AclCategory::Fast, AclCategory::String],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "SET",
            &[AclCategory::Write, AclCategory::Slow, AclCategory::String],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "INCR",
            &[AclCategory::Write, AclCategory::Fast, AclCategory::String],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "DEL",
            &[AclCategory::Write, AclCategory::Slow, AclCategory::Keyspace],
            KeyExtractor::AllFrom(1),
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "EXPIRE",
            &[AclCategory::Write, AclCategory::Fast, AclCategory::Keyspace],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "PEXPIRE",
            &[AclCategory::Write, AclCategory::Fast, AclCategory::Keyspace],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "EXPIREAT",
            &[AclCategory::Write, AclCategory::Fast, AclCategory::Keyspace],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "PEXPIREAT",
            &[AclCategory::Write, AclCategory::Fast, AclCategory::Keyspace],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "PERSIST",
            &[AclCategory::Write, AclCategory::Fast, AclCategory::Keyspace],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "TTL",
            &[AclCategory::Read, AclCategory::Fast, AclCategory::Keyspace],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "PTTL",
            &[AclCategory::Read, AclCategory::Fast, AclCategory::Keyspace],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "SETNX",
            &[AclCategory::Write, AclCategory::Slow, AclCategory::String],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "GETSET",
            &[AclCategory::Write, AclCategory::Slow, AclCategory::String],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "GETDEL",
            &[AclCategory::Write, AclCategory::Slow, AclCategory::String],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "SETEX",
            &[AclCategory::Write, AclCategory::Slow, AclCategory::String],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "PSETEX",
            &[AclCategory::Write, AclCategory::Slow, AclCategory::String],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "GETEX",
            &[AclCategory::Write, AclCategory::Slow, AclCategory::String],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "APPEND",
            &[AclCategory::Write, AclCategory::Fast, AclCategory::String],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "STRLEN",
            &[AclCategory::Read, AclCategory::Fast, AclCategory::String],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "SETRANGE",
            &[AclCategory::Write, AclCategory::Slow, AclCategory::String],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "GETRANGE",
            &[AclCategory::Read, AclCategory::Slow, AclCategory::String],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "MGET",
            &[AclCategory::Read, AclCategory::Fast, AclCategory::String],
            KeyExtractor::AllFrom(1),
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "MSET",
            &[AclCategory::Write, AclCategory::Slow, AclCategory::String],
            KeyExtractor::EveryOtherFrom(1),
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "MSETNX",
            &[AclCategory::Write, AclCategory::Slow, AclCategory::String],
            KeyExtractor::EveryOtherFrom(1),
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "INCRBY",
            &[AclCategory::Write, AclCategory::Fast, AclCategory::String],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "DECR",
            &[AclCategory::Write, AclCategory::Fast, AclCategory::String],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "DECRBY",
            &[AclCategory::Write, AclCategory::Fast, AclCategory::String],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "SETBIT",
            &[
                AclCategory::Write,
                AclCategory::Slow,
                AclCategory::String,
                AclCategory::Bitmap,
            ],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "GETBIT",
            &[
                AclCategory::Read,
                AclCategory::Fast,
                AclCategory::String,
                AclCategory::Bitmap,
            ],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "BITCOUNT",
            &[
                AclCategory::Read,
                AclCategory::Slow,
                AclCategory::String,
                AclCategory::Bitmap,
            ],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "BITPOS",
            &[
                AclCategory::Read,
                AclCategory::Slow,
                AclCategory::String,
                AclCategory::Bitmap,
            ],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "BITOP",
            &[
                AclCategory::Write,
                AclCategory::Slow,
                AclCategory::String,
                AclCategory::Bitmap,
            ],
            KeyExtractor::AllFrom(1),
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "BITFIELD",
            &[
                AclCategory::Write,
                AclCategory::Slow,
                AclCategory::String,
                AclCategory::Bitmap,
            ],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "BITFIELD_RO",
            &[
                AclCategory::Read,
                AclCategory::Slow,
                AclCategory::String,
                AclCategory::Bitmap,
            ],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "PFADD",
            &[
                AclCategory::Write,
                AclCategory::Slow,
                AclCategory::String,
                AclCategory::HyperLogLog,
            ],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "PFCOUNT",
            &[
                AclCategory::Read,
                AclCategory::Slow,
                AclCategory::String,
                AclCategory::HyperLogLog,
            ],
            KeyExtractor::AllFrom(1),
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "PFMERGE",
            &[
                AclCategory::Write,
                AclCategory::Slow,
                AclCategory::String,
                AclCategory::HyperLogLog,
            ],
            KeyExtractor::AllFrom(1),
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "HSET",
            &[AclCategory::Write, AclCategory::Fast, AclCategory::Hash],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "HMSET",
            &[AclCategory::Write, AclCategory::Fast, AclCategory::Hash],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "HSETNX",
            &[AclCategory::Write, AclCategory::Fast, AclCategory::Hash],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "HGET",
            &[AclCategory::Read, AclCategory::Fast, AclCategory::Hash],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "HMGET",
            &[AclCategory::Read, AclCategory::Fast, AclCategory::Hash],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "HGETALL",
            &[AclCategory::Read, AclCategory::Slow, AclCategory::Hash],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "HDEL",
            &[AclCategory::Write, AclCategory::Fast, AclCategory::Hash],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "HEXISTS",
            &[AclCategory::Read, AclCategory::Fast, AclCategory::Hash],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "HKEYS",
            &[AclCategory::Read, AclCategory::Slow, AclCategory::Hash],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "HVALS",
            &[AclCategory::Read, AclCategory::Slow, AclCategory::Hash],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "HLEN",
            &[AclCategory::Read, AclCategory::Fast, AclCategory::Hash],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "HSTRLEN",
            &[AclCategory::Read, AclCategory::Fast, AclCategory::Hash],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "HINCRBY",
            &[AclCategory::Write, AclCategory::Fast, AclCategory::Hash],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "HINCRBYFLOAT",
            &[AclCategory::Write, AclCategory::Fast, AclCategory::Hash],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "HSCAN",
            &[AclCategory::Read, AclCategory::Slow, AclCategory::Hash],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "HRANDFIELD",
            &[AclCategory::Read, AclCategory::Slow, AclCategory::Hash],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "LPUSH",
            &[AclCategory::Write, AclCategory::Fast, AclCategory::List],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "RPUSH",
            &[AclCategory::Write, AclCategory::Fast, AclCategory::List],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "LPOP",
            &[AclCategory::Write, AclCategory::Fast, AclCategory::List],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "RPOP",
            &[AclCategory::Write, AclCategory::Fast, AclCategory::List],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "LLEN",
            &[AclCategory::Read, AclCategory::Fast, AclCategory::List],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "LINDEX",
            &[AclCategory::Read, AclCategory::Fast, AclCategory::List],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "LRANGE",
            &[AclCategory::Read, AclCategory::Slow, AclCategory::List],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "LSET",
            &[AclCategory::Write, AclCategory::Slow, AclCategory::List],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "LTRIM",
            &[AclCategory::Write, AclCategory::Slow, AclCategory::List],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "LINSERT",
            &[AclCategory::Write, AclCategory::Slow, AclCategory::List],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "LPOS",
            &[AclCategory::Read, AclCategory::Slow, AclCategory::List],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "LMOVE",
            &[AclCategory::Write, AclCategory::Slow, AclCategory::List],
            KeyExtractor::Pair,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "LMPOP",
            &[AclCategory::Write, AclCategory::Slow, AclCategory::List],
            KeyExtractor::Counted {
                count_index: 1,
                first_key: 2,
            },
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "BLPOP",
            &[AclCategory::Write, AclCategory::Blocking, AclCategory::List],
            KeyExtractor::AllExceptLastFrom(1),
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "BRPOP",
            &[AclCategory::Write, AclCategory::Blocking, AclCategory::List],
            KeyExtractor::AllExceptLastFrom(1),
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "BLMPOP",
            &[AclCategory::Write, AclCategory::Blocking, AclCategory::List],
            KeyExtractor::Counted {
                count_index: 2,
                first_key: 3,
            },
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "BRPOPLPUSH",
            &[AclCategory::Write, AclCategory::Blocking, AclCategory::List],
            KeyExtractor::Pair,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "SADD",
            &[AclCategory::Write, AclCategory::Fast, AclCategory::Set],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "SREM",
            &[AclCategory::Write, AclCategory::Fast, AclCategory::Set],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "SMEMBERS",
            &[AclCategory::Read, AclCategory::Slow, AclCategory::Set],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "SCARD",
            &[AclCategory::Read, AclCategory::Fast, AclCategory::Set],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "SMOVE",
            &[AclCategory::Write, AclCategory::Slow, AclCategory::Set],
            KeyExtractor::Pair,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "SPOP",
            &[AclCategory::Write, AclCategory::Slow, AclCategory::Set],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "SINTER",
            &[AclCategory::Read, AclCategory::Slow, AclCategory::Set],
            KeyExtractor::AllFrom(1),
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "SUNION",
            &[AclCategory::Read, AclCategory::Slow, AclCategory::Set],
            KeyExtractor::AllFrom(1),
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "SDIFF",
            &[AclCategory::Read, AclCategory::Slow, AclCategory::Set],
            KeyExtractor::AllFrom(1),
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "SSCAN",
            &[AclCategory::Read, AclCategory::Slow, AclCategory::Set],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "SISMEMBER",
            &[AclCategory::Read, AclCategory::Fast, AclCategory::Set],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "SDIFFSTORE",
            &[AclCategory::Write, AclCategory::Slow, AclCategory::Set],
            KeyExtractor::AllFrom(1),
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "SINTERCARD",
            &[AclCategory::Read, AclCategory::Slow, AclCategory::Set],
            KeyExtractor::Counted {
                count_index: 1,
                first_key: 2,
            },
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "SINTERSTORE",
            &[AclCategory::Write, AclCategory::Slow, AclCategory::Set],
            KeyExtractor::AllFrom(1),
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "SUNIONSTORE",
            &[AclCategory::Write, AclCategory::Slow, AclCategory::Set],
            KeyExtractor::AllFrom(1),
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "SRANDMEMBER",
            &[AclCategory::Read, AclCategory::Slow, AclCategory::Set],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "ZADD",
            &[
                AclCategory::Write,
                AclCategory::Slow,
                AclCategory::SortedSet,
            ],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "ZREM",
            &[
                AclCategory::Write,
                AclCategory::Fast,
                AclCategory::SortedSet,
            ],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "ZCARD",
            &[AclCategory::Read, AclCategory::Fast, AclCategory::SortedSet],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "ZCOUNT",
            &[AclCategory::Read, AclCategory::Slow, AclCategory::SortedSet],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "ZSCORE",
            &[AclCategory::Read, AclCategory::Fast, AclCategory::SortedSet],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "ZRANK",
            &[AclCategory::Read, AclCategory::Fast, AclCategory::SortedSet],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "ZREVRANK",
            &[AclCategory::Read, AclCategory::Fast, AclCategory::SortedSet],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "ZINCRBY",
            &[
                AclCategory::Write,
                AclCategory::Fast,
                AclCategory::SortedSet,
            ],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "ZMSCORE",
            &[AclCategory::Read, AclCategory::Fast, AclCategory::SortedSet],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "ZRANGE",
            &[AclCategory::Read, AclCategory::Slow, AclCategory::SortedSet],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "ZPOPMIN",
            &[
                AclCategory::Write,
                AclCategory::Fast,
                AclCategory::SortedSet,
            ],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "ZPOPMAX",
            &[
                AclCategory::Write,
                AclCategory::Fast,
                AclCategory::SortedSet,
            ],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "BZPOPMIN",
            &[
                AclCategory::Write,
                AclCategory::Blocking,
                AclCategory::SortedSet,
            ],
            KeyExtractor::AllExceptLastFrom(1),
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "BZPOPMAX",
            &[
                AclCategory::Write,
                AclCategory::Blocking,
                AclCategory::SortedSet,
            ],
            KeyExtractor::AllExceptLastFrom(1),
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "ZMPOP",
            &[
                AclCategory::Write,
                AclCategory::Slow,
                AclCategory::SortedSet,
            ],
            KeyExtractor::Counted {
                count_index: 1,
                first_key: 2,
            },
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "BZMPOP",
            &[
                AclCategory::Write,
                AclCategory::Blocking,
                AclCategory::SortedSet,
            ],
            KeyExtractor::Counted {
                count_index: 2,
                first_key: 3,
            },
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "ZINTER",
            &[AclCategory::Read, AclCategory::Slow, AclCategory::SortedSet],
            KeyExtractor::Counted {
                count_index: 1,
                first_key: 2,
            },
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "ZUNION",
            &[AclCategory::Read, AclCategory::Slow, AclCategory::SortedSet],
            KeyExtractor::Counted {
                count_index: 1,
                first_key: 2,
            },
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "ZDIFF",
            &[AclCategory::Read, AclCategory::Slow, AclCategory::SortedSet],
            KeyExtractor::Counted {
                count_index: 1,
                first_key: 2,
            },
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "ZSCAN",
            &[AclCategory::Read, AclCategory::Slow, AclCategory::SortedSet],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "ZRANDMEMBER",
            &[AclCategory::Read, AclCategory::Slow, AclCategory::SortedSet],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "ZRANGEBYSCORE",
            &[AclCategory::Read, AclCategory::Slow, AclCategory::SortedSet],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "ZREVRANGEBYSCORE",
            &[AclCategory::Read, AclCategory::Slow, AclCategory::SortedSet],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "ZREMRANGEBYRANK",
            &[
                AclCategory::Write,
                AclCategory::Slow,
                AclCategory::SortedSet,
            ],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "GEOADD",
            &[AclCategory::Write, AclCategory::Slow, AclCategory::Geo],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "GEOPOS",
            &[AclCategory::Read, AclCategory::Slow, AclCategory::Geo],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "GEODIST",
            &[AclCategory::Read, AclCategory::Slow, AclCategory::Geo],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "GEOHASH",
            &[AclCategory::Read, AclCategory::Slow, AclCategory::Geo],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "GEORADIUS",
            &[AclCategory::Read, AclCategory::Slow, AclCategory::Geo],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "GEORADIUS_RO",
            &[AclCategory::Read, AclCategory::Slow, AclCategory::Geo],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "GEOSEARCH",
            &[AclCategory::Read, AclCategory::Slow, AclCategory::Geo],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "GEOSEARCHSTORE",
            &[AclCategory::Write, AclCategory::Slow, AclCategory::Geo],
            KeyExtractor::Pair,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "GEORADIUSBYMEMBER",
            &[AclCategory::Read, AclCategory::Slow, AclCategory::Geo],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "GEORADIUSBYMEMBER_RO",
            &[AclCategory::Read, AclCategory::Slow, AclCategory::Geo],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "XADD",
            &[AclCategory::Write, AclCategory::Slow, AclCategory::Stream],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "XLEN",
            &[AclCategory::Read, AclCategory::Fast, AclCategory::Stream],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "XDEL",
            &[AclCategory::Write, AclCategory::Fast, AclCategory::Stream],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "XRANGE",
            &[AclCategory::Read, AclCategory::Slow, AclCategory::Stream],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "XREVRANGE",
            &[AclCategory::Read, AclCategory::Slow, AclCategory::Stream],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "XTRIM",
            &[AclCategory::Write, AclCategory::Slow, AclCategory::Stream],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "XREAD",
            &[
                AclCategory::Read,
                AclCategory::Blocking,
                AclCategory::Stream,
            ],
            KeyExtractor::XReadStyle,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "XGROUP",
            &[AclCategory::Write, AclCategory::Slow, AclCategory::Stream],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "XACK",
            &[AclCategory::Write, AclCategory::Fast, AclCategory::Stream],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "XCLAIM",
            &[AclCategory::Write, AclCategory::Slow, AclCategory::Stream],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "XPENDING",
            &[AclCategory::Read, AclCategory::Slow, AclCategory::Stream],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "XREADGROUP",
            &[
                AclCategory::Read,
                AclCategory::Blocking,
                AclCategory::Stream,
            ],
            KeyExtractor::XReadStyle,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "XAUTOCLAIM",
            &[AclCategory::Write, AclCategory::Slow, AclCategory::Stream],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "EXISTS",
            &[AclCategory::Read, AclCategory::Fast, AclCategory::Keyspace],
            KeyExtractor::AllFrom(1),
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "TOUCH",
            &[AclCategory::Read, AclCategory::Fast, AclCategory::Keyspace],
            KeyExtractor::AllFrom(1),
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "UNLINK",
            &[AclCategory::Write, AclCategory::Fast, AclCategory::Keyspace],
            KeyExtractor::AllFrom(1),
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "TYPE",
            &[AclCategory::Read, AclCategory::Fast, AclCategory::Keyspace],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "RENAME",
            &[AclCategory::Write, AclCategory::Slow, AclCategory::Keyspace],
            KeyExtractor::Pair,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "RENAMENX",
            &[AclCategory::Write, AclCategory::Slow, AclCategory::Keyspace],
            KeyExtractor::Pair,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "DBSIZE",
            &[AclCategory::Read, AclCategory::Fast, AclCategory::Keyspace],
            KeyExtractor::None,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "KEYS",
            &[
                AclCategory::Read,
                AclCategory::Slow,
                AclCategory::Keyspace,
                AclCategory::Dangerous,
            ],
            KeyExtractor::None,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "SCAN",
            &[AclCategory::Read, AclCategory::Slow, AclCategory::Keyspace],
            KeyExtractor::None,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "MOVE",
            &[AclCategory::Write, AclCategory::Slow, AclCategory::Keyspace],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "DUMP",
            &[AclCategory::Read, AclCategory::Slow, AclCategory::Keyspace],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "RESTORE",
            &[
                AclCategory::Write,
                AclCategory::Slow,
                AclCategory::Keyspace,
                AclCategory::Dangerous,
            ],
            KeyExtractor::Single,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "SORT",
            &[AclCategory::Write, AclCategory::Slow, AclCategory::Keyspace],
            KeyExtractor::SortStore,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "COPY",
            &[AclCategory::Write, AclCategory::Slow, AclCategory::Keyspace],
            KeyExtractor::Pair,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "FLUSHDB",
            ADMIN_DANGEROUS,
            KeyExtractor::None,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "FLUSHALL",
            ADMIN_DANGEROUS,
            KeyExtractor::None,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "EVAL",
            &[AclCategory::Scripting, AclCategory::Slow],
            KeyExtractor::EvalStyle,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "EVAL_RO",
            &[AclCategory::Scripting, AclCategory::Slow, AclCategory::Read],
            KeyExtractor::EvalStyle,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "EVALSHA",
            &[AclCategory::Scripting, AclCategory::Slow],
            KeyExtractor::EvalStyle,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "EVALSHA_RO",
            &[AclCategory::Scripting, AclCategory::Slow, AclCategory::Read],
            KeyExtractor::EvalStyle,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "SCRIPT",
            &[AclCategory::Scripting, AclCategory::Slow],
            KeyExtractor::None,
            ChannelExtractor::None,
        ),
        CommandSpec::new(
            "PUBLISH",
            PUBSUB_WRITE,
            KeyExtractor::None,
            ChannelExtractor::First,
        ),
        CommandSpec::new(
            "SUBSCRIBE",
            PUBSUB_READ,
            KeyExtractor::None,
            ChannelExtractor::AllFrom(1),
        ),
        CommandSpec::new(
            "UNSUBSCRIBE",
            PUBSUB_READ,
            KeyExtractor::None,
            ChannelExtractor::AllFrom(1),
        ),
        CommandSpec::new(
            "PSUBSCRIBE",
            PUBSUB_READ,
            KeyExtractor::None,
            ChannelExtractor::AllFrom(1),
        ),
        CommandSpec::new(
            "PUNSUBSCRIBE",
            PUBSUB_READ,
            KeyExtractor::None,
            ChannelExtractor::AllFrom(1),
        ),
        CommandSpec::new(
            "PUBSUB",
            PUBSUB_READ,
            KeyExtractor::None,
            ChannelExtractor::None,
        ),
    ];
    ALL_SPECS
}

fn extract_keys<'a>(extractor: KeyExtractor, args: &'a [CompactArg]) -> Vec<&'a [u8]> {
    match extractor {
        KeyExtractor::None => Vec::new(),
        KeyExtractor::Single => get_arg(args, 1).into_iter().collect(),
        KeyExtractor::Pair => [get_arg(args, 1), get_arg(args, 2)]
            .into_iter()
            .flatten()
            .collect(),
        KeyExtractor::AllFrom(index) => args.iter().skip(index).map(CompactArg::as_slice).collect(),
        KeyExtractor::AllExceptLastFrom(index) => args
            .get(index..args.len().saturating_sub(1))
            .unwrap_or(&[])
            .iter()
            .map(CompactArg::as_slice)
            .collect(),
        KeyExtractor::EveryOtherFrom(index) => args
            .iter()
            .skip(index)
            .step_by(2)
            .map(CompactArg::as_slice)
            .collect(),
        KeyExtractor::Counted {
            count_index,
            first_key,
        } => parse_count(args, count_index)
            .and_then(|count| args.get(first_key..first_key + count))
            .unwrap_or(&[])
            .iter()
            .map(CompactArg::as_slice)
            .collect(),
        KeyExtractor::EvalStyle => parse_count(args, 2)
            .and_then(|count| args.get(3..3 + count))
            .unwrap_or(&[])
            .iter()
            .map(CompactArg::as_slice)
            .collect(),
        KeyExtractor::XReadStyle => args
            .iter()
            .position(|arg| arg.as_slice().eq_ignore_ascii_case(b"STREAMS"))
            .map(|streams_index| {
                let after_streams = &args[streams_index + 1..];
                let split = after_streams.len() / 2;
                after_streams[..split]
                    .iter()
                    .map(CompactArg::as_slice)
                    .collect()
            })
            .unwrap_or_default(),
        KeyExtractor::SortStore => {
            let mut keys = get_arg(args, 1).into_iter().collect::<Vec<_>>();
            let mut index = 2;
            while index + 1 < args.len() {
                if args[index].as_slice().eq_ignore_ascii_case(b"STORE") {
                    keys.push(args[index + 1].as_slice());
                    break;
                }
                index += 1;
            }
            keys
        }
    }
}

fn extract_channels<'a>(extractor: ChannelExtractor, args: &'a [CompactArg]) -> Vec<&'a [u8]> {
    match extractor {
        ChannelExtractor::None => Vec::new(),
        ChannelExtractor::First => get_arg(args, 1).into_iter().collect(),
        ChannelExtractor::AllFrom(index) => {
            args.iter().skip(index).map(CompactArg::as_slice).collect()
        }
    }
}

fn get_arg(args: &[CompactArg], index: usize) -> Option<&[u8]> {
    args.get(index).map(CompactArg::as_slice)
}

fn parse_count(args: &[CompactArg], index: usize) -> Option<usize> {
    std::str::from_utf8(args.get(index)?.as_slice())
        .ok()?
        .parse()
        .ok()
}
