mod oc_allocator;
mod oc_parse;
mod oc_parse_stack_size;

pub use oc_allocator::ContractAllocator;
pub use oc_parse::*;
pub use oc_parse_stack_size::*;

use crate::ei::EIVersion;

/// Collection of flags, specified in the multicontract config.
#[derive(Clone, PartialEq, Eq)]
pub struct OutputContractSettings {
    /// External view contracts are just readers of data from another contract.
    pub external_view: bool,

    /// Panic messages add a lot of bloat to the final bytecode,
    /// so they should only be used for debugging purposes.
    pub panic_message: bool,

    /// Post-processing check of the VM hooks is based on this.
    pub check_ei: Option<EIVersion>,

    /// Allocator config, i.e which allocator to choose for the contract.
    pub allocator: ContractAllocator,

    pub stack_size: usize,

    /// Features that are activated on the contract crate, from wasm.
    pub features: Vec<String>,

    /// Forcibly remove the original contrct legacy callback.
    pub kill_legacy_callback: bool,
}

impl Default for OutputContractSettings {
    fn default() -> Self {
        OutputContractSettings {
            external_view: Default::default(),
            panic_message: Default::default(),
            check_ei: Some(EIVersion::default()),
            allocator: Default::default(),
            stack_size: DEFAULT_STACK_SIZE,
            features: Default::default(),
            kill_legacy_callback: false,
        }
    }
}
