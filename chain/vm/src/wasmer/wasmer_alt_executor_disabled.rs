use multiversx_chain_vm_executor::{
    CompilationOptions, Executor, ExecutorError, Instance, OpcodeCost,
};
use std::fmt;

use crate::host::runtime::RuntimeWeakRef;

/// Conditional compilation variant of the WasmerAltExecutor, for when the "wasmer" feature is disabled and Wasmer cannot be used.
///
/// Always fails to produce instances.
pub struct WasmerAltExecutor;

impl fmt::Debug for WasmerAltExecutor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WasmerAltExecutor (disabled)").finish()
    }
}

impl WasmerAltExecutor {
    pub fn new(_runtime_ref: RuntimeWeakRef) -> Self {
        WasmerAltExecutor
    }
}

impl Executor for WasmerAltExecutor {
    fn set_vm_hooks_ptr(
        &mut self,
        _vm_hooks_ptr: *mut std::ffi::c_void,
    ) -> Result<(), ExecutorError> {
        todo!()
    }

    fn set_opcode_cost(&mut self, _opcode_cost: &OpcodeCost) -> Result<(), ExecutorError> {
        Ok(())
    }

    fn new_instance(
        &self,
        _wasm_bytes: &[u8],
        _compilation_options: &CompilationOptions,
    ) -> Result<Box<dyn Instance>, ExecutorError> {
        panic!("Wasmer executor not available, need to activate features = [\"wasmer\"] in multiversx-sc-scenario or multiversx-chain-vm ")
    }

    fn new_instance_from_cache(
        &self,
        _cache_bytes: &[u8],
        _compilation_options: &CompilationOptions,
    ) -> Result<Box<dyn Instance>, ExecutorError> {
        panic!("WasmerAltExecutor new_instance_from_cache not supported")
    }
}
