mod contract_allocator;
mod stack_size;

pub use contract_allocator::{parse_allocator, ContractAllocator};
pub use stack_size::*;

use crate::{ei::EIVersion, tools};

use super::ContractVariantProfileSerde;

/// Collection of flags, specified in the multicontract config.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ContractVariantSettings {
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

    /// Allows disabling default features in the contract crate, from wasm.
    pub default_features: Option<bool>,

    /// Forcibly remove the original contract legacy callback.
    pub kill_legacy_callback: bool,

    pub profile: ContractVariantProfile,

    pub rustc_target: String,
}

impl Default for ContractVariantSettings {
    fn default() -> Self {
        ContractVariantSettings {
            external_view: Default::default(),
            panic_message: Default::default(),
            check_ei: Some(EIVersion::default()),
            allocator: Default::default(),
            stack_size: DEFAULT_STACK_SIZE,
            features: Default::default(),
            default_features: None,
            kill_legacy_callback: false,
            profile: Default::default(),
            rustc_target: tools::build_target::default_target().to_owned(),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ContractVariantProfile {
    pub codegen_units: u8,
    pub opt_level: String,
    pub lto: bool,
    pub debug: bool,
    pub panic: String,
    pub overflow_checks: bool,
}

impl Default for ContractVariantProfile {
    fn default() -> ContractVariantProfile {
        ContractVariantProfile {
            codegen_units: 1u8,
            opt_level: "z".to_owned(),
            lto: true,
            debug: false,
            panic: "abort".to_owned(),
            overflow_checks: false,
        }
    }
}

impl ContractVariantProfile {
    pub fn from_serde(opt_serde_profile: &Option<ContractVariantProfileSerde>) -> Self {
        let mut result = Self::default();
        if let Some(serde_profile) = opt_serde_profile {
            if let Some(codegen_units) = serde_profile.codegen_units {
                result.codegen_units = codegen_units;
            }
            if let Some(opt_level) = &serde_profile.opt_level {
                result.opt_level.clone_from(opt_level);
            }
            if let Some(lto) = serde_profile.lto {
                result.lto = lto;
            }
            if let Some(debug) = serde_profile.debug {
                result.debug = debug;
            }
            if let Some(panic) = &serde_profile.panic {
                result.panic.clone_from(panic);
            }
            if let Some(overflow_checks) = serde_profile.overflow_checks {
                result.overflow_checks = overflow_checks;
            }
        }
        result
    }
}
