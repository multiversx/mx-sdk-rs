#[derive(Default, Clone, PartialEq, Eq)]
pub enum ContractAllocator {
    /// No allocation is allowed. Any attempt causes `signalError` to be thrown.
    #[default]
    AllocationForbidden,

    /// An allocator that never deallocates. It calls memory grow to reserve memory chuncks.
    LeakingAllocator,

    /// An allocator that 
    StaticAllocator64K,

    /// Backwards compatibility, for now.
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
