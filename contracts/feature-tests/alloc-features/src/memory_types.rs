multiversx_sc::imports!();

/// Storage tests: direct load from storage to the heap.
#[multiversx_sc::module]
pub trait MemoryTypes {
    #[endpoint]
    #[label("alloc-mem-fail")]
    fn alloc_with_fail_memory(&self) -> i32 {
        let _x = String::from("H");
        1
    }

    #[endpoint]
    #[label("alloc-mem-leaking")]
    fn alloc_with_leaking_memory(&self) -> i32 {
        let _ = Box::new(42);
        1
    }
}
