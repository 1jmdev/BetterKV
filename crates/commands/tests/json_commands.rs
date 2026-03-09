use commands::dispatcher::dispatch_args;
use engine::store::Store;
use protocol::types::{BulkData, RespFrame};
use types::value::CompactArg;

fn arg(value: &str) -> CompactArg {
    CompactArg::from_slice(value.as_bytes())
}

#[test]
fn json_set_and_get_root_document() {
    let store = Store::new(1);

    let set_args = [
        arg("JSON.SET"),
        arg("doc"),
        arg("$"),
        arg(r#"{"name":"Alice"}"#),
    ];
    assert_eq!(dispatch_args(&store, &set_args), RespFrame::ok());

    let get_args = [arg("JSON.GET"), arg("doc"), arg("$")];
    assert_eq!(
        dispatch_args(&store, &get_args),
        RespFrame::Bulk(Some(BulkData::from_vec(br#"{"name":"Alice"}"#.to_vec())))
    );
}

#[test]
fn json_array_commands_mutate_in_place() {
    let store = Store::new(1);

    assert_eq!(
        dispatch_args(
            &store,
            &[arg("JSON.SET"), arg("arr"), arg("$"), arg("[1,2,3]")]
        ),
        RespFrame::ok()
    );
    assert_eq!(
        dispatch_args(
            &store,
            &[arg("JSON.ARRAPPEND"), arg("arr"), arg("$"), arg("4")]
        ),
        RespFrame::Integer(4)
    );
    assert_eq!(
        dispatch_args(&store, &[arg("JSON.ARRPOP"), arg("arr"), arg("$")]),
        RespFrame::Bulk(Some(BulkData::from_vec(b"4".to_vec())))
    );
}

#[test]
fn json_object_and_numeric_commands_work() {
    let store = Store::new(1);

    assert_eq!(
        dispatch_args(
            &store,
            &[
                arg("JSON.SET"),
                arg("doc"),
                arg("$"),
                arg(r#"{"count":5,"obj":{"a":1}}"#)
            ]
        ),
        RespFrame::ok()
    );
    assert_eq!(
        dispatch_args(&store, &[arg("JSON.OBJLEN"), arg("doc"), arg("$.obj")]),
        RespFrame::Integer(1)
    );
    assert_eq!(
        dispatch_args(
            &store,
            &[arg("JSON.NUMINCRBY"), arg("doc"), arg("$.count"), arg("3")]
        ),
        RespFrame::Bulk(Some(BulkData::from_vec(b"8".to_vec())))
    );
}

#[test]
fn json_mget_returns_nil_for_missing_key() {
    let store = Store::new(1);

    assert_eq!(
        dispatch_args(
            &store,
            &[arg("JSON.SET"), arg("k1"), arg("$"), arg(r#"{"a":1}"#)]
        ),
        RespFrame::ok()
    );

    assert_eq!(
        dispatch_args(
            &store,
            &[arg("JSON.MGET"), arg("k1"), arg("missing"), arg("$.a")]
        ),
        RespFrame::Array(Some(vec![
            RespFrame::Bulk(Some(BulkData::from_vec(b"1".to_vec()))),
            RespFrame::Bulk(None)
        ]))
    );
}
