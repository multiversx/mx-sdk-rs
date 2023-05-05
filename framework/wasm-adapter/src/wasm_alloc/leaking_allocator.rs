use super::memory_grow::{memory_grow, PageCount, PAGE_SIZE};
use core::{
    alloc::{GlobalAlloc, Layout},
    cell::UnsafeCell,
};

/// A non-thread safe bump-pointer allocator.
/// Does not free or reuse memory.
/// Efficient for small allocations.
///
/// Largely inspired by lol_alloc:
/// https://github.com/Craig-Macomber/lol_alloc
pub struct LeakingAllocator {
    used: UnsafeCell<usize>, // bytes
    size: UnsafeCell<usize>, // bytes
}

/// Single-threaded context only.
unsafe impl Sync for LeakingAllocator {}

impl LeakingAllocator {
    pub const fn new() -> Self {
        LeakingAllocator {
            used: UnsafeCell::new(0),
            size: UnsafeCell::new(0),
        }
    }
}

unsafe impl GlobalAlloc for LeakingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size: &mut usize = &mut *self.size.get();
        let used: &mut usize = &mut *self.used.get();
        // This assumes PAGE_SIZE is always a multiple of the required alignment, which should be true for all practical use.
        // If this is not true, this could go past size.
        let alignment = layout.align();
        let offset = *used % alignment;
        if offset != 0 {
            *used += alignment - offset;
        }

        let requested_size = layout.size();
        let new_total = *used + requested_size;
        if new_total > *size {
            // Request enough new space for this allocation, even if we have some space left over from the last one incase they end up non-contiguous.
            // Round up to a number of pages
            let requested_pages = (requested_size + PAGE_SIZE - 1) / PAGE_SIZE;
            let previous_page_count = memory_grow(PageCount(requested_pages));

            let previous_size = previous_page_count.size_in_bytes();
            if previous_size != *size {
                // New memory is not contiguous with old: something else allocated in-between.
                // TODO: is handling this case necessary? Maybe make it optional behind a feature?
                // This assumes PAGE_SIZE is always a multiple of the required alignment, which should be true for all practical use.
                *used = previous_size;
            }
            *size = previous_size + requested_pages * PAGE_SIZE;
        }

        let start = *used;
        *used += requested_size;
        start as *mut u8
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}
