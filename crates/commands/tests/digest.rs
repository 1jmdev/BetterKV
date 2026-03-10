use commands::dispatch::dispatch_args;
use engine::store::Store;
use protocol::types::{BulkData, RespFrame};
use types::value::CompactArg;

fn arg(value: &str) -> CompactArg {
    CompactArg::from_slice(value.as_bytes())
}

fn run(store: &Store, args: &[&str]) -> RespFrame {
    let parsed: Vec<CompactArg> = args.iter().map(|value| arg(value)).collect();
    dispatch_args(store, &parsed)
}

#[test]
fn digest_returns_xxh3_hex_for_string_values() {
    let store = Store::new(1);

    assert_eq!(run(&store, &["SET", "key", "value"]), RespFrame::ok());
    let expected = match store.digest(b"key") {
        Ok(Some(value)) => value,
        Ok(None) => panic!("digest should exist"),
        Err(()) => panic!("digest should not fail for string"),
    };

    assert_eq!(
        run(&store, &["DIGEST", "key"]),
        RespFrame::Bulk(Some(BulkData::from_vec(expected)))
    );
}

#[test]
fn digest_rejects_non_string_keys() {
    let store = Store::new(1);

    assert_eq!(
        run(&store, &["LPUSH", "list", "value"]),
        RespFrame::Integer(1)
    );
    assert_eq!(
        run(&store, &["DIGEST", "list"]),
        RespFrame::error_static(
            "WRONGTYPE Operation against a key holding the wrong kind of value"
        )
    );
}
