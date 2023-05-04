use core::{
    alloc::{GlobalAlloc, Layout},
    cell::UnsafeCell,
};

/// The pre-allocated buffer size.
///
/// TODO: make configurable, experiment with it.
pub const SIZE_64K: usize = 64 * 1024;

pub type StaticAllocator64K = StaticAllocator<SIZE_64K>;

/// Uses a statically pre-allocated section to allocate all memory.
///
/// Does not free up memory. Cannot grow beyond what was statically pre-allocated.
///
/// Never calls `memory.grow`.
///
/// Largely inspired by this blog post:
/// https://surma.dev/things/rust-to-webassembly/
#[repr(C, align(32))]
pub struct StaticAllocator<const SIZE: usize> {
    arena: UnsafeCell<[u8; SIZE]>,
    head: UnsafeCell<usize>,
}

impl<const SIZE: usize> StaticAllocator<SIZE> {
    pub const fn new() -> Self {
        StaticAllocator {
            arena: UnsafeCell::new([0; SIZE]),
            head: UnsafeCell::new(0),
        }
    }
}

unsafe impl<const SIZE: usize> Sync for StaticAllocator<SIZE> {}

unsafe impl<const SIZE: usize> GlobalAlloc for StaticAllocator<SIZE> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();

        // Find the next address that has the right alignment.
        let idx = (*self.head.get()).next_multiple_of(align);
        // Bump the head to the next free byte
        *self.head.get() = idx + size;
        let arena: &mut [u8; SIZE] = &mut (*self.arena.get());
        // If we ran out of arena space, kill execution with mem_alloc_error.
        match arena.get_mut(idx) {
            Some(item) => item as *mut u8,
            _ => super::mem_alloc_error(),
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}
