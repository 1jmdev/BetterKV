use super::constants::{INITIAL_BUCKETS, NIL};

pub(super) struct Table {
    pub(super) heads: Vec<u32>,
}

impl Table {
    #[inline(always)]
    pub(super) fn with_buckets(count: usize) -> Self {
        let _trace = profiler::scope("rehash::table::with_buckets");
        let count = count.max(INITIAL_BUCKETS);
        Self {
            heads: vec![NIL; count],
        }
    }

    #[inline(always)]
    pub(super) fn len(&self) -> usize {
        let _trace = profiler::scope("rehash::table::len");
        self.heads.len()
    }
}
