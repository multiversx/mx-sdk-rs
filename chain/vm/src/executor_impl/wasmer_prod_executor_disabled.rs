use multiversx_chain_vm_executor::{
    CompilationOptions, Executor, ExecutorError, Instance, OpcodeCost,
};
use std::fmt;

use crate::host::runtime::RuntimeWeakRef;

/// Conditional compilation variant of the WasmerProdExecutor, for when the "wasmer" feature is disabled and Wasmer cannot be used.
///
/// Always fails to produce instances.
pub struct WasmerProdExecutor;

impl fmt::Debug for WasmerProdExecutor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WasmerProdExecutor (disabled)").finish()
    }
}

impl WasmerProdExecutor {
    pub fn new(_runtime_ref: RuntimeWeakRef) -> Self {
        WasmerProdExecutor
    }
}

impl Executor for WasmerProdExecutor {
    fn set_opcode_cost(&mut self, _opcode_cost: &OpcodeCost) -> Result<(), ExecutorError> {
        panic!("Wasmer executor not available, need to activate features = [\"wasmer-prod\"] in multiversx-sc-scenario or multiversx-chain-vm")
    }

    fn new_instance(
        &self,
        _wasm_bytes: &[u8],
        _compilation_options: &CompilationOptions,
    ) -> Result<Box<dyn Instance>, ExecutorError> {
        panic!("Wasmer executor not available, need to activate features = [\"wasmer-prod\"] in multiversx-sc-scenario or multiversx-chain-vm")
    }

    fn new_instance_from_cache(
        &self,
        _cache_bytes: &[u8],
        _compilation_options: &CompilationOptions,
    ) -> Result<Box<dyn Instance>, ExecutorError> {
        panic!("WasmerProdExecutor new_instance_from_cache not supported")
    }
}
