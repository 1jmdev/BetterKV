use commands::dispatch::CommandId;
use std::collections::BTreeSet;

use types::value::CompactArg;

use super::command::{AclCategory, CommandSpec, command_spec};
use super::error::PermissionError;
use super::password::{PasswordHash, format_password_hash, parse_password_hash, password_hash};
use super::pattern::any_pattern_matches;

#[derive(Clone, Debug)]
pub(super) struct User {
    pub(super) enabled: bool,
    pub(super) nopass: bool,
    pub(super) password_hashes: Vec<PasswordHash>,
    pub(super) key_patterns: Vec<Vec<u8>>,
    pub(super) channel_patterns: Vec<Vec<u8>>,
    pub(super) command_rules: CommandRules,
}

#[derive(Clone, Debug)]
pub(super) struct CommandRules {
    allow_all: bool,
    allowed_commands: BTreeSet<String>,
    denied_commands: BTreeSet<String>,
    allowed_categories: BTreeSet<AclCategory>,
    denied_categories: BTreeSet<AclCategory>,
}

impl Default for User {
    fn default() -> Self {
        let _trace = profiler::scope("server::auth::default_user");
        Self {
            enabled: true,
            nopass: true,
            password_hashes: Vec::new(),
            key_patterns: vec![b"*".to_vec()],
            channel_patterns: vec![b"*".to_vec()],
            command_rules: CommandRules::allow_all(),
        }
    }
}

impl User {
    #[inline(always)]
    pub(super) fn acl_check_required(&self) -> bool {
        !self.command_rules.is_allow_all_unrestricted()
            || !is_all_pattern(&self.key_patterns)
            || !is_all_pattern(&self.channel_patterns)
    }

    pub(super) fn new_restricted() -> Self {
        let _trace = profiler::scope("server::auth::new_restricted_user");
        Self {
            enabled: false,
            nopass: false,
            password_hashes: Vec::new(),
            key_patterns: Vec::new(),
            channel_patterns: Vec::new(),
            command_rules: CommandRules::deny_all(),
        }
    }

    pub(super) fn reset(&mut self) {
        let _trace = profiler::scope("server::auth::reset_user");
        *self = Self::new_restricted();
    }

    pub(super) fn add_password(&mut self, password: &[u8]) {
        let _trace = profiler::scope("server::auth::add_password");
        self.nopass = false;
        let hash = password_hash(password);
        if !self.password_hashes.contains(&hash) {
            self.password_hashes.push(hash);
        }
    }

    pub(super) fn remove_password(&mut self, password: &[u8]) {
        let _trace = profiler::scope("server::auth::remove_password");
        let hash = password_hash(password);
        self.password_hashes.retain(|candidate| candidate != &hash);
    }

    pub(super) fn add_password_hash(&mut self, hash: &str) -> Result<(), String> {
        let _trace = profiler::scope("server::auth::add_password_hash");
        self.nopass = false;
        let hash = parse_password_hash(hash)?;
        if !self.password_hashes.contains(&hash) {
            self.password_hashes.push(hash);
        }
        Ok(())
    }

    pub(super) fn remove_password_hash(&mut self, hash: &str) -> Result<(), String> {
        let _trace = profiler::scope("server::auth::remove_password_hash");
        let hash = parse_password_hash(hash)?;
        self.password_hashes.retain(|candidate| candidate != &hash);
        Ok(())
    }

    pub(super) fn check_password(&self, password: &[u8]) -> bool {
        let _trace = profiler::scope("server::auth::check_password");
        self.nopass || self.password_hashes.contains(&password_hash(password))
    }

    pub(super) fn password_tokens(&self) -> Vec<String> {
        let _trace = profiler::scope("server::auth::password_tokens");
        if self.nopass {
            return vec!["nopass".to_string()];
        }
        if self.password_hashes.is_empty() {
            return vec!["resetpass".to_string()];
        }
        self.password_hashes
            .iter()
            .map(|hash| format!("#{}", format_password_hash(hash)))
            .collect()
    }

    pub(super) fn acl_tokens(&self) -> Vec<String> {
        let _trace = profiler::scope("server::auth::acl_tokens");
        let mut tokens = Vec::new();
        tokens.push(if self.enabled {
            "on".to_string()
        } else {
            "off".to_string()
        });
        tokens.extend(self.password_tokens());
        tokens.extend(pattern_tokens(
            &self.key_patterns,
            "allkeys",
            "resetkeys",
            '~',
        ));
        tokens.extend(pattern_tokens(
            &self.channel_patterns,
            "allchannels",
            "resetchannels",
            '&',
        ));
        tokens.extend(self.command_rules.tokens());
        tokens
    }

    pub(super) fn acl_line(&self, name: &str) -> String {
        let _trace = profiler::scope("server::auth::acl_line");
        let mut parts = vec![format!("user {name}")];
        parts.extend(self.acl_tokens());
        parts.join(" ")
    }

    pub(super) fn check_permissions(
        &self,
        command: CommandId,
        args: &[CompactArg],
    ) -> Result<(), PermissionError> {
        let _trace = profiler::scope("server::auth::check_permissions");
        let spec = command_spec(command).unwrap_or_else(CommandSpec::unknown);

        if !self.command_rules.allows(&spec) {
            return Err(PermissionError::Command(
                command.name().map(str::to_string).unwrap_or_else(|| {
                    String::from_utf8_lossy(
                        args.first().map(CompactArg::as_slice).unwrap_or_default(),
                    )
                    .to_string()
                }),
            ));
        }

        if !spec
            .keys(args)
            .iter()
            .all(|key| any_pattern_matches(&self.key_patterns, key))
        {
            return Err(PermissionError::Key);
        }

        if !spec
            .channels(args)
            .iter()
            .all(|channel| any_pattern_matches(&self.channel_patterns, channel))
        {
            return Err(PermissionError::Channel);
        }

        Ok(())
    }

    pub(super) fn getuser_reply(&self, name: &str) -> protocol::types::RespFrame {
        let _trace = profiler::scope("server::auth::getuser_reply");
        use protocol::types::{BulkData, RespFrame};

        let flags = RespFrame::Array(Some(
            self.acl_tokens()
                .into_iter()
                .filter(|token| {
                    matches!(
                        token.as_str(),
                        "on" | "off"
                            | "nopass"
                            | "resetpass"
                            | "allkeys"
                            | "resetkeys"
                            | "allchannels"
                            | "resetchannels"
                    ) || token.starts_with('#')
                })
                .map(|token| RespFrame::Bulk(Some(BulkData::from_vec(token.into_bytes()))))
                .collect(),
        ));
        let passwords = RespFrame::Array(Some(
            self.password_hashes
                .iter()
                .map(|hash| {
                    RespFrame::Bulk(Some(BulkData::from_vec(
                        format_password_hash(hash).into_bytes(),
                    )))
                })
                .collect(),
        ));
        let commands = RespFrame::Bulk(Some(BulkData::from_vec(
            self.command_rules.describe().into_bytes(),
        )));
        let keys = RespFrame::Bulk(Some(BulkData::from_vec(
            describe_patterns(&self.key_patterns, '~').into_bytes(),
        )));
        let channels = RespFrame::Bulk(Some(BulkData::from_vec(
            describe_patterns(&self.channel_patterns, '&').into_bytes(),
        )));

        RespFrame::Array(Some(vec![
            bulk_string("flags"),
            flags,
            bulk_string("passwords"),
            passwords,
            bulk_string("commands"),
            commands,
            bulk_string("keys"),
            keys,
            bulk_string("channels"),
            channels,
            bulk_string("username"),
            RespFrame::Bulk(Some(BulkData::from_vec(name.as_bytes().to_vec()))),
            bulk_string("enabled"),
            RespFrame::Integer(if self.enabled { 1 } else { 0 }),
        ]))
    }
}

impl CommandRules {
    #[inline(always)]
    pub(super) fn is_allow_all_unrestricted(&self) -> bool {
        self.allow_all
            && self.allowed_commands.is_empty()
            && self.denied_commands.is_empty()
            && self.allowed_categories.is_empty()
            && self.denied_categories.is_empty()
    }

    pub(super) fn allow_all() -> Self {
        let _trace = profiler::scope("server::auth::command_rules_allow_all");
        Self {
            allow_all: true,
            allowed_commands: BTreeSet::new(),
            denied_commands: BTreeSet::new(),
            allowed_categories: BTreeSet::new(),
            denied_categories: BTreeSet::new(),
        }
    }

    pub(super) fn deny_all() -> Self {
        let _trace = profiler::scope("server::auth::command_rules_deny_all");
        Self {
            allow_all: false,
            allowed_commands: BTreeSet::new(),
            denied_commands: BTreeSet::new(),
            allowed_categories: BTreeSet::new(),
            denied_categories: BTreeSet::new(),
        }
    }

    pub(super) fn reset(&mut self) {
        let _trace = profiler::scope("server::auth::command_rules_reset");
        *self = Self::deny_all();
    }

    pub(super) fn allow_command(&mut self, command: &str) {
        let _trace = profiler::scope("server::auth::allow_command");
        let command = command.to_ascii_uppercase();
        self.denied_commands.remove(&command);
        self.allowed_commands.insert(command);
    }

    pub(super) fn deny_command(&mut self, command: &str) {
        let _trace = profiler::scope("server::auth::deny_command");
        let command = command.to_ascii_uppercase();
        self.allowed_commands.remove(&command);
        self.denied_commands.insert(command);
    }

    pub(super) fn allow_category(&mut self, category: AclCategory) {
        let _trace = profiler::scope("server::auth::allow_category");
        self.denied_categories.remove(&category);
        self.allowed_categories.insert(category);
    }

    pub(super) fn deny_category(&mut self, category: AclCategory) {
        let _trace = profiler::scope("server::auth::deny_category");
        self.allowed_categories.remove(&category);
        self.denied_categories.insert(category);
    }

    pub(super) fn set_allow_all(&mut self, allow_all: bool) {
        let _trace = profiler::scope("server::auth::set_allow_all_commands");
        self.allow_all = allow_all;
        if allow_all {
            self.allowed_categories.remove(&AclCategory::All);
        }
    }

    pub(super) fn allows(&self, spec: &CommandSpec) -> bool {
        let _trace = profiler::scope("server::auth::command_rules_allows");
        if self.denied_commands.contains(spec.name) {
            return false;
        }
        if spec
            .categories
            .iter()
            .any(|category| self.denied_categories.contains(category))
        {
            return false;
        }
        if self.allow_all {
            return true;
        }
        self.allowed_commands.contains(spec.name)
            || spec
                .categories
                .iter()
                .any(|category| self.allowed_categories.contains(category))
    }

    pub(super) fn tokens(&self) -> Vec<String> {
        let _trace = profiler::scope("server::auth::command_rule_tokens");
        let mut out = Vec::new();
        out.push(if self.allow_all {
            "+@all".to_string()
        } else {
            "-@all".to_string()
        });
        out.extend(
            self.allowed_categories
                .iter()
                .map(|category| format!("+@{}", category.as_str())),
        );
        out.extend(
            self.denied_categories
                .iter()
                .filter(|category| **category != AclCategory::All)
                .map(|category| format!("-@{}", category.as_str())),
        );
        out.extend(
            self.allowed_commands
                .iter()
                .map(|command| format!("+{command}")),
        );
        out.extend(
            self.denied_commands
                .iter()
                .map(|command| format!("-{command}")),
        );
        out
    }

    pub(super) fn describe(&self) -> String {
        let _trace = profiler::scope("server::auth::describe_command_rules");
        self.tokens().join(" ")
    }
}

fn describe_patterns(patterns: &[Vec<u8>], prefix: char) -> String {
    let _trace = profiler::scope("server::auth::describe_patterns");
    if patterns.is_empty() {
        return String::new();
    }
    patterns
        .iter()
        .map(|pattern| format!("{prefix}{}", String::from_utf8_lossy(pattern)))
        .collect::<Vec<_>>()
        .join(" ")
}

fn pattern_tokens(
    patterns: &[Vec<u8>],
    all_token: &str,
    reset_token: &str,
    prefix: char,
) -> Vec<String> {
    let _trace = profiler::scope("server::auth::pattern_tokens");
    if patterns.is_empty() {
        return vec![reset_token.to_string()];
    }
    if patterns.len() == 1 && patterns[0] == b"*" {
        return vec![all_token.to_string()];
    }
    patterns
        .iter()
        .map(|pattern| format!("{prefix}{}", String::from_utf8_lossy(pattern)))
        .collect()
}

#[inline(always)]
fn is_all_pattern(patterns: &[Vec<u8>]) -> bool {
    patterns.len() == 1 && patterns[0] == b"*"
}

fn bulk_string(value: &str) -> protocol::types::RespFrame {
    protocol::types::RespFrame::Bulk(Some(protocol::types::BulkData::from_vec(
        value.as_bytes().to_vec(),
    )))
}
