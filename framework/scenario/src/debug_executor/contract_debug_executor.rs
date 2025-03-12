use super::*;

use multiversx_chain_vm::tx_execution::RuntimeWeakRef;
use multiversx_chain_vm_executor::{
    CompilationOptions, Executor, ExecutorError, Instance, OpcodeCost,
};
use std::fmt;

pub struct ContractDebugExecutor {
    runtime_ref: RuntimeWeakRef,
    contract_map_ref: ContractMapRef,
}

impl fmt::Debug for ContractDebugExecutor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DebugSCExecutor").finish()
    }
}

impl ContractDebugExecutor {
    pub fn new(runtime_ref: RuntimeWeakRef, contract_map_ref: ContractMapRef) -> Self {
        ContractDebugExecutor {
            runtime_ref,
            contract_map_ref,
        }
    }
}

impl Executor for ContractDebugExecutor {
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
        wasm_bytes: &[u8],
        _compilation_options: &CompilationOptions,
    ) -> Result<Box<dyn Instance>, ExecutorError> {
        let contract_container = self.contract_map_ref.lock().get_contract(wasm_bytes);
        let tx_context_ref = self.runtime_ref.upgrade().get_executor_context();
        let instance = ContractDebugInstance::new(tx_context_ref, contract_container);
        Ok(Box::new(instance))
    }

    fn new_instance_from_cache(
        &self,
        _cache_bytes: &[u8],
        _compilation_options: &CompilationOptions,
    ) -> Result<Box<dyn Instance>, ExecutorError> {
        panic!("DebugSCExecutor new_instance_from_cache not supported")
    }
}
