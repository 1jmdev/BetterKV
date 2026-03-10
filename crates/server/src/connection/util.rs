use protocol::types::RespFrame;

pub(super) fn wrong_args(command: &str) -> RespFrame {
    let _trace = profiler::scope("server::connection::util::wrong_args");
    let command = command.to_ascii_lowercase();
    RespFrame::Error(format!(
        "ERR wrong number of arguments for '{command}' command"
    ))
}
