use multiversx_chain_vm_executor::{
    CompilationOptions, Executor, ExecutorError, Instance, OpcodeCost,
};
use multiversx_chain_vm_executor_wasmer::{WasmerExecutorData, WasmerInstance};
use std::{cell::RefCell, fmt, rc::Rc};

use crate::{
    executor_impl::ExecutorFileNotFoundError,
    host::{
        runtime::RuntimeWeakRef,
        vm_hooks::{TxContextVMHooksHandler, VMHooksDispatcher},
    },
};

use super::{WasmerProdInstance, WasmerProdInstanceState};

/// Executor implementation that produces wasmer instances with correctly injected VM hooks from runtime.
pub struct WasmerProdExecutor {
    runtime_ref: RuntimeWeakRef,
}

impl fmt::Debug for WasmerProdExecutor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WasmerProdExecutor").finish()
    }
}

impl WasmerProdExecutor {
    pub fn new(runtime_ref: RuntimeWeakRef) -> Self {
        WasmerProdExecutor { runtime_ref }
    }

    fn new_instance_from_bytes(
        &self,
        wasm_bytes: &[u8],
        compilation_options: &CompilationOptions,
    ) -> Box<dyn Instance> {
        let runtime = self.runtime_ref.upgrade();
        let tx_context_ref = runtime.get_executor_context();

        let inner_instance_ref = Rc::new_cyclic(|weak| {
            let vh_handler = TxContextVMHooksHandler::new(
                tx_context_ref,
                Rc::new(WasmerProdInstanceState::new(weak.clone())),
            );
            let vm_hooks = VMHooksDispatcher::new(Box::new(vh_handler));
            let executor_data = WasmerExecutorData::new_with_gas(
                Box::new(vm_hooks),
                runtime.vm_ref.gas_schedule.wasm_opcode_cost.clone(),
            );
            let executor_data_ref = Rc::new(RefCell::new(executor_data));

            WasmerInstance::try_new_instance(
                executor_data_ref.clone(),
                wasm_bytes,
                compilation_options,
            )
            .expect("instance init failed")
        });

        let wasmer_instance_ref = WasmerProdInstance::new(inner_instance_ref);

        Box::new(wasmer_instance_ref)
    }
}

impl Executor for WasmerProdExecutor {
    fn set_opcode_cost(&mut self, _opcode_cost: &OpcodeCost) -> Result<(), ExecutorError> {
        Ok(())
    }

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
