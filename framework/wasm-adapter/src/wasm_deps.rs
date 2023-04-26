mod fail_allocator;
mod panic_fmt;

pub use fail_allocator::FailAllocator;
pub use panic_fmt::{panic_fmt, panic_fmt_with_message};

/// Used in wasm crate macros.
pub use core::panic::PanicInfo;

/// TODO: remove.
pub use wee_alloc::WeeAlloc;
