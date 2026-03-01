use super::constants::NIL;

pub(super) struct Table {
    pub(super) heads: Vec<u32>,
}

impl Table {
    pub(super) fn with_buckets(count: usize) -> Self {
        Self {
            heads: vec![NIL; count],
        }
    }

    pub(super) fn len(&self) -> usize {
        self.heads.len()
    }
}
