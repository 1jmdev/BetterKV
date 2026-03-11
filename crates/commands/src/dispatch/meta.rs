use super::list::CommandId;
use super::registry::with_command_registry;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum AclCategory {
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
    Transaction,
    Write,
}

impl AclCategory {
    pub const ALL_COMMAND_CATEGORIES: &'static [Self] = &[
        Self::Admin,
        Self::Bitmap,
        Self::Blocking,
        Self::Connection,
        Self::Dangerous,
        Self::Fast,
        Self::Geo,
        Self::Hash,
        Self::HyperLogLog,
        Self::Keyspace,
        Self::List,
        Self::PubSub,
        Self::Read,
        Self::Scripting,
        Self::Set,
        Self::Slow,
        Self::SortedSet,
        Self::Stream,
        Self::String,
        Self::Transaction,
        Self::Write,
    ];

    pub fn as_str(self) -> &'static str {
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
            Self::Transaction => "transaction",
            Self::Write => "write",
        }
    }

    pub fn parse(value: &str) -> Result<Self, String> {
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
            "transaction" => Ok(Self::Transaction),
            "write" => Ok(Self::Write),
            _ => Err(format!("ERR Unknown ACL category '{value}'")),
        }
    }
}

#[derive(Clone, Copy)]
pub enum KeyExtraction {
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
pub enum ChannelExtraction {
    None,
    First,
    AllFrom(usize),
}

#[derive(Clone, Copy)]
pub struct CommandAuthSpec {
    pub name: &'static [u8],
    pub categories: &'static [AclCategory],
    pub key_extraction: KeyExtraction,
    pub channel_extraction: ChannelExtraction,
}

#[derive(Clone, Copy)]
pub enum NotificationKeyArguments {
    Argument(usize),
    AllFrom(usize),
    EveryOtherFrom(usize),
}

#[derive(Clone, Copy)]
pub enum NotificationResponsePolicy {
    AnySuccess,
    IntegerOne,
    PositiveInteger,
    OkOrIntegerOne,
}

#[derive(Clone, Copy)]
pub struct CommandNotificationSpec {
    pub event: &'static [u8],
    pub class: u8,
    pub keys: NotificationKeyArguments,
    pub response: NotificationResponsePolicy,
}

#[derive(Clone, Copy)]
pub struct CommandRuntimeSpec {
    pub supported: bool,
    pub group: &'static str,
    pub arity: i64,
    pub first_key: i64,
    pub last_key: i64,
    pub step: i64,
    pub readonly: bool,
}

macro_rules! auth_spec_value {
    (bytes: $bytes:expr, auth: none) => {
        None
    };
    (bytes: $bytes:expr, auth: some { categories: $categories:expr, keys: $keys:expr, channels: $channels:expr }) => {
        Some(CommandAuthSpec {
            name: $bytes,
            categories: $categories,
            key_extraction: $keys,
            channel_extraction: $channels,
        })
    };
}

macro_rules! notification_spec_value {
    ($variant:ident, notify: none) => {
        None
    };
    ($variant:ident, notify: some { event: $event:expr, class: $class:expr, keys: $keys:expr, response: $response:expr }) => {
        Some(CommandNotificationSpec {
            event: $event,
            class: $class,
            keys: $keys,
            response: $response,
        })
    };
}

macro_rules! generate_meta {
    (
        $(
            $len:literal => {
                $(
                    $first:expr => {
                        $(
                            {
                                variant: $variant:ident,
                                bytes: $bytes:expr,
                                dispatch: [ $($dispatch:tt)* ],
                                supported: $supported:tt,
                                group: $group:literal,
                                shape: ($arity:expr, $first_key:expr, $last_key:expr, $step:expr),
                                readonly: $readonly:tt,
                                write: $write:tt,
                                auth: $auth_kind:ident $( {
                                    categories: $categories:expr,
                                    keys: $keys:expr,
                                    channels: $channels:expr,
                                } )?,
                                notify: $notify_kind:ident $( {
                                    event: $event:expr,
                                    class: $class:expr,
                                    keys: $notify_keys:expr,
                                    response: $response:expr,
                                } )?,
                            }
                        )*
                    }
                )*
            }
        )*
    ) => {
        #[inline(always)]
        pub fn command_runtime_spec(command: CommandId) -> CommandRuntimeSpec {
            match command {
                $( $( $(
                    CommandId::$variant => CommandRuntimeSpec {
                        supported: $supported,
                        group: $group,
                        arity: $arity,
                        first_key: $first_key,
                        last_key: $last_key,
                        step: $step,
                        readonly: $readonly,
                    },
                )* )* )*
                CommandId::Unknown => CommandRuntimeSpec {
                    supported: false,
                    group: "",
                    arity: 0,
                    first_key: 0,
                    last_key: 0,
                    step: 0,
                    readonly: false,
                },
            }
        }

        #[inline(always)]
        pub fn is_supported_command_id(command: CommandId) -> bool {
            command_runtime_spec(command).supported
        }

        #[inline(always)]
        pub fn command_auth_spec(command: CommandId) -> Option<CommandAuthSpec> {
            match command {
                $( $( $( CommandId::$variant => auth_spec_value!(bytes: $bytes, auth: $auth_kind $( { categories: $categories, keys: $keys, channels: $channels } )?), )* )* )*
                CommandId::Unknown => None,
            }
        }

        #[inline(always)]
        pub fn is_write_command_id(command: CommandId) -> bool {
            match command {
                $( $( $( CommandId::$variant => $write, )* )* )*
                CommandId::Unknown => false,
            }
        }

        #[inline(always)]
        pub fn command_flags(command: CommandId) -> &'static [&'static str] {
            let runtime = command_runtime_spec(command);
            if !runtime.supported {
                &[]
            } else if runtime.readonly {
                &["readonly"]
            } else {
                &["write"]
            }
        }

        #[inline(always)]
        pub fn command_notification_spec(command: CommandId) -> Option<CommandNotificationSpec> {
            match command {
                $( $( $( CommandId::$variant => notification_spec_value!($variant, notify: $notify_kind $( { event: $event, class: $class, keys: $notify_keys, response: $response } )?), )* )* )*
                CommandId::Unknown => None,
            }
        }
    };
}

with_command_registry!(generate_meta);
