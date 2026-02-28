use crate::commands::util::{eq_ascii, wrong_args, Args};
use crate::protocol::types::RespFrame;

pub fn handle(command: &[u8], args: &Args) -> Option<RespFrame> {
    if eq_ascii(command, b"AUTH") {
        return Some(auth(args));
    }
    if eq_ascii(command, b"HELLO") {
        return Some(hello(args));
    }
    if eq_ascii(command, b"CLIENT") {
        return Some(client(args));
    }
    if eq_ascii(command, b"COMMAND") {
        return Some(RespFrame::Array(Some(vec![])));
    }
    if eq_ascii(command, b"SELECT") {
        return Some(select_db(args));
    }
    if eq_ascii(command, b"QUIT") {
        return Some(quit(args));
    }
    if eq_ascii(command, b"PING") {
        return Some(ping(args));
    }
    if eq_ascii(command, b"ECHO") {
        return Some(echo(args));
    }
    None
}

fn auth(args: &Args) -> RespFrame {
    if args.is_empty() || args.len() > 3 {
        return wrong_args("AUTH");
    }
    RespFrame::ok()
}

fn hello(args: &Args) -> RespFrame {
    if args.len() == 1 {
        return hello_response(2);
    }

    let version = match std::str::from_utf8(&args[1]) {
        Ok(raw) => raw.parse::<u8>().ok(),
        Err(_) => None,
    };

    match version {
        Some(proto) if proto == 2 || proto == 3 => hello_response(proto),
        _ => RespFrame::Error("NOPROTO unsupported protocol version".to_string()),
    }
}

fn hello_response(proto: u8) -> RespFrame {
    if proto == 3 {
        return RespFrame::Map(vec![
            (
                RespFrame::Bulk(Some(b"server".to_vec())),
                RespFrame::Bulk(Some(b"valkey".to_vec())),
            ),
            (
                RespFrame::Bulk(Some(b"version".to_vec())),
                RespFrame::Bulk(Some(env!("CARGO_PKG_VERSION").as_bytes().to_vec())),
            ),
            (
                RespFrame::Bulk(Some(b"proto".to_vec())),
                RespFrame::Integer(3),
            ),
            (RespFrame::Bulk(Some(b"id".to_vec())), RespFrame::Integer(1)),
            (
                RespFrame::Bulk(Some(b"mode".to_vec())),
                RespFrame::Bulk(Some(b"standalone".to_vec())),
            ),
            (
                RespFrame::Bulk(Some(b"role".to_vec())),
                RespFrame::Bulk(Some(b"master".to_vec())),
            ),
            (
                RespFrame::Bulk(Some(b"modules".to_vec())),
                RespFrame::Array(Some(vec![])),
            ),
        ]);
    }

    RespFrame::Array(Some(vec![
        RespFrame::Bulk(Some(b"server".to_vec())),
        RespFrame::Bulk(Some(b"valkey".to_vec())),
        RespFrame::Bulk(Some(b"version".to_vec())),
        RespFrame::Bulk(Some(env!("CARGO_PKG_VERSION").as_bytes().to_vec())),
        RespFrame::Bulk(Some(b"proto".to_vec())),
        RespFrame::Integer(proto as i64),
        RespFrame::Bulk(Some(b"id".to_vec())),
        RespFrame::Integer(1),
        RespFrame::Bulk(Some(b"mode".to_vec())),
        RespFrame::Bulk(Some(b"standalone".to_vec())),
        RespFrame::Bulk(Some(b"role".to_vec())),
        RespFrame::Bulk(Some(b"master".to_vec())),
        RespFrame::Bulk(Some(b"modules".to_vec())),
        RespFrame::Array(Some(vec![])),
    ]))
}

fn client(args: &Args) -> RespFrame {
    if args.len() < 2 {
        return wrong_args("CLIENT");
    }

    let sub = args[1].as_slice();
    if eq_ascii(sub, b"SETINFO") || eq_ascii(sub, b"SETNAME") {
        RespFrame::ok()
    } else if eq_ascii(sub, b"GETNAME") {
        RespFrame::Bulk(None)
    } else if eq_ascii(sub, b"ID") {
        RespFrame::Integer(1)
    } else {
        RespFrame::Error("ERR unknown subcommand for CLIENT".to_string())
    }
}

fn select_db(args: &Args) -> RespFrame {
    if args.len() != 2 {
        return wrong_args("SELECT");
    }

    let index = match std::str::from_utf8(&args[1]) {
        Ok(raw) => raw.parse::<u32>().ok(),
        Err(_) => None,
    };

    match index {
        Some(0) => RespFrame::ok(),
        Some(_) => RespFrame::Error("ERR DB index is out of range".to_string()),
        None => RespFrame::Error("ERR value is not an integer or out of range".to_string()),
    }
}

fn quit(args: &Args) -> RespFrame {
    if args.len() != 1 {
        return wrong_args("QUIT");
    }
    RespFrame::ok()
}

fn ping(args: &Args) -> RespFrame {
    if args.len() == 1 {
        return RespFrame::Simple("PONG".to_string());
    }
    if args.len() == 2 {
        return RespFrame::Bulk(Some(args[1].clone()));
    }
    wrong_args("PING")
}

fn echo(args: &Args) -> RespFrame {
    if args.len() != 2 {
        return wrong_args("ECHO");
    }
    RespFrame::Bulk(Some(args[1].clone()))
}
