use multiversx_chain_vm_executor::{
    CompilationOptions, Executor, ExecutorError, Instance, VMHooks,
};
use multiversx_chain_vm_executor_wasmer_experimental::{
    ExperimentalInstance, ExperimentalInstanceState, ExperimentalVMHooksBuilder,
};
use std::{fmt, sync::Arc};

use crate::host::{
    context::TxContextRef,
    runtime::RuntimeWeakRef,
    vm_hooks::{TxContextVMHooksHandler, VMHooksDispatcher},
};

use super::ExecutorFileNotFoundError;

/// Executor implementation that produces wasmer instances with correctly injected VM hooks from runtime.
pub struct ExperimentalExecutor {
    runtime_ref: RuntimeWeakRef,
}

impl fmt::Debug for ExperimentalExecutor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WasmerExperimentalExecutor").finish()
    }
}

impl ExperimentalExecutor {
    pub fn new(runtime_ref: RuntimeWeakRef) -> Self {
        ExperimentalExecutor { runtime_ref }
    }

    fn new_instance_from_bytes(
        &self,
        wasm_bytes: &[u8],
        compilation_options: &CompilationOptions,
    ) -> Box<dyn Instance> {
        let runtime = self.runtime_ref.upgrade();
        let tx_context_ref = runtime.get_executor_context();
        let vm_hooks_builder = ExperimentalTxContextVMHooksBuilder::new(tx_context_ref);

        let opcode_cost = runtime.vm_ref.gas_schedule.wasm_opcode_cost.clone();

        Box::new(
            ExperimentalInstance::try_new_instance(
                Box::new(vm_hooks_builder),
                Arc::new(opcode_cost),
                wasm_bytes,
                compilation_options,
            )
            .expect("instance init failed"),
        )
    }
}

impl Executor for ExperimentalExecutor {
    fn new_instance(
        &self,
        wasm_bytes: &[u8],
        compilation_options: &CompilationOptions,
    ) -> Result<Box<dyn Instance>, ExecutorError> {
        if wasm_bytes.starts_with("MISSING:".as_bytes()) {
            return Err(Box::new(ExecutorFileNotFoundError(
                String::from_utf8_lossy(wasm_bytes).to_string(),
            )));
        }

        Ok(self.new_instance_from_bytes(wasm_bytes, compilation_options))
    }

    fn new_instance_from_cache(
        &self,
        _cache_bytes: &[u8],
        _compilation_options: &CompilationOptions,
    ) -> Result<Box<dyn Instance>, ExecutorError> {
        panic!("WasmerProdExecutor new_instance_from_cache not supported")
    }
}

/// Combines the VM's `TxContextRef` with the `ExperimentalInstanceState` from the executor,
/// to create the `VMHooks.
pub struct ExperimentalTxContextVMHooksBuilder {
    tx_context_ref: TxContextRef,
}

impl ExperimentalTxContextVMHooksBuilder {
    pub fn new(tx_context_ref: TxContextRef) -> Self {
        ExperimentalTxContextVMHooksBuilder { tx_context_ref }
    }
}

impl ExperimentalVMHooksBuilder for ExperimentalTxContextVMHooksBuilder {
    fn create_vm_hooks<'b, 'h>(
        &'b self,
        instance_state_ref: &'h mut ExperimentalInstanceState,
    ) -> Box<dyn VMHooks + 'h> {
        let handler = TxContextVMHooksHandler::new(self.tx_context_ref.clone(), instance_state_ref);
        Box::new(VMHooksDispatcher::new(handler))
    }
}
