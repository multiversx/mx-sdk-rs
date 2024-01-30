multiversx_sc::imports!();

/// Storage tests: direct load from storage to the heap.
#[multiversx_sc::module]
pub trait MemoryTypes {
    #[endpoint]
    #[label("fail-memory")]
    fn alloc_with_fail_memory(&self) -> i32 {
        let _ = Box::new([0u8; 1024]);
        1
    }

    #[endpoint]
    #[label("leaking-memory")]
    fn alloc_with_leaking_memory(&self) -> i32 {
        let _ = Box::new(42);
        1
    }

}
