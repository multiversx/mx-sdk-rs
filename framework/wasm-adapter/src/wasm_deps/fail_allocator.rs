use core::alloc::{GlobalAlloc, Layout};

fn signal_allocation_not_allowed() -> ! {
    crate::error_hook::signal_error(&b"memory allocation forbidden"[..])
}

/// Allocator that fails (with signal error) whenever an allocation is attempted.
pub struct FailAllocator;

unsafe impl GlobalAlloc for FailAllocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        signal_allocation_not_allowed()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        signal_allocation_not_allowed()
    }
}
