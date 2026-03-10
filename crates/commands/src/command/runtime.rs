use crate::dispatch::{
    AclCategory, CommandId, command_auth_spec, command_flags, command_runtime_spec, identify,
    is_supported_command_id,
};
use crate::util::{Args, eq_ascii};
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
    let runtime = command_runtime_spec(id);
    RespFrame::Array(Some(vec![
        bulk(lower_name.as_bytes()),
        RespFrame::Integer(runtime.arity),
        RespFrame::Array(Some(
            command_flags(id)
                .iter()
                .map(|flag| bulk(flag.as_bytes()))
                .collect(),
        )),
        RespFrame::Integer(runtime.first_key),
        RespFrame::Integer(runtime.last_key),
        RespFrame::Integer(runtime.step),
        RespFrame::Array(Some(
            acl_categories(id)
                .into_iter()
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
        .filter(|id| is_supported_command_id(*id))
}

fn supported_command(name: &[u8]) -> Option<CommandId> {
    let id = identify(name);
    is_supported_command_id(id).then_some(id)
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

fn acl_categories(id: CommandId) -> Vec<&'static str> {
    command_auth_spec(id)
        .map(|spec| {
            spec.categories
                .iter()
                .filter_map(|category| acl_category_name(*category))
                .collect()
        })
        .unwrap_or_default()
}

fn acl_category_name(category: AclCategory) -> Option<&'static str> {
    match category {
        AclCategory::All => None,
        AclCategory::Admin => Some("@admin"),
        AclCategory::Bitmap => Some("@bitmap"),
        AclCategory::Blocking => Some("@blocking"),
        AclCategory::Connection => Some("@connection"),
        AclCategory::Dangerous => Some("@dangerous"),
        AclCategory::Fast => Some("@fast"),
        AclCategory::Geo => Some("@geo"),
        AclCategory::Hash => Some("@hash"),
        AclCategory::HyperLogLog => Some("@hyperloglog"),
        AclCategory::Keyspace => Some("@keyspace"),
        AclCategory::List => Some("@list"),
        AclCategory::PubSub => Some("@pubsub"),
        AclCategory::Read => Some("@read"),
        AclCategory::Scripting => Some("@scripting"),
        AclCategory::Set => Some("@set"),
        AclCategory::Slow => Some("@slow"),
        AclCategory::SortedSet => Some("@sortedset"),
        AclCategory::Stream => Some("@stream"),
        AclCategory::String => Some("@string"),
        AclCategory::Write => Some("@write"),
    }
}

fn group_of(id: CommandId) -> &'static str {
    command_runtime_spec(id).group
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
