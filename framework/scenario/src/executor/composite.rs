use multiversx_chain_vm_executor::{
    CompilationOptions, Executor, ExecutorError, Instance, MissingWasmError,
};
use simple_error::SimpleError;
use std::fmt;

use crate::executor::debug::ContractDebugExecutorNotRegisteredError;

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
    pub fn new(executors: Vec<Box<dyn Executor + Send + Sync>>) -> Self {
        CompositeExecutor { executors }
    }
}

impl Executor for CompositeExecutor {
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
    if err.is::<MissingWasmError>() {
        return true;
    }
    false
}
