use commands::dispatch::{
    CommandId, NotificationKeyArguments, NotificationResponsePolicy, command_notification_spec,
};
use protocol::types::RespFrame;
use types::value::CompactArg;

use super::super::pubsub::PubSubHub;

pub(super) fn emit_command_notifications(
    hub: &PubSubHub,
    command: CommandId,
    args: &[CompactArg],
    response: &RespFrame,
) {
    let _trace = profiler::scope("server::connection::notifications::emit_command_notifications");
    let Some(spec) = command_notification_spec(command) else {
        return;
    };

    if !response_matches(spec.response, response) {
        return;
    }

    emit_keys(hub, spec.event, spec.class, spec.keys, args);
}

fn response_matches(policy: NotificationResponsePolicy, response: &RespFrame) -> bool {
    let _trace = profiler::scope("server::connection::notifications::response_matches");
    match policy {
        NotificationResponsePolicy::AnySuccess => {
            !matches!(response, RespFrame::Error(_) | RespFrame::ErrorStatic(_))
        }
        NotificationResponsePolicy::IntegerOne => matches!(response, RespFrame::Integer(1)),
        NotificationResponsePolicy::PositiveInteger => {
            matches!(response, RespFrame::Integer(value) if *value > 0)
        }
        NotificationResponsePolicy::OkOrIntegerOne => {
            matches!(response, RespFrame::Simple(value) if value == "OK")
                || matches!(response, RespFrame::Integer(1))
        }
    }
}

fn emit_keys(
    hub: &PubSubHub,
    event: &'static [u8],
    class: u8,
    keys: NotificationKeyArguments,
    args: &[CompactArg],
) {
    let _trace = profiler::scope("server::connection::notifications::emit_keys");
    match keys {
        NotificationKeyArguments::Argument(index) => {
            if let Some(key) = args.get(index) {
                hub.emit_keyspace_event(event, key.as_slice(), class);
            }
        }
        NotificationKeyArguments::AllFrom(index) => {
            for key in args.iter().skip(index) {
                hub.emit_keyspace_event(event, key.as_slice(), class);
            }
        }
        NotificationKeyArguments::EveryOtherFrom(index) => {
            for key in args.iter().skip(index).step_by(2) {
                hub.emit_keyspace_event(event, key.as_slice(), class);
            }
        }
    }
}
