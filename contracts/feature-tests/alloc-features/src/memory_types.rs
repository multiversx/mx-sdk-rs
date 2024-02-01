multiversx_sc::imports!();

/// Storage tests: direct load from storage to the heap.
#[multiversx_sc::module]
pub trait MemoryTypes {
    #[endpoint]
    #[label("fail-memory")]
    fn alloc_with_fail_memory(&self) -> i32 {
        let _x = String::from("H");
        1
    }
}
