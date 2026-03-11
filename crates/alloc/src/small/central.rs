use std::alloc::Layout;

use crate::lock::SpinLock;
use crate::small::class::{CLASS_COUNT, SizeClass};
use crate::small::freelist::FreeList;
use crate::system;

static CENTRAL_POOLS: [CentralPool; CLASS_COUNT] = [const { CentralPool::new() }; CLASS_COUNT];

pub fn refill(class: SizeClass, local: &mut FreeList, target_count: usize) {
    CENTRAL_POOLS[class.index].refill(class, local, target_count);
}

pub fn drain(class: SizeClass, local: &mut FreeList, retain: usize) {
    CENTRAL_POOLS[class.index].drain(local, retain);
}

struct CentralPool {
    state: SpinLock<CentralState>,
}

struct CentralState {
    free_list: FreeList,
}

impl CentralPool {
    const fn new() -> Self {
        Self {
            state: SpinLock::new(CentralState {
                free_list: FreeList::new(),
            }),
        }
    }

    fn refill(&self, class: SizeClass, local: &mut FreeList, target_count: usize) {
        let mut state = self.state.lock();
        let needed = target_count.saturating_sub(local.len());
        if needed == 0 {
            return;
        }

        while state.free_list.len() < needed {
            let mut run = allocate_run(class);
            state.free_list.append(&mut run);
        }

        let mut batch = state.free_list.split_off(needed);
        local.append(&mut batch);
    }

    fn drain(&self, local: &mut FreeList, retain: usize) {
        let mut state = self.state.lock();

        if local.len() <= retain {
            return;
        }

        let mut overflow = local.split_off(local.len() - retain);
        state.free_list.append(&mut overflow);
    }
}

fn allocate_run(class: SizeClass) -> FreeList {
    let slot_size = class.slot_size;
    let slot_count = class.batch_count();
    let total_size = match slot_size.checked_mul(slot_count) {
        Some(total_size) => total_size,
        None => std::alloc::handle_alloc_error(layout_for_run(slot_size, 1)),
    };
    let layout = layout_for_run(total_size, 16);
    let base_ptr = unsafe { system::alloc(layout) };
    let mut run = FreeList::new();

    for index in (0..slot_count).rev() {
        let slot_ptr = unsafe { base_ptr.add(index * slot_size) };
        unsafe {
            run.push(slot_ptr);
        }
    }

    run
}

fn layout_for_run(size: usize, align: usize) -> Layout {
    match Layout::from_size_align(size, align) {
        Ok(layout) => layout,
        Err(_) => panic!("invalid central run layout: size={size} align={align}"),
    }
}
