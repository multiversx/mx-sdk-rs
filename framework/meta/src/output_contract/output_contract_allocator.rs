#[derive(Default, Clone, PartialEq, Eq)]
pub enum ContractAllocator {
    /// No allocation is allowed. Any attempt causes `signalError` to be thrown.
    #[default]
    AllocationForbidden,

    /// An allocator that never deallocates. It calls memory grow to reserve memory chuncks.
    LeakingAllocator,

    /// An allocator that uses a statically pre-allocated chunk of memory, of 64kb.
    ///
    /// It also never deallocates.
    StaticAllocator64K,

    /// Uses wee-alloc, but wee-alloc needs to be explicitly imported by the contract wasm crate.
    ///
    /// Mostly present for historical reasons, or if in some extreme case the contract needs deallocation.
    WeeAlloc,
}

impl ContractAllocator {
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "fail" => Some(ContractAllocator::AllocationForbidden),
            "leaking" => Some(ContractAllocator::LeakingAllocator),
            "static64k" => Some(ContractAllocator::StaticAllocator64K),
            "wee_alloc" => Some(ContractAllocator::WeeAlloc),
            _ => None,
        }
    }

    pub fn parse_or_panic(s: &str) -> Self {
        Self::parse(s).unwrap_or_else(|| {
            panic!("Unknown allocator option '{s}'. Valid options are: 'fail', 'leaking', 'static64k', 'wee_alloc'.")
        })
    }

    pub fn to_allocator_macro_selector(&self) -> &'static str {
        match self {
            ContractAllocator::AllocationForbidden => "",
            ContractAllocator::LeakingAllocator => "leaking",
            ContractAllocator::StaticAllocator64K => "static64k",
            ContractAllocator::WeeAlloc => "wee_alloc",
        }
    }
}
