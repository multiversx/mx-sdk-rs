use multiversx_chain_vm::tx_mock::{TxContextRef, TxFunctionName, TxPanic};
use multiversx_chain_vm_executor::{BreakpointValue, ExecutorError, Instance, MemLength, MemPtr};
use multiversx_sc::chain_core::types::ReturnCode;
use std::sync::Arc;

use crate::api::DebugApiBackend;

use super::{catch_tx_panic, ContractContainerRef, StaticVarData};

pub struct DebugSCInstance {
    pub tx_context_ref: TxContextRef,
    pub contract_container_ref: ContractContainerRef,
    pub static_var_data_ref: Arc<StaticVarData>,
}

impl DebugSCInstance {
    pub fn new(tx_context_ref: TxContextRef, contract_container: ContractContainerRef) -> Self {
        DebugSCInstance {
            tx_context_ref,
            contract_container_ref: contract_container,
            static_var_data_ref: Arc::new(StaticVarData::default()),
        }
    }
}

impl Instance for DebugSCInstance {
    fn call(&self, func_name: &str) -> Result<(), String> {
        let tx_func_name = TxFunctionName::from(func_name);

        let result = catch_tx_panic(self.contract_container_ref.0.panic_message, || {
            let call_successful = self.contract_container_ref.0.call(&tx_func_name);
            if call_successful {
                Ok(())
            } else {
                Err(TxPanic::new(
                    ReturnCode::FunctionNotFound,
                    "invalid function (not found)",
                ))
            }
        });

        if let Err(tx_panic) = result {
            self.tx_context_ref
                .clone()
                .replace_tx_result_with_error(tx_panic);
        }

        Ok(())
    }

    fn check_signatures(&self) -> bool {
        true
    }

    fn has_function(&self, func_name: &str) -> bool {
        self.contract_container_ref.has_function(func_name)
    }

    fn get_exported_function_names(&self) -> Vec<String> {
        panic!("ContractContainer get_exported_function_names not yet supported")
    }

    fn set_points_limit(&self, _limit: u64) -> Result<(), String> {
        panic!("ContractContainerRef set_points_limit not supported")
    }

    fn set_points_used(&self, _points: u64) -> Result<(), String> {
        panic!("ContractContainerRef set_points_used not supported")
    }

    fn get_points_used(&self) -> Result<u64, String> {
        panic!("ContractContainerRef get_points_used not supported")
    }

    fn memory_length(&self) -> Result<u64, String> {
        panic!("ContractContainerRef memory_length not supported")
    }

    fn memory_ptr(&self) -> Result<*mut u8, String> {
        panic!("ContractContainerRef memory_ptr not supported")
    }

    fn memory_load(
        &self,
        _mem_ptr: MemPtr,
        _mem_length: MemLength,
    ) -> Result<&[u8], ExecutorError> {
        panic!("ContractContainerRef memory_load not supported")
    }

    fn memory_store(&self, _mem_ptr: MemPtr, _data: &[u8]) -> Result<(), ExecutorError> {
        panic!("ContractContainerRef memory_store not supported")
    }

    fn memory_grow(&self, _by_num_pages: u32) -> Result<u32, ExecutorError> {
        panic!("ContractContainerRef memory_grow not supported")
    }

    fn set_breakpoint_value(&self, _value: BreakpointValue) -> Result<(), String> {
        panic!("ContractContainerRef set_breakpoint_value not supported")
    }

    fn get_breakpoint_value(&self) -> Result<BreakpointValue, String> {
        panic!("ContractContainerRef get_breakpoint_value not supported")
    }

    fn reset(&self) -> Result<(), String> {
        panic!("ContractContainerRef reset not supported")
    }

    fn cache(&self) -> Result<Vec<u8>, String> {
        panic!("ContractContainerRef cache not supported")
    }

    fn on_stack_top_enter(&self) {
        DebugApiBackend::replace_current_tx_context(Some(self.tx_context_ref.clone()));
        DebugApiBackend::replace_static_var_data(Some(self.static_var_data_ref.clone()));
    }

    fn on_stack_top_leave(&self) {
        DebugApiBackend::replace_current_tx_context(None);
        DebugApiBackend::replace_static_var_data(None);
    }
}
