use crate::dispatch::{identify, CommandId};
use crate::util::{eq_ascii, Args};
use protocol::types::{BulkData, RespFrame};
use types::value::CompactArg;

pub(crate) fn command(args: &Args) -> RespFrame {
    if args.len() == 1 {
        return RespFrame::Array(Some(supported_ids().map(info_frame).collect()));
    }

    if eq_ascii(&args[1], b"COUNT") {
        if args.len() != 2 {
            return RespFrame::error_static(
                "ERR wrong number of arguments for 'command|count' command",
            );
        }
        return RespFrame::Integer(supported_ids().count() as i64);
    }

    if eq_ascii(&args[1], b"INFO") {
        return command_info(args);
    }

    if eq_ascii(&args[1], b"DOCS") {
        return command_docs(args);
    }

    if eq_ascii(&args[1], b"LIST") {
        return command_list(args);
    }

    RespFrame::Error(format!(
        "ERR unknown subcommand '{}'.",
        String::from_utf8_lossy(args[1].as_slice())
    ))
}

fn command_info(args: &Args) -> RespFrame {
    if args.len() == 2 {
        return RespFrame::Array(Some(supported_ids().map(info_frame).collect()));
    }

    RespFrame::Array(Some(
        args[2..]
            .iter()
            .map(|name| match supported_command(name.as_slice()) {
                Some(id) => info_frame(id),
                None => RespFrame::Bulk(None),
            })
            .collect(),
    ))
}

fn command_docs(args: &Args) -> RespFrame {
    let names = if args.len() == 2 {
        supported_ids().collect::<Vec<_>>()
    } else {
        args[2..]
            .iter()
            .filter_map(|name| supported_command(name.as_slice()))
            .collect::<Vec<_>>()
    };

    RespFrame::Map(
        names
            .into_iter()
            .map(|id| (bulk_lower(name_of(id)), docs_frame(id)))
            .collect(),
    )
}

fn command_list(args: &Args) -> RespFrame {
    let filtered = match args.len() {
        2 => supported_ids().collect::<Vec<_>>(),
        5 if eq_ascii(&args[2], b"FILTERBY") => {
            if eq_ascii(&args[3], b"MODULE") {
                Vec::new()
            } else if eq_ascii(&args[3], b"ACLCAT") {
                let category = args[4].as_slice();
                supported_ids()
                    .filter(|id| {
                        acl_categories(*id)
                            .iter()
                            .any(|item| category_matches(item, category))
                    })
                    .collect()
            } else if eq_ascii(&args[3], b"PATTERN") {
                let pattern = args[4].as_slice();
                supported_ids()
                    .filter(|id| glob_matches(pattern, name_of(*id).as_bytes()))
                    .collect()
            } else {
                return RespFrame::error_static("ERR syntax error");
            }
        }
        _ => return RespFrame::error_static("ERR syntax error"),
    };

    RespFrame::Array(Some(
        filtered
            .into_iter()
            .map(|id| bulk_upper(name_of(id)))
            .collect(),
    ))
}

fn info_frame(id: CommandId) -> RespFrame {
    let lower_name = name_of(id).to_ascii_lowercase();
    let (arity, first_key, last_key, step) = command_shape(id);
    RespFrame::Array(Some(vec![
        bulk(lower_name.as_bytes()),
        RespFrame::Integer(arity),
        RespFrame::Array(Some(
            command_flags(id)
                .iter()
                .map(|flag| bulk(flag.as_bytes()))
                .collect(),
        )),
        RespFrame::Integer(first_key),
        RespFrame::Integer(last_key),
        RespFrame::Integer(step),
        RespFrame::Array(Some(
            acl_categories(id)
                .iter()
                .map(|flag| bulk(flag.as_bytes()))
                .collect(),
        )),
        RespFrame::Array(Some(vec![])),
        RespFrame::Array(Some(vec![])),
        RespFrame::Array(Some(vec![])),
    ]))
}

fn docs_frame(id: CommandId) -> RespFrame {
    RespFrame::Map(vec![
        (bulk_static("summary"), bulk(summary_of(id).as_bytes())),
        (bulk_static("since"), bulk_static("2.8.13")),
        (bulk_static("group"), bulk(group_of(id).as_bytes())),
        (
            bulk_static("complexity"),
            bulk(complexity_of(id).as_bytes()),
        ),
        (bulk_static("arguments"), RespFrame::Array(Some(vec![]))),
    ])
}

fn supported_ids() -> impl Iterator<Item = CommandId> {
    CommandId::ALL
        .iter()
        .copied()
        .filter(|id| id.is_supported())
}

fn supported_command(name: &[u8]) -> Option<CommandId> {
    let id = identify(name);
    id.is_supported().then_some(id)
}

fn bulk_static(value: &'static str) -> RespFrame {
    RespFrame::Bulk(Some(BulkData::Arg(CompactArg::from_slice(
        value.as_bytes(),
    ))))
}

fn bulk(value: &[u8]) -> RespFrame {
    RespFrame::Bulk(Some(BulkData::Arg(CompactArg::from_slice(value))))
}

fn bulk_upper(value: &str) -> RespFrame {
    bulk(value.as_bytes())
}

fn bulk_lower(value: &str) -> RespFrame {
    let value = value.to_ascii_lowercase();
    bulk(value.as_bytes())
}

fn name_of(id: CommandId) -> &'static str {
    id.name().unwrap_or("UNKNOWN")
}

impl CommandId {
    fn is_supported(self) -> bool {
        !matches!(
            self,
            Self::Unknown
                | Self::Delex
                | Self::Digest
                | Self::SubStr
                | Self::Lcs
                | Self::MSetEx
                | Self::Acl
                | Self::Config
                | Self::Publish
                | Self::Subscribe
                | Self::Unsubscribe
                | Self::PSubscribe
                | Self::PUnsubscribe
                | Self::PubSub
                | Self::Multi
                | Self::Exec
                | Self::Discard
                | Self::Watch
                | Self::Unwatch
        )
    }
}

fn command_shape(id: CommandId) -> (i64, i64, i64, i64) {
    match id {
        CommandId::Command => (-1, 0, 0, 0),
        CommandId::Ping
        | CommandId::Echo
        | CommandId::Auth
        | CommandId::Hello
        | CommandId::Client
        | CommandId::Script
        | CommandId::Select => (-1, 0, 0, 0),
        CommandId::Del
        | CommandId::Exists
        | CommandId::Touch
        | CommandId::Unlink
        | CommandId::MGet
        | CommandId::SInter
        | CommandId::SUnion
        | CommandId::SDiff => (-2, 1, -1, 1),
        CommandId::MSet | CommandId::MSetNx => (-3, 1, -1, 2),
        CommandId::Rename
        | CommandId::RenameNx
        | CommandId::Copy
        | CommandId::SMove
        | CommandId::LMove
        | CommandId::RPopLPush
        | CommandId::BRPopLPush => (3, 1, 2, 1),
        CommandId::BLPop | CommandId::BRPop | CommandId::BZPopMin | CommandId::BZPopMax => {
            (-3, 1, -2, 1)
        }
        CommandId::Eval | CommandId::EvalRo | CommandId::EvalSha | CommandId::EvalShaRo => {
            (-3, 3, 3, 1)
        }
        CommandId::ZInter
        | CommandId::ZUnion
        | CommandId::ZDiff
        | CommandId::SInterCard
        | CommandId::LMPop
        | CommandId::ZMPop => (-3, 2, -1, 1),
        CommandId::ZInterStore
        | CommandId::ZUnionStore
        | CommandId::ZDiffStore
        | CommandId::SDiffStore
        | CommandId::SInterStore
        | CommandId::SUnionStore => (-4, 1, -1, 1),
        CommandId::JsonMSet => (-4, 1, -1, 3),
        CommandId::JsonMGet => (-3, 1, -2, 1),
        CommandId::XRead | CommandId::XReadGroup => (-4, 0, 0, 0),
        CommandId::Sort => (-2, 1, 1, 1),
        CommandId::Keys
        | CommandId::Scan
        | CommandId::DbSize
        | CommandId::FlushDb
        | CommandId::FlushAll
        | CommandId::Quit => (-1, 0, 0, 0),
        _ if id.is_supported() => (-2, 1, 1, 1),
        _ => (0, 0, 0, 0),
    }
}

fn command_flags(id: CommandId) -> &'static [&'static str] {
    match id {
        CommandId::Get
        | CommandId::MGet
        | CommandId::Exists
        | CommandId::Type
        | CommandId::Ttl
        | CommandId::PTtl
        | CommandId::Keys
        | CommandId::Scan
        | CommandId::Dump
        | CommandId::HGet
        | CommandId::HMGet
        | CommandId::HGetAll
        | CommandId::HExists
        | CommandId::HKeys
        | CommandId::HVals
        | CommandId::HLen
        | CommandId::HStrLen
        | CommandId::HScan
        | CommandId::HRandField
        | CommandId::LLen
        | CommandId::LIndex
        | CommandId::LRange
        | CommandId::LPos
        | CommandId::SMembers
        | CommandId::SCard
        | CommandId::SInter
        | CommandId::SUnion
        | CommandId::SDiff
        | CommandId::SScan
        | CommandId::SIsMember
        | CommandId::SMIsMember
        | CommandId::SRandMember
        | CommandId::ZCard
        | CommandId::ZCount
        | CommandId::ZScore
        | CommandId::ZRank
        | CommandId::ZRevRank
        | CommandId::ZMScore
        | CommandId::ZRange
        | CommandId::ZRevRange
        | CommandId::ZInter
        | CommandId::ZUnion
        | CommandId::ZDiff
        | CommandId::ZScan
        | CommandId::ZRandMember
        | CommandId::ZRangeByScore
        | CommandId::ZRangeByLex
        | CommandId::ZRevRangeByScore
        | CommandId::ZRevRangeByLex
        | CommandId::ZLexCount
        | CommandId::GeoPos
        | CommandId::GeoDist
        | CommandId::GeoHash
        | CommandId::GeoRadiusRo
        | CommandId::GeoSearch
        | CommandId::GeoRadiusByMemberRo
        | CommandId::XLen
        | CommandId::XRange
        | CommandId::XRevRange
        | CommandId::XPending
        | CommandId::JsonGet
        | CommandId::JsonMGet
        | CommandId::JsonObjKeys
        | CommandId::JsonObjLen
        | CommandId::JsonResp
        | CommandId::JsonStrLen
        | CommandId::JsonType
        | CommandId::Command => &["readonly"],
        _ if id.is_supported() => &["write"],
        _ => &[],
    }
}

fn acl_categories(id: CommandId) -> &'static [&'static str] {
    match group_of(id) {
        "string" => &["@read", "@string"],
        "hash" => &["@hash"],
        "list" => &["@list"],
        "set" => &["@set"],
        "sorted-set" => &["@sortedset"],
        "stream" => &["@stream"],
        "geo" => &["@geo"],
        "connection" => &["@connection"],
        "server" => &["@slow", "@connection"],
        _ => &["@keyspace"],
    }
}

fn group_of(id: CommandId) -> &'static str {
    match id {
        CommandId::Auth
        | CommandId::Hello
        | CommandId::Client
        | CommandId::Select
        | CommandId::Quit
        | CommandId::Ping
        | CommandId::Echo => "connection",
        CommandId::Command | CommandId::DbSize | CommandId::FlushDb | CommandId::FlushAll => {
            "server"
        }
        CommandId::Eval
        | CommandId::EvalRo
        | CommandId::EvalSha
        | CommandId::EvalShaRo
        | CommandId::Script => "scripting",
        CommandId::Get
        | CommandId::Set
        | CommandId::SetNx
        | CommandId::GetSet
        | CommandId::GetDel
        | CommandId::SetEx
        | CommandId::PSetEx
        | CommandId::GetEx
        | CommandId::Append
        | CommandId::StrLen
        | CommandId::SetRange
        | CommandId::GetRange
        | CommandId::MGet
        | CommandId::MSet
        | CommandId::MSetNx
        | CommandId::Incr
        | CommandId::IncrBy
        | CommandId::IncrByFloat
        | CommandId::Decr
        | CommandId::DecrBy
        | CommandId::SetBit
        | CommandId::GetBit
        | CommandId::BitCount
        | CommandId::BitPos
        | CommandId::BitOp
        | CommandId::BitField
        | CommandId::BitFieldRo
        | CommandId::PfAdd
        | CommandId::PfCount
        | CommandId::PfMerge => "string",
        CommandId::HSet
        | CommandId::HMSet
        | CommandId::HSetNx
        | CommandId::HGet
        | CommandId::HMGet
        | CommandId::HGetAll
        | CommandId::HDel
        | CommandId::HExists
        | CommandId::HKeys
        | CommandId::HVals
        | CommandId::HLen
        | CommandId::HStrLen
        | CommandId::HIncrBy
        | CommandId::HIncrByFloat
        | CommandId::HScan
        | CommandId::HRandField => "hash",
        CommandId::LPush
        | CommandId::LPushX
        | CommandId::RPush
        | CommandId::RPushX
        | CommandId::LPop
        | CommandId::RPop
        | CommandId::LRem
        | CommandId::LLen
        | CommandId::LIndex
        | CommandId::LRange
        | CommandId::LSet
        | CommandId::LTrim
        | CommandId::LInsert
        | CommandId::LPos
        | CommandId::LMove
        | CommandId::LMPop
        | CommandId::BLPop
        | CommandId::BRPop
        | CommandId::BLMPop
        | CommandId::BRPopLPush
        | CommandId::RPopLPush => "list",
        CommandId::SAdd
        | CommandId::SRem
        | CommandId::SMembers
        | CommandId::SCard
        | CommandId::SMove
        | CommandId::SPop
        | CommandId::SInter
        | CommandId::SUnion
        | CommandId::SDiff
        | CommandId::SScan
        | CommandId::SIsMember
        | CommandId::SMIsMember
        | CommandId::SDiffStore
        | CommandId::SInterCard
        | CommandId::SInterStore
        | CommandId::SUnionStore
        | CommandId::SRandMember => "set",
        CommandId::ZAdd
        | CommandId::ZRem
        | CommandId::ZCard
        | CommandId::ZCount
        | CommandId::ZScore
        | CommandId::ZRank
        | CommandId::ZRevRank
        | CommandId::ZIncrBy
        | CommandId::ZMScore
        | CommandId::ZRange
        | CommandId::ZRevRange
        | CommandId::ZPopMin
        | CommandId::ZPopMax
        | CommandId::BZPopMin
        | CommandId::BZPopMax
        | CommandId::ZMPop
        | CommandId::BZMPop
        | CommandId::ZInter
        | CommandId::ZInterStore
        | CommandId::ZUnion
        | CommandId::ZUnionStore
        | CommandId::ZDiff
        | CommandId::ZDiffStore
        | CommandId::ZScan
        | CommandId::ZRandMember
        | CommandId::ZRangeStore
        | CommandId::ZRangeByScore
        | CommandId::ZRangeByLex
        | CommandId::ZRevRangeByScore
        | CommandId::ZRevRangeByLex
        | CommandId::ZLexCount
        | CommandId::ZRemRangeByLex
        | CommandId::ZRemRangeByRank
        | CommandId::ZRemRangeByScore => "sorted-set",
        CommandId::GeoAdd
        | CommandId::GeoPos
        | CommandId::GeoDist
        | CommandId::GeoHash
        | CommandId::GeoRadius
        | CommandId::GeoRadiusRo
        | CommandId::GeoSearch
        | CommandId::GeoSearchStore
        | CommandId::GeoRadiusByMember
        | CommandId::GeoRadiusByMemberRo => "geo",
        CommandId::XAdd
        | CommandId::XLen
        | CommandId::XDel
        | CommandId::XRange
        | CommandId::XRevRange
        | CommandId::XTrim
        | CommandId::XRead
        | CommandId::XGroup
        | CommandId::XAck
        | CommandId::XClaim
        | CommandId::XPending
        | CommandId::XReadGroup
        | CommandId::XAutoClaim => "stream",
        CommandId::JsonArrAppend
        | CommandId::JsonArrIndex
        | CommandId::JsonArrInsert
        | CommandId::JsonArrLen
        | CommandId::JsonArrPop
        | CommandId::JsonArrTrim
        | CommandId::JsonClear
        | CommandId::JsonDebug
        | CommandId::JsonDel
        | CommandId::JsonForget
        | CommandId::JsonGet
        | CommandId::JsonMerge
        | CommandId::JsonMGet
        | CommandId::JsonMSet
        | CommandId::JsonNumIncrBy
        | CommandId::JsonNumMultBy
        | CommandId::JsonObjKeys
        | CommandId::JsonObjLen
        | CommandId::JsonResp
        | CommandId::JsonSet
        | CommandId::JsonStrAppend
        | CommandId::JsonStrLen
        | CommandId::JsonToggle
        | CommandId::JsonType => "generic",
        _ if id.is_supported() => "generic",
        _ => "",
    }
}

fn summary_of(id: CommandId) -> &'static str {
    match id {
        CommandId::Command => "Returns metadata about server commands.",
        _ => "Supported BetterKV command.",
    }
}

fn complexity_of(id: CommandId) -> &'static str {
    match id {
        CommandId::Command => "O(N) where N is the number of supported commands",
        _ => "O(1)",
    }
}

fn category_matches(category: &str, value: &[u8]) -> bool {
    let category = category.strip_prefix('@').unwrap_or(category);
    eq_ascii(category.as_bytes(), value) || eq_ascii(format!("@{category}").as_bytes(), value)
}

fn glob_matches(pattern: &[u8], candidate: &[u8]) -> bool {
    fn inner(pattern: &[u8], candidate: &[u8]) -> bool {
        if pattern.is_empty() {
            return candidate.is_empty();
        }
        match pattern[0] {
            b'*' => {
                inner(&pattern[1..], candidate)
                    || (!candidate.is_empty() && inner(pattern, &candidate[1..]))
            }
            b'?' => !candidate.is_empty() && inner(&pattern[1..], &candidate[1..]),
            byte => {
                !candidate.is_empty()
                    && byte.eq_ignore_ascii_case(&candidate[0])
                    && inner(&pattern[1..], &candidate[1..])
            }
        }
    }

    inner(pattern, candidate)
}
