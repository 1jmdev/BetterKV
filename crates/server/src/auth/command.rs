use commands::command::CommandId;
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

pub(super) fn command_spec(command: CommandId) -> Option<CommandSpec> {
    spec_by_name(command.name()?)
}

#[inline(always)]
fn spec_by_name(name: &str) -> Option<CommandSpec> {
    all_specs().iter().copied().find(|spec| spec.name == name)
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
