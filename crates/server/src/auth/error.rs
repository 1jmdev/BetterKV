use protocol::types::RespFrame;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthError {
    WrongPass,
    NoPasswordSet,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PermissionError {
    Command(String),
    Key,
    Channel,
}

pub fn no_auth() -> RespFrame {
    RespFrame::error_static("NOAUTH Authentication required.")
}

pub fn no_perm(error: PermissionError) -> RespFrame {
    match error {
        PermissionError::Command(command) => RespFrame::Error(format!(
            "NOPERM this user has no permissions to run the '{command}' command"
        )),
        PermissionError::Key => RespFrame::error_static(
            "NOPERM this user has no permissions to access one of the keys used as arguments",
        ),
        PermissionError::Channel => RespFrame::error_static(
            "NOPERM this user has no permissions to access one of the channels used as arguments",
        ),
    }
}
