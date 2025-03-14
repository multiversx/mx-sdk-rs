#![allow(unused)]

use multiversx_chain_vm_executor::{
    CompilationOptions, Executor, ExecutorError, Instance, OpcodeCost,
};

#[derive(Debug)]
pub struct ExecutorDeclinedError;

impl std::fmt::Display for ExecutorDeclinedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("executor cannot process this input")
    }
}

impl std::error::Error for ExecutorDeclinedError {}

pub struct CompositeExecutor {
    pub executors: Vec<Box<dyn Executor + Send + Sync>>,
}

impl Executor for CompositeExecutor {
    fn set_vm_hooks_ptr(
        &mut self,
        vm_hooks_ptr: *mut std::ffi::c_void,
    ) -> Result<(), ExecutorError> {
        todo!()
    }

    fn set_opcode_cost(&mut self, opcode_cost: &OpcodeCost) -> Result<(), ExecutorError> {
        todo!()
    }

    fn new_instance(
        &self,
        wasm_bytes: &[u8],
        compilation_options: &CompilationOptions,
    ) -> Result<Box<dyn Instance>, ExecutorError> {
        todo!()
    }

    fn new_instance_from_cache(
        &self,
        _cache_bytes: &[u8],
        _compilation_options: &CompilationOptions,
    ) -> Result<Box<dyn Instance>, ExecutorError> {
        panic!("new_instance_from_cache not supported")
    }
}
