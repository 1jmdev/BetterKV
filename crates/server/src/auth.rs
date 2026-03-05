use std::collections::BTreeMap;
use std::sync::Arc;

use parking_lot::RwLock;
use protocol::types::{BulkData, RespFrame};
use types::value::CompactArg;

use crate::config::Config;

const DEFAULT_USER: &str = "default";

#[derive(Clone, Debug)]
pub struct UserDirectiveConfig {
    pub name: String,
    pub rules: Vec<String>,
}

#[derive(Clone)]
pub struct AuthService {
    inner: Arc<RwLock<AuthState>>,
}

#[derive(Clone, Debug)]
pub struct SessionAuth {
    user: Option<String>,
    authorized: bool,
}

#[derive(Clone, Debug)]
struct AuthState {
    users: BTreeMap<String, User>,
}

#[derive(Clone, Debug)]
struct User {
    enabled: bool,
    nopass: bool,
    passwords: Vec<Vec<u8>>,
}

impl Default for User {
    fn default() -> Self {
        let _trace = profiler::scope("server::auth::default_user");
        Self {
            enabled: true,
            nopass: true,
            passwords: Vec::new(),
        }
    }
}

impl AuthService {
    pub fn from_config(config: &Config) -> Result<Self, String> {
        let _trace = profiler::scope("server::auth::from_config");
        let mut users = BTreeMap::new();
        users.insert(DEFAULT_USER.to_string(), User::default());

        if let Some(requirepass) = &config.requirepass {
            let default = users
                .get_mut(DEFAULT_USER)
                .expect("default user is always present");
            default.enabled = true;
            default.nopass = false;
            default.passwords.clear();
            default.passwords.push(requirepass.as_bytes().to_vec());
        }

        let mut state = AuthState { users };
        for user_directive in &config.user_directives {
            state.set_user(&user_directive.name, &user_directive.rules)?;
        }

        Ok(Self {
            inner: Arc::new(RwLock::new(state)),
        })
    }

    pub fn new_session(&self) -> SessionAuth {
        let _trace = profiler::scope("server::auth::new_session");
        let state = self.inner.read();
        if state.default_user_auto_auth() {
            SessionAuth {
                user: Some(DEFAULT_USER.to_string()),
                authorized: true,
            }
        } else {
            SessionAuth {
                user: None,
                authorized: false,
            }
        }
    }

    pub fn is_authorized(&self, session: &SessionAuth) -> bool {
        let _trace = profiler::scope("server::auth::is_authorized");
        session.is_authorized()
    }

    pub fn authenticate(&self, username: &[u8], password: &[u8]) -> Result<String, AuthError> {
        let _trace = profiler::scope("server::auth::authenticate");
        let username = std::str::from_utf8(username)
            .map_err(|_| AuthError::WrongPass)?
            .to_ascii_lowercase();

        let state = self.inner.read();
        let Some(user) = state.users.get(&username) else {
            return Err(AuthError::WrongPass);
        };

        if !user.enabled {
            return Err(AuthError::WrongPass);
        }
        if user.nopass {
            return Ok(username);
        }
        if user.passwords.is_empty() {
            return Err(AuthError::NoPasswordSet);
        }
        if user.passwords.iter().any(|candidate| candidate == password) {
            Ok(username)
        } else {
            Err(AuthError::WrongPass)
        }
    }

    pub fn acl_command(&self, session: &SessionAuth, args: &[CompactArg]) -> RespFrame {
        let _trace = profiler::scope("server::auth::acl_command");
        if !session.is_authorized() {
            return no_auth();
        }
        if args.len() < 2 {
            return RespFrame::error_static("ERR wrong number of arguments for 'ACL' command");
        }

        let sub = args[1].as_slice();
        if sub.eq_ignore_ascii_case(b"WHOAMI") {
            if args.len() != 2 {
                return RespFrame::error_static(
                    "ERR wrong number of arguments for 'ACL WHOAMI' command",
                );
            }
            let current = session
                .user
                .clone()
                .unwrap_or_else(|| DEFAULT_USER.to_string());
            return RespFrame::Bulk(Some(BulkData::from_vec(current.into_bytes())));
        }

        if sub.eq_ignore_ascii_case(b"USERS") {
            if args.len() != 2 {
                return RespFrame::error_static(
                    "ERR wrong number of arguments for 'ACL USERS' command",
                );
            }
            let users = self
                .inner
                .read()
                .users
                .keys()
                .map(|name| RespFrame::Bulk(Some(BulkData::from_vec(name.as_bytes().to_vec()))))
                .collect();
            return RespFrame::Array(Some(users));
        }

        if sub.eq_ignore_ascii_case(b"LIST") {
            if args.len() != 2 {
                return RespFrame::error_static(
                    "ERR wrong number of arguments for 'ACL LIST' command",
                );
            }
            let lines = self
                .inner
                .read()
                .users
                .iter()
                .map(|(name, user)| user_acl_line(name, user))
                .map(|line| RespFrame::Bulk(Some(BulkData::from_vec(line.into_bytes()))))
                .collect();
            return RespFrame::Array(Some(lines));
        }

        if sub.eq_ignore_ascii_case(b"SETUSER") {
            if args.len() < 3 {
                return RespFrame::error_static(
                    "ERR wrong number of arguments for 'ACL SETUSER' command",
                );
            }
            let username = match std::str::from_utf8(args[2].as_slice()) {
                Ok(name) => name.to_ascii_lowercase(),
                Err(_) => return RespFrame::error_static("ERR invalid ACL username"),
            };
            let rules = args[3..]
                .iter()
                .map(|arg| String::from_utf8_lossy(arg.as_slice()).to_string())
                .collect::<Vec<_>>();

            let mut state = self.inner.write();
            match state.set_user(&username, &rules) {
                Ok(()) => RespFrame::ok(),
                Err(err) => RespFrame::Error(err),
            }
        } else if sub.eq_ignore_ascii_case(b"DELUSER") {
            if args.len() < 3 {
                return RespFrame::error_static(
                    "ERR wrong number of arguments for 'ACL DELUSER' command",
                );
            }
            let mut state = self.inner.write();
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
            RespFrame::Integer(removed)
        } else {
            RespFrame::error_static("ERR Unknown ACL subcommand")
        }
    }

    pub fn default_user_has_password(&self) -> bool {
        let _trace = profiler::scope("server::auth::default_user_has_password");
        let state = self.inner.read();
        state.default_user_has_password()
    }
}

impl SessionAuth {
    pub fn user(&self) -> Option<&str> {
        let _trace = profiler::scope("server::auth::session_user");
        self.user.as_deref()
    }

    pub fn set_user(&mut self, user: String) {
        let _trace = profiler::scope("server::auth::session_set_user");
        self.user = Some(user);
        self.authorized = true;
    }

    pub fn is_authorized(&self) -> bool {
        let _trace = profiler::scope("server::auth::session_is_authorized");
        self.authorized
    }
}

impl AuthState {
    fn default_user_has_password(&self) -> bool {
        self.users
            .get(DEFAULT_USER)
            .is_some_and(|default_user| !default_user.nopass && !default_user.passwords.is_empty())
    }

    fn default_user_auto_auth(&self) -> bool {
        self.users
            .get(DEFAULT_USER)
            .is_some_and(|user| user.enabled && user.nopass)
    }

    fn set_user(&mut self, username: &str, rules: &[String]) -> Result<(), String> {
        let _trace = profiler::scope("server::auth::set_user");
        let user = self.users.entry(username.to_string()).or_insert(User {
            enabled: false,
            nopass: false,
            passwords: Vec::new(),
        });
        for rule in rules {
            if rule.eq_ignore_ascii_case("on") {
                user.enabled = true;
                continue;
            }
            if rule.eq_ignore_ascii_case("off") {
                user.enabled = false;
                continue;
            }
            if rule.eq_ignore_ascii_case("nopass") {
                user.nopass = true;
                user.passwords.clear();
                continue;
            }
            if rule.eq_ignore_ascii_case("resetpass") {
                user.nopass = false;
                user.passwords.clear();
                continue;
            }
            if rule.eq_ignore_ascii_case("reset") {
                *user = User {
                    enabled: false,
                    nopass: false,
                    passwords: Vec::new(),
                };
                continue;
            }
            if let Some(password) = rule.strip_prefix('>') {
                user.nopass = false;
                user.passwords.push(password.as_bytes().to_vec());
                continue;
            }
            if let Some(password) = rule.strip_prefix('<') {
                user.passwords.retain(|item| item != password.as_bytes());
                continue;
            }

            if is_ignored_acl_rule(rule) {
                continue;
            }
            return Err(format!("ERR Unsupported ACL rule '{rule}'"));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AuthError {
    WrongPass,
    NoPasswordSet,
}

pub fn parse_user_directive(values: &[String]) -> Result<UserDirectiveConfig, String> {
    let _trace = profiler::scope("server::auth::parse_user_directive");
    if values.is_empty() {
        return Err("directive 'user' requires at least a username".to_string());
    }

    Ok(UserDirectiveConfig {
        name: values[0].to_ascii_lowercase(),
        rules: values[1..].to_vec(),
    })
}

pub fn no_auth() -> RespFrame {
    let _trace = profiler::scope("server::auth::no_auth");
    RespFrame::error_static("NOAUTH Authentication required.")
}

fn user_acl_line(name: &str, user: &User) -> String {
    let _trace = profiler::scope("server::auth::user_acl_line");
    let mut parts = vec![format!("user {name}")];
    parts.push(if user.enabled {
        "on".to_string()
    } else {
        "off".to_string()
    });

    if user.nopass {
        parts.push("nopass".to_string());
    } else if user.passwords.is_empty() {
        parts.push("resetpass".to_string());
    } else {
        for password in &user.passwords {
            parts.push(format!(">{}", String::from_utf8_lossy(password)));
        }
    }

    parts.push("~*".to_string());
    parts.push("+@all".to_string());
    parts.join(" ")
}

fn is_ignored_acl_rule(rule: &str) -> bool {
    let _trace = profiler::scope("server::auth::is_ignored_acl_rule");
    if rule.starts_with('~') || rule.starts_with('&') {
        return true;
    }
    if rule.starts_with('+') || rule.starts_with('-') {
        return true;
    }
    matches!(
        rule.to_ascii_lowercase().as_str(),
        "allkeys" | "resetkeys" | "allchannels" | "resetchannels" | "allcommands" | "nocommands"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn requirepass_enforces_auth() {
        let _trace = profiler::scope("server::auth::tests::requirepass_enforces_auth");
        let config = Config {
            requirepass: Some("secret".to_string()),
            ..Config::default()
        };
        let auth = AuthService::from_config(&config).expect("auth service");
        let session = auth.new_session();
        assert!(!auth.is_authorized(&session));
        assert!(matches!(
            auth.authenticate(b"default", b"secret"),
            Ok(user) if user == "default"
        ));
    }

    #[test]
    fn user_directive_supports_custom_user() {
        let _trace = profiler::scope("server::auth::tests::user_directive_supports_custom_user");
        let config = Config {
            user_directives: vec![UserDirectiveConfig {
                name: "alice".to_string(),
                rules: vec!["on".to_string(), ">wonderland".to_string()],
            }],
            ..Config::default()
        };
        let auth = AuthService::from_config(&config).expect("auth service");
        assert!(matches!(
            auth.authenticate(b"alice", b"wonderland"),
            Ok(user) if user == "alice"
        ));
        assert!(matches!(
            auth.authenticate(b"alice", b"wrong"),
            Err(AuthError::WrongPass)
        ));
    }

    #[test]
    fn default_config_auto_authorizes_default_user() {
        let _trace =
            profiler::scope("server::auth::tests::default_config_auto_authorizes_default_user");
        let auth = AuthService::from_config(&Config::default()).expect("auth service");
        let session = auth.new_session();
        assert!(session.is_authorized());
        assert_eq!(session.user(), Some("default"));
    }
}
