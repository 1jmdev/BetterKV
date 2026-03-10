#[derive(Clone, Copy)]
pub(super) struct NodeMeta {
    pub(super) next: u32,
}

impl NodeMeta {
    #[inline(always)]
    pub(super) fn new(next: u32) -> Self {
        Self { next }
    }
}
