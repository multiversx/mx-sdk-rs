mod fail_allocator;
mod leaking_allocator;
mod memory_grow;
mod static_allocator;

pub use fail_allocator::FailAllocator;
pub use leaking_allocator::LeakingAllocator;
pub use static_allocator::{StaticAllocator, StaticAllocator64K};
