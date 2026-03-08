use super::state::DEFAULT_USER;

#[derive(Clone, Debug)]
pub struct SessionAuth {
    pub(super) user: Option<String>,
    pub(super) authorized: bool,
    pub(super) acl_check_required: bool,
    pub(super) acl_epoch: u64,
}

impl SessionAuth {
    pub(super) fn auto_authorized() -> Self {
        Self {
            user: Some(DEFAULT_USER.to_string()),
            authorized: true,
            acl_check_required: false,
            acl_epoch: 0,
        }
    }

    pub(super) fn unauthenticated() -> Self {
        Self {
            user: None,
            authorized: false,
            acl_check_required: false,
            acl_epoch: 0,
        }
    }

    pub fn user(&self) -> Option<&str> {
        let _trace = profiler::scope("server::auth::session_user");
        self.user.as_deref()
    }

    pub fn set_user(&mut self, user: String) {
        let _trace = profiler::scope("server::auth::session_set_user");
        self.user = Some(user);
        self.authorized = true;
        self.acl_check_required = false;
        self.acl_epoch = 0;
    }

    pub(crate) fn set_acl_state(&mut self, acl_check_required: bool, acl_epoch: u64) {
        let _trace = profiler::scope("server::auth::session_set_acl_state");
        self.acl_check_required = acl_check_required;
        self.acl_epoch = acl_epoch;
    }

    pub(crate) fn revoke(&mut self) {
        let _trace = profiler::scope("server::auth::session_revoke");
        self.user = None;
        self.authorized = false;
        self.acl_check_required = false;
        self.acl_epoch = 0;
    }

    pub fn is_authorized(&self) -> bool {
        let _trace = profiler::scope("server::auth::session_is_authorized");
        self.authorized
    }

    #[inline(always)]
    pub fn authorized(&self) -> bool {
        self.authorized
    }

    #[inline(always)]
    pub fn acl_check_required(&self) -> bool {
        self.acl_check_required
    }

    #[inline(always)]
    pub fn acl_epoch(&self) -> u64 {
        self.acl_epoch
    }
}
