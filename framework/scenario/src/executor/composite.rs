use multiversx_chain_vm::{
    host::runtime::RuntimeWeakRef,
    wasmer::{WasmerAltExecutor, WasmerAltExecutorFileNotFoundError},
};
use multiversx_chain_vm_executor::{
    CompilationOptions, Executor, ExecutorError, Instance, OpcodeCost,
};
use simple_error::SimpleError;
use std::fmt;

use crate::executor::debug::ContractDebugExecutorNotRegisteredError;

use super::debug::{ContractDebugExecutor, ContractMapRef};

/// An executor that delegates instance creation to several other executors.
///
/// The executors are tried one after the other. If they return known whitelisted errors, we fall back to the next executor on the list.
pub struct CompositeExecutor {
    executors: Vec<Box<dyn Executor + Send + Sync>>,
}

impl fmt::Debug for CompositeExecutor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CompositeExecutor").finish()
    }
}

impl CompositeExecutor {
    pub fn new_debugger_then_wasmer(
        runtime_ref: RuntimeWeakRef,
        contract_map_ref: ContractMapRef,
    ) -> Self {
        CompositeExecutor {
            executors: vec![
                Box::new(ContractDebugExecutor::new(
                    runtime_ref.clone(),
                    contract_map_ref.clone(),
                )),
                Box::new(WasmerAltExecutor::new(runtime_ref)),
            ],
        }
    }
}

impl Executor for CompositeExecutor {
    fn set_vm_hooks_ptr(
        &mut self,
        _vm_hooks_ptr: *mut std::ffi::c_void,
    ) -> Result<(), ExecutorError> {
        panic!("CompositeExecutor set_vm_hooks_ptr not yet supported")
    }

    fn set_opcode_cost(&mut self, _opcode_cost: &OpcodeCost) -> Result<(), ExecutorError> {
        Ok(())
    }

    fn new_instance(
        &self,
        wasm_bytes: &[u8],
        compilation_options: &CompilationOptions,
    ) -> Result<Box<dyn Instance>, ExecutorError> {
        for executor in &self.executors {
            match executor.new_instance(wasm_bytes, compilation_options) {
                Ok(instance) => {
                    return Ok(instance);
                },
                Err(err) => {
                    if !is_recoverable_error(&err) {
                        return Err(err);
                    }
                },
            }
        }

        Err(Box::new(SimpleError::new("contract not found")))
    }

    fn new_instance_from_cache(
        &self,
        _cache_bytes: &[u8],
        _compilation_options: &CompilationOptions,
    ) -> Result<Box<dyn Instance>, ExecutorError> {
        panic!("DebugSCExecutor new_instance_from_cache not supported")
    }
}

fn is_recoverable_error(err: &ExecutorError) -> bool {
    if err.is::<ContractDebugExecutorNotRegisteredError>() {
        return true;
    }
    if err.is::<WasmerAltExecutorFileNotFoundError>() {
        return true;
    }
    false
}
