/// A number of WebAssembly memory pages.
#[derive(Eq, PartialEq)]
pub struct PageCount(pub usize);

impl PageCount {
    pub fn size_in_bytes(self) -> usize {
        self.0 * PAGE_SIZE
    }
}

/// The WebAssembly page size, in bytes.
pub const PAGE_SIZE: usize = 65536;

#[cfg(target_arch = "wasm32")]
pub fn memory_grow(delta: PageCount) -> PageCount {
    // This should use `core::arch::wasm` instead of `core::arch::wasm32`,
    // but `core::arch::wasm` depends on `#![feature(simd_wasm64)]` on current nightly.
    // See https://github.com/Craig-Macomber/lol_alloc/issues/1
    PageCount(core::arch::wasm32::memory_grow(0, delta.0))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn memory_grow(_delta: PageCount) -> PageCount {
    super::mem_alloc_error()
}

/// Not currently used, leaving it for reference in case someone needs to access the data.
#[allow(unused)]
#[cfg(target_arch = "wasm32")]
pub fn memory_size() -> PageCount {
    PageCount(core::arch::wasm32::memory_size(0))
}

/// Not currently used, leaving it for reference in case someone needs to access the data.
#[allow(unused)]
#[cfg(not(target_arch = "wasm32"))]
pub fn memory_size() -> PageCount {
    super::mem_alloc_error()
}
