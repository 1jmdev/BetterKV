use commands::command::identify;
use parking_lot::RwLock;
use protocol::types::{BulkData, RespFrame};
use std::sync::atomic::{AtomicU64, Ordering};
use types::value::CompactArg;

use super::command::{AclCategory, acl_categories, commands_in_category};
use super::error::{PermissionError, no_auth, no_perm};
use super::session::SessionAuth;
use super::state::{AuthState, DEFAULT_USER};

pub(super) fn handle_acl_command(
    inner: &RwLock<AuthState>,
    acl_epoch: &AtomicU64,
    session: &SessionAuth,
    args: &[CompactArg],
) -> RespFrame {
    let _trace = profiler::scope("server::auth::acl_command");
    if !session.is_authorized() {
        return no_auth();
    }
    if args.len() < 2 {
        return RespFrame::error_static("ERR wrong number of arguments for 'ACL' command");
    }

    let sub = args[1].as_slice();
    if sub.eq_ignore_ascii_case(b"WHOAMI") {
        return acl_whoami(session, args);
    }
    if sub.eq_ignore_ascii_case(b"USERS") {
        return acl_users(inner, args);
    }
    if sub.eq_ignore_ascii_case(b"LIST") {
        return acl_list(inner, args);
    }
    if sub.eq_ignore_ascii_case(b"SETUSER") {
        return acl_setuser(inner, acl_epoch, args);
    }
    if sub.eq_ignore_ascii_case(b"DELUSER") {
        return acl_deluser(inner, acl_epoch, args);
    }
    if sub.eq_ignore_ascii_case(b"GETUSER") {
        return acl_getuser(inner, args);
    }
    if sub.eq_ignore_ascii_case(b"CAT") {
        return acl_cat(args);
    }
    if sub.eq_ignore_ascii_case(b"GENPASS") {
        return acl_genpass(args);
    }
    if sub.eq_ignore_ascii_case(b"HELP") {
        return acl_help(args);
    }
    if sub.eq_ignore_ascii_case(b"LOG") {
        return acl_log(args);
    }
    if sub.eq_ignore_ascii_case(b"LOAD") || sub.eq_ignore_ascii_case(b"SAVE") {
        return acl_noop(args);
    }
    if sub.eq_ignore_ascii_case(b"DRYRUN") {
        return acl_dryrun(inner, args);
    }

    RespFrame::error_static("ERR Unknown ACL subcommand")
}

fn acl_whoami(session: &SessionAuth, args: &[CompactArg]) -> RespFrame {
    if args.len() != 2 {
        return RespFrame::error_static("ERR wrong number of arguments for 'ACL WHOAMI' command");
    }
    let current = session.user().unwrap_or(DEFAULT_USER).as_bytes().to_vec();
    RespFrame::Bulk(Some(BulkData::from_vec(current)))
}

fn acl_users(inner: &RwLock<AuthState>, args: &[CompactArg]) -> RespFrame {
    if args.len() != 2 {
        return RespFrame::error_static("ERR wrong number of arguments for 'ACL USERS' command");
    }
    RespFrame::Array(Some(
        inner
            .read()
            .users
            .keys()
            .map(|name| RespFrame::Bulk(Some(BulkData::from_vec(name.as_bytes().to_vec()))))
            .collect(),
    ))
}

fn acl_list(inner: &RwLock<AuthState>, args: &[CompactArg]) -> RespFrame {
    if args.len() != 2 {
        return RespFrame::error_static("ERR wrong number of arguments for 'ACL LIST' command");
    }
    RespFrame::Array(Some(
        inner
            .read()
            .users
            .iter()
            .map(|(name, user)| {
                RespFrame::Bulk(Some(BulkData::from_vec(user.acl_line(name).into_bytes())))
            })
            .collect(),
    ))
}

fn acl_setuser(inner: &RwLock<AuthState>, acl_epoch: &AtomicU64, args: &[CompactArg]) -> RespFrame {
    if args.len() < 3 {
        return RespFrame::error_static("ERR wrong number of arguments for 'ACL SETUSER' command");
    }
    let username = match std::str::from_utf8(args[2].as_slice()) {
        Ok(name) => name.to_ascii_lowercase(),
        Err(_) => return RespFrame::error_static("ERR invalid ACL username"),
    };
    let rules = args[3..]
        .iter()
        .map(|arg| String::from_utf8_lossy(arg.as_slice()).to_string())
        .collect::<Vec<_>>();

    match inner.write().set_user(&username, &rules) {
        Ok(()) => {
            acl_epoch.fetch_add(1, Ordering::Relaxed);
            RespFrame::ok()
        }
        Err(err) => RespFrame::Error(err),
    }
}

fn acl_deluser(inner: &RwLock<AuthState>, acl_epoch: &AtomicU64, args: &[CompactArg]) -> RespFrame {
    if args.len() < 3 {
        return RespFrame::error_static("ERR wrong number of arguments for 'ACL DELUSER' command");
    }
    let mut state = inner.write();
    let mut removed = 0;
    for name in &args[2..] {
        if let Ok(username) = std::str::from_utf8(name.as_slice()) {
            if username.eq_ignore_ascii_case(DEFAULT_USER) {
                continue;
            }
            if state.users.remove(&username.to_ascii_lowercase()).is_some() {
                removed += 1;
            }
        }
    }
    if removed > 0 {
        acl_epoch.fetch_add(1, Ordering::Relaxed);
    }
    RespFrame::Integer(removed)
}

fn acl_getuser(inner: &RwLock<AuthState>, args: &[CompactArg]) -> RespFrame {
    if args.len() != 3 {
        return RespFrame::error_static("ERR wrong number of arguments for 'ACL GETUSER' command");
    }
    let username = match std::str::from_utf8(args[2].as_slice()) {
        Ok(name) => name.to_ascii_lowercase(),
        Err(_) => return RespFrame::Bulk(None),
    };
    let state = inner.read();
    match state.users.get(&username) {
        Some(user) => user.getuser_reply(&username),
        None => RespFrame::Bulk(None),
    }
}

fn acl_cat(args: &[CompactArg]) -> RespFrame {
    if args.len() > 3 {
        return RespFrame::error_static("ERR wrong number of arguments for 'ACL CAT' command");
    }
    if args.len() == 2 {
        return RespFrame::Array(Some(
            acl_categories()
                .iter()
                .map(|category| {
                    RespFrame::Bulk(Some(BulkData::from_vec(
                        category.as_str().as_bytes().to_vec(),
                    )))
                })
                .collect(),
        ));
    }
    let category = match std::str::from_utf8(args[2].as_slice()) {
        Ok(category) => match AclCategory::parse(category) {
            Ok(category) => category,
            Err(err) => return RespFrame::Error(err),
        },
        Err(_) => return RespFrame::error_static("ERR invalid ACL category"),
    };
    RespFrame::Array(Some(
        commands_in_category(category)
            .into_iter()
            .map(|command| RespFrame::Bulk(Some(BulkData::from_vec(command.as_bytes().to_vec()))))
            .collect(),
    ))
}

fn acl_genpass(args: &[CompactArg]) -> RespFrame {
    if args.len() > 3 {
        return RespFrame::error_static("ERR wrong number of arguments for 'ACL GENPASS' command");
    }
    let bits = if args.len() == 3 {
        match std::str::from_utf8(args[2].as_slice())
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
        {
            Some(bits) if bits > 0 => bits,
            _ => {
                return RespFrame::error_static(
                    "ERR ACL GENPASS argument must be a positive number",
                );
            }
        }
    } else {
        256
    };
    let bytes = bits.div_ceil(8);
    let data = (0..bytes).map(|_| rand::random::<u8>()).collect::<Vec<_>>();
    let mut hex = String::with_capacity(data.len() * 2);
    for byte in data {
        hex.push(hex_nibble(byte >> 4));
        hex.push(hex_nibble(byte & 0x0f));
    }
    RespFrame::Bulk(Some(BulkData::from_vec(hex.into_bytes())))
}

fn acl_help(args: &[CompactArg]) -> RespFrame {
    if args.len() != 2 {
        return RespFrame::error_static("ERR wrong number of arguments for 'ACL HELP' command");
    }
    RespFrame::Array(Some(
        [
            "CAT [category] -- List ACL categories or commands in a category.",
            "DELUSER <username> [username ...] -- Remove one or more users.",
            "DRYRUN <username> <command> [arg ...] -- Check whether a user can run a command.",
            "GENPASS [bits] -- Generate a password with the requested entropy.",
            "GETUSER <username> -- Show the configuration for a user.",
            "HELP -- Show this help.",
            "LIST -- List all users in ACL syntax.",
            "LOAD -- Reload ACL state.",
            "LOG [count|RESET] -- Show ACL log entries.",
            "SAVE -- Persist ACL state.",
            "SETUSER <username> [rule ...] -- Update a user using ACL rules.",
            "USERS -- List usernames.",
            "WHOAMI -- Return the username of the current connection.",
        ]
        .into_iter()
        .map(|line| RespFrame::Bulk(Some(BulkData::from_vec(line.as_bytes().to_vec()))))
        .collect(),
    ))
}

fn acl_log(args: &[CompactArg]) -> RespFrame {
    if args.len() == 2 {
        return RespFrame::Array(Some(vec![]));
    }
    if args.len() == 3 && args[2].as_slice().eq_ignore_ascii_case(b"RESET") {
        return RespFrame::ok();
    }
    if args.len() == 3 {
        return RespFrame::Array(Some(vec![]));
    }
    RespFrame::error_static("ERR wrong number of arguments for 'ACL LOG' command")
}

fn acl_noop(args: &[CompactArg]) -> RespFrame {
    if args.len() != 2 {
        return RespFrame::error_static("ERR wrong number of arguments for ACL command");
    }
    RespFrame::ok()
}

fn acl_dryrun(inner: &RwLock<AuthState>, args: &[CompactArg]) -> RespFrame {
    if args.len() < 4 {
        return RespFrame::error_static("ERR wrong number of arguments for 'ACL DRYRUN' command");
    }
    let username = match std::str::from_utf8(args[2].as_slice()) {
        Ok(name) => name.to_ascii_lowercase(),
        Err(_) => return RespFrame::error_static("ERR invalid ACL username"),
    };
    let state = inner.read();
    let Some(user) = state.users.get(&username) else {
        return RespFrame::Error(format!(
            "WRONGPASS invalid username-password pair or user is disabled. user={username}"
        ));
    };
    match user.check_permissions(identify(args[3].as_slice()), &args[3..]) {
        Ok(()) => RespFrame::ok(),
        Err(error) => no_perm(match error {
            PermissionError::Command(command) => PermissionError::Command(command),
            PermissionError::Key => PermissionError::Key,
            PermissionError::Channel => PermissionError::Channel,
        }),
    }
}

fn hex_nibble(value: u8) -> char {
    match value {
        0..=9 => (b'0' + value) as char,
        10..=15 => (b'a' + (value - 10)) as char,
        _ => unreachable!("nibble out of range"),
    }
}
