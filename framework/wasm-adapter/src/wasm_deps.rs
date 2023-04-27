mod fail_allocator;
mod leaking_allocator;
mod memory_grow;
mod panic_fmt;
mod static_allocator;

pub use fail_allocator::FailAllocator;
pub use leaking_allocator::LeakingAllocator;
pub use panic_fmt::{panic_fmt, panic_fmt_with_message};
pub use static_allocator::{StaticAllocator, StaticAllocator64K};

/// Used in wasm crate macros.
pub use core::panic::PanicInfo;
