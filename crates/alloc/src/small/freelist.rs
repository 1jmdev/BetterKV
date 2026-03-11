#[repr(C)]
struct FreeNode {
    next: usize,
}

#[derive(Clone, Copy)]
pub struct FreeList {
    head: usize,
    tail: usize,
    len: usize,
}

impl FreeList {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            head: 0,
            tail: 0,
            len: 0,
        }
    }

    #[inline(always)]
    pub const fn from_raw(head: usize, tail: usize, len: usize) -> Self {
        Self { head, tail, len }
    }

    #[inline(always)]
    pub const fn head(&self) -> usize {
        self.head
    }

    #[inline(always)]
    pub const fn tail(&self) -> usize {
        self.tail
    }

    #[inline(always)]
    pub const fn len(&self) -> usize {
        self.len
    }

    #[inline(always)]
    pub fn pop(&mut self) -> *mut u8 {
        if self.head == 0 {
            return std::ptr::null_mut();
        }

        let node = self.head as *mut FreeNode;
        unsafe {
            self.head = (*node).next;
        }
        self.len -= 1;
        if self.head == 0 {
            self.tail = 0;
        }
        node.cast::<u8>()
    }

    #[inline(always)]
    pub unsafe fn push(&mut self, slot_ptr: *mut u8) {
        let node = slot_ptr.cast::<FreeNode>();
        unsafe {
            (*node).next = self.head;
        }
        self.head = node as usize;
        if self.tail == 0 {
            self.tail = self.head;
        }
        self.len += 1;
    }

    #[inline(always)]
    pub fn append(&mut self, other: &mut Self) {
        if other.head == 0 {
            return;
        }

        if self.head == 0 {
            *self = *other;
        } else {
            unsafe {
                (*(other.tail as *mut FreeNode)).next = self.head;
            }
            self.head = other.head;
            self.len += other.len;
        }

        *other = Self::new();
    }

    #[inline(always)]
    pub fn split_off(&mut self, count: usize) -> Self {
        if count == 0 || self.head == 0 {
            return Self::new();
        }
        if count >= self.len {
            let taken = *self;
            *self = Self::new();
            return taken;
        }

        let taken_head = self.head;
        let mut taken_tail = taken_head as *mut FreeNode;
        for _ in 1..count {
            unsafe {
                taken_tail = (*taken_tail).next as *mut FreeNode;
            }
        }

        let next_head = unsafe { (*taken_tail).next };
        unsafe {
            (*taken_tail).next = 0;
        }

        self.head = next_head;
        self.len -= count;

        Self {
            head: taken_head,
            tail: taken_tail as usize,
            len: count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FreeList;

    #[test]
    fn split_and_append_preserve_all_nodes() {
        let mut slots = [[0u8; 8]; 4];
        let mut list = FreeList::new();

        for slot in &mut slots {
            unsafe {
                list.push(slot.as_mut_ptr());
            }
        }

        let original_len = list.len();
        let mut taken = list.split_off(2);
        assert_eq!(taken.len(), 2);
        assert_eq!(list.len(), original_len - 2);

        list.append(&mut taken);
        assert_eq!(list.len(), original_len);
        assert_eq!(taken.len(), 0);
    }
}
