#[derive(Clone, Copy)]
pub(super) struct NodeMeta {
    pub(super) hash: u32,
    pub(super) next: u32,
}

impl NodeMeta {
    #[inline(always)]
    pub(super) fn new(hash: u32, next: u32) -> Self {
        Self { hash, next }
    }
}
