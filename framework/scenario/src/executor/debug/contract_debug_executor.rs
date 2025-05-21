use super::*;

use multiversx_chain_vm::host::runtime::RuntimeWeakRef;
use multiversx_chain_vm_executor::{CompilationOptions, Executor, ExecutorError, Instance};
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

    pub fn new_contract_container_instance(
        &self,
        contract_container: ContractContainerRef,
    ) -> Box<dyn Instance> {
        let tx_context_ref = self.runtime_ref.upgrade().get_executor_context();
        let instance = ContractDebugInstance::new(tx_context_ref, contract_container);
        Box::new(instance)
    }
}

impl Executor for ContractDebugExecutor {
    fn new_instance(
        &self,
        wasm_bytes: &[u8],
        _compilation_options: &CompilationOptions,
    ) -> Result<Box<dyn Instance>, ExecutorError> {
        if let Some(contract_container) = self.contract_map_ref.lock().get_contract(wasm_bytes) {
            Ok(self.new_contract_container_instance(contract_container))
        } else {
            Err(Box::new(ContractDebugExecutorNotRegisteredError::new(
                wasm_bytes,
            )))
        }
    }

    fn new_instance_from_cache(
        &self,
        _cache_bytes: &[u8],
        _compilation_options: &CompilationOptions,
    ) -> Result<Box<dyn Instance>, ExecutorError> {
        panic!("DebugSCExecutor new_instance_from_cache not supported")
    }
}
