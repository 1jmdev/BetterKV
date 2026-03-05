#[derive(Clone, Copy)]
pub(super) struct NodeMeta {
    pub(super) hash: u64,
    pub(super) key_len: u32,
    pub(super) next: u32,
}
