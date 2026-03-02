mod support;

use std::time::Duration;

use justkv::protocol::types::RespFrame;
use support::{connect, send_command, spawn_server};

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn string_variants_work() {
    let (server, port) = spawn_server().await;
    let mut conn = connect(port).await;

    assert_eq!(
        send_command(&mut conn, &[b"SETNX", b"k", b"1"]).await,
        RespFrame::Integer(1)
    );
    assert_eq!(
        send_command(&mut conn, &[b"SETNX", b"k", b"2"]).await,
        RespFrame::Integer(0)
    );
    assert_eq!(
        send_command(&mut conn, &[b"APPEND", b"k", b"23"]).await,
        RespFrame::Integer(3)
    );
    assert_eq!(
        send_command(&mut conn, &[b"STRLEN", b"k"]).await,
        RespFrame::Integer(3)
    );

    assert_eq!(
        send_command(&mut conn, &[b"INCRBY", b"counter", b"5"]).await,
        RespFrame::Integer(5)
    );
    assert_eq!(
        send_command(&mut conn, &[b"DECR", b"counter"]).await,
        RespFrame::Integer(4)
    );
    assert_eq!(
        send_command(&mut conn, &[b"DECRBY", b"counter", b"3"]).await,
        RespFrame::Integer(1)
    );

    server.abort();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn keyspace_commands_work() {
    let (server, port) = spawn_server().await;
    let mut conn = connect(port).await;

    let _ = send_command(&mut conn, &[b"MSET", b"a", b"1", b"ab", b"2", b"b", b"3"]).await;
    assert_eq!(
        send_command(&mut conn, &[b"DBSIZE"]).await,
        RespFrame::Integer(3)
    );
    assert_eq!(
        send_command(&mut conn, &[b"TYPE", b"a"]).await,
        RespFrame::Simple("string".to_string())
    );

    let keys = send_command(&mut conn, &[b"KEYS", b"a*"]).await;
    match keys {
        RespFrame::Array(Some(values)) => assert_eq!(values.len(), 2),
        other => panic!("unexpected KEYS response: {other:?}"),
    }

    assert_eq!(
        send_command(&mut conn, &[b"RENAME", b"a", b"x"]).await,
        RespFrame::Simple("OK".to_string())
    );
    assert_eq!(
        send_command(&mut conn, &[b"RENAMENX", b"ab", b"x"]).await,
        RespFrame::Integer(0)
    );
    assert_eq!(
        send_command(&mut conn, &[b"FLUSHDB"]).await,
        RespFrame::Simple("OK".to_string())
    );
    assert_eq!(
        send_command(&mut conn, &[b"DBSIZE"]).await,
        RespFrame::Integer(0)
    );

    server.abort();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn ttl_variants_work() {
    let (server, port) = spawn_server().await;
    let mut conn = connect(port).await;

    let _ = send_command(&mut conn, &[b"SET", b"exp", b"v"]).await;
    assert_eq!(
        send_command(&mut conn, &[b"PEXPIRE", b"exp", b"900"]).await,
        RespFrame::Integer(1)
    );

    match send_command(&mut conn, &[b"PTTL", b"exp"]).await {
        RespFrame::Integer(value) => assert!(value > 0),
        other => panic!("unexpected PTTL response: {other:?}"),
    }

    assert_eq!(
        send_command(&mut conn, &[b"PERSIST", b"exp"]).await,
        RespFrame::Integer(1)
    );
    assert_eq!(
        send_command(&mut conn, &[b"PTTL", b"exp"]).await,
        RespFrame::Integer(-1)
    );

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("unix epoch")
        .as_secs();
    assert_eq!(
        send_command(
            &mut conn,
            &[b"EXPIREAT", b"exp", (now + 1).to_string().as_bytes()]
        )
        .await,
        RespFrame::Integer(1)
    );

    tokio::time::sleep(Duration::from_millis(1200)).await;
    assert_eq!(
        send_command(&mut conn, &[b"TTL", b"exp"]).await,
        RespFrame::Integer(-2)
    );

    server.abort();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn bitmap_variants_work() {
    let (server, port) = spawn_server().await;
    let mut conn = connect(port).await;

    assert_eq!(
        send_command(&mut conn, &[b"SETBIT", b"bits", b"1", b"1"]).await,
        RespFrame::Integer(0)
    );
    assert_eq!(
        send_command(&mut conn, &[b"GETBIT", b"bits", b"1"]).await,
        RespFrame::Integer(1)
    );
    assert_eq!(
        send_command(&mut conn, &[b"BITCOUNT", b"bits"]).await,
        RespFrame::Integer(1)
    );
    assert_eq!(
        send_command(&mut conn, &[b"BITPOS", b"bits", b"1"]).await,
        RespFrame::Integer(1)
    );

    let _ = send_command(&mut conn, &[b"SET", b"left", b"\x0f"]).await;
    let _ = send_command(&mut conn, &[b"SET", b"right", b"\xf0"]).await;
    assert_eq!(
        send_command(&mut conn, &[b"BITOP", b"OR", b"merged", b"left", b"right"]).await,
        RespFrame::Integer(1)
    );

    assert_eq!(
        send_command(
            &mut conn,
            &[
                b"BITFIELD",
                b"bf",
                b"SET",
                b"u4",
                b"0",
                b"9",
                b"GET",
                b"u4",
                b"0"
            ]
        )
        .await,
        RespFrame::Array(Some(vec![RespFrame::Integer(0), RespFrame::Integer(9)]))
    );
    assert_eq!(
        send_command(&mut conn, &[b"BITFIELD_RO", b"bf", b"GET", b"u4", b"0"]).await,
        RespFrame::Array(Some(vec![RespFrame::Integer(9)]))
    );

    server.abort();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn hyperlog_variants_work() {
    let (server, port) = spawn_server().await;
    let mut conn = connect(port).await;

    assert_eq!(
        send_command(&mut conn, &[b"PFADD", b"h", b"a", b"b", b"c"]).await,
        RespFrame::Integer(1)
    );
    assert_eq!(
        send_command(&mut conn, &[b"PFADD", b"h", b"a", b"b"]).await,
        RespFrame::Integer(0)
    );
    assert_eq!(
        send_command(&mut conn, &[b"PFCOUNT", b"h"]).await,
        RespFrame::Integer(3)
    );

    let _ = send_command(&mut conn, &[b"PFADD", b"h2", b"c", b"d"]).await;
    assert_eq!(
        send_command(&mut conn, &[b"PFMERGE", b"h3", b"h", b"h2"]).await,
        RespFrame::Simple("OK".to_string())
    );
    assert_eq!(
        send_command(&mut conn, &[b"PFCOUNT", b"h3"]).await,
        RespFrame::Integer(4)
    );
    assert_eq!(
        send_command(&mut conn, &[b"PFCOUNT", b"h", b"h2"]).await,
        RespFrame::Integer(4)
    );

    server.abort();
}
