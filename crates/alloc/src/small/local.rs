use std::cell::UnsafeCell;

use crate::small::central;
use crate::small::class::{CLASS_COUNT, LOCAL_FLUSH_COUNT, LOCAL_REFILL_COUNT, SizeClass};
use crate::small::freelist::FreeList;

thread_local! {
    static LOCAL_CACHE: LocalCache = const { LocalCache::new() };
}

#[derive(Clone, Copy)]
struct LocalListState {
    head: usize,
    tail: usize,
    len: usize,
}

impl LocalListState {
    const fn new() -> Self {
        Self {
            head: 0,
            tail: 0,
            len: 0,
        }
    }
}

pub fn alloc(class: SizeClass) -> *mut u8 {
    LOCAL_CACHE.with(|cache| {
        let mut local_list = cache.list(class.index);
        let slot_ptr = local_list.pop();
        if !slot_ptr.is_null() {
            cache.store_list(class.index, local_list);
            return slot_ptr;
        }

        central::refill(class, &mut local_list, usize::from(LOCAL_REFILL_COUNT));
        let slot_ptr = local_list.pop();
        cache.store_list(class.index, local_list);
        slot_ptr
    })
}

pub fn dealloc(class: SizeClass, slot_ptr: *mut u8) {
    LOCAL_CACHE.with(|cache| {
        let mut local_list = cache.list(class.index);
        unsafe {
            local_list.push(slot_ptr);
        }

        if local_list.len() >= usize::from(LOCAL_FLUSH_COUNT) {
            central::drain(class, &mut local_list, usize::from(LOCAL_REFILL_COUNT));
        }

        cache.store_list(class.index, local_list);
    });
}

struct LocalCache {
    lists: UnsafeCell<[LocalListState; CLASS_COUNT]>,
}

impl LocalCache {
    const fn new() -> Self {
        Self {
            lists: UnsafeCell::new([const { LocalListState::new() }; CLASS_COUNT]),
        }
    }

    #[inline(always)]
    fn list(&self, index: usize) -> FreeList {
        unsafe {
            let state = &(*self.lists.get())[index];
            FreeList::from_raw(state.head, state.tail, state.len)
        }
    }

    #[inline(always)]
    fn store_list(&self, index: usize, list: FreeList) {
        unsafe {
            let state = &mut (*self.lists.get())[index];
            state.head = list.head();
            state.tail = list.tail();
            state.len = list.len();
        }
    }
}
