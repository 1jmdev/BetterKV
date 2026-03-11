use protocol::types::RespFrame;

pub(super) fn wrong_args(command: &str) -> RespFrame {
    let command = command.to_ascii_lowercase();
    RespFrame::Error(format!(
        "ERR wrong number of arguments for '{command}' command"
    ))
}
