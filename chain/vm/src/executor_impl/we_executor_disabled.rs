use multiversx_chain_vm_executor::{CompilationOptions, Executor, ExecutorError, Instance};
use std::fmt;

use crate::host::runtime::RuntimeWeakRef;

/// Conditional compilation variant of the WasmerProdExecutor, for when the "wasmer" feature is disabled and Wasmer cannot be used.
///
/// Always fails to produce instances.
pub struct ExperimentalExecutor;

impl fmt::Debug for ExperimentalExecutor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExperimentalExecutor (disabled)").finish()
    }
}

impl ExperimentalExecutor {
    pub fn new(_runtime_ref: RuntimeWeakRef) -> Self {
        ExperimentalExecutor
    }
}

impl Executor for ExperimentalExecutor {
    fn new_instance(
        &self,
        _wasm_bytes: &[u8],
        _compilation_options: &CompilationOptions,
    ) -> Result<Box<dyn Instance>, ExecutorError> {
        panic!("ExperimentalExecutor not available, need to activate features = [\"wasmer-experimental\"] in multiversx-sc-scenario or multiversx-chain-vm")
    }

    fn new_instance_from_cache(
        &self,
        _cache_bytes: &[u8],
        _compilation_options: &CompilationOptions,
    ) -> Result<Box<dyn Instance>, ExecutorError> {
        panic!("ExperimentalExecutor new_instance_from_cache not supported")
    }
}
