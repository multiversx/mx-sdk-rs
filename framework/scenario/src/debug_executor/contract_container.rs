use multiversx_chain_vm::tx_mock::{TxContextRef, TxFunctionName, TxPanic};
use multiversx_chain_vm_executor::{BreakpointValue, ExecutorError, Instance, MemLength, MemPtr};
use multiversx_sc::contract_base::CallableContract;
use std::sync::Arc;

use super::{catch_tx_panic, StaticVarStack};

/// Contains a reference to a contract implementation.
///
/// It can optionally also contain an allowed endpoint whitelist, to simulate multi-contract.
pub struct ContractContainer {
    callable: Box<dyn CallableContract>,
    function_whitelist: Option<Vec<String>>,
    pub panic_message: bool,
}

impl ContractContainer {
    pub fn new(
        callable: Box<dyn CallableContract>,
        function_whitelist: Option<Vec<String>>,
        panic_message: bool,
    ) -> Self {
        ContractContainer {
            callable,
            function_whitelist,
            panic_message,
        }
    }

    fn validate_function_name(&self, function_name: &TxFunctionName) -> bool {
        if let Some(function_whitelist) = &self.function_whitelist {
            function_whitelist
                .iter()
                .any(|whitelisted_endpoint| whitelisted_endpoint.as_str() == function_name.as_str())
        } else {
            true
        }
    }

    pub fn call(&self, function_name: &TxFunctionName) -> bool {
        if self.validate_function_name(function_name) {
            self.callable.call(function_name.as_str())
        } else {
            false
        }
    }
}

/// Prepares the StaticVarStack and catches panics.
/// The result of the panics is written to the top of the TxContext stack.
pub fn contract_instance_wrapped_execution<F>(panic_message: bool, f: F)
where
    F: FnOnce() -> Result<(), TxPanic>,
{
    StaticVarStack::static_push();

    let result = catch_tx_panic(panic_message, f);

    if let Err(tx_panic) = result {
        TxContextRef::new_from_static().replace_tx_result_with_error(tx_panic);
    }

    StaticVarStack::static_pop();
}

#[derive(Clone)]
pub struct ContractContainerRef(pub(crate) Arc<ContractContainer>);

impl ContractContainerRef {
    pub fn new(contract_container: ContractContainer) -> Self {
        ContractContainerRef(Arc::new(contract_container))
    }
}

impl Instance for ContractContainerRef {
    fn call(&self, func_name: &str) -> Result<(), String> {
        let tx_func_name = TxFunctionName::from(func_name);

        contract_instance_wrapped_execution(self.0.panic_message, || {
            let call_successful = self.0.call(&tx_func_name);
            if call_successful {
                Ok(())
            } else {
                Err(TxPanic::new(1, "invalid function (not found)"))
            }
        });

        Ok(())
    }

    fn check_signatures(&self) -> bool {
        true
    }

    fn has_function(&self, func_name: &str) -> bool {
        let tx_func_name = TxFunctionName::from(func_name);
        self.0.validate_function_name(&tx_func_name)
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
}
