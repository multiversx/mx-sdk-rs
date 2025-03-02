use std::{cell::RefCell, fmt, rc::Rc};

use multiversx_chain_vm::{
    tx_execution::RuntimeWeakRef,
    vm_hooks::{TxContextVMHooksHandler, VMHooksDispatcher},
};
use multiversx_chain_vm_executor::{
    CompilationOptions, Executor, ExecutorError, Instance, OpcodeCost,
};
use multiversx_chain_vm_executor_wasmer::{WasmerExecutorData, WasmerInstance};

use super::WrappedInstance;

pub struct WasmerAltExecutor {
    runtime_ref: RuntimeWeakRef,
}

impl fmt::Debug for WasmerAltExecutor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WasmerAltExecutor").finish()
    }
}

impl WasmerAltExecutor {
    pub fn new(runtime_ref: RuntimeWeakRef) -> Self {
        WasmerAltExecutor { runtime_ref }
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
        wasm_bytes: &[u8],
        compilation_options: &CompilationOptions,
    ) -> Result<Box<dyn Instance>, ExecutorError> {
        let tx_context_ref = self.runtime_ref.upgrade().current_context();

        let inner_instance_ref = Rc::new_cyclic(|weak| {
            let vh_handler = TxContextVMHooksHandler::new(tx_context_ref, weak.clone());
            let vm_hooks = VMHooksDispatcher::new(Box::new(vh_handler));
            let executor_data_ref =
                Rc::new(RefCell::new(WasmerExecutorData::new(Box::new(vm_hooks))));

            Box::new(
                WasmerInstance::try_new_instance(
                    executor_data_ref.clone(),
                    wasm_bytes,
                    compilation_options,
                )
                .expect("instance init failed"),
            )
        });

        let wasmer_instance_ref = WrappedInstance::new(inner_instance_ref);

        Ok(Box::new(wasmer_instance_ref))
    }

    fn new_instance_from_cache(
        &self,
        _cache_bytes: &[u8],
        _compilation_options: &CompilationOptions,
    ) -> Result<Box<dyn Instance>, ExecutorError> {
        panic!("WasmerAltExecutor new_instance_from_cache not supported")
    }
}
