use std::rc::Rc;

use multiversx_chain_vm::{
    host::context::{TxContextRef, TxFunctionName, TxPanic},
    host::runtime::RuntimeInstanceCall,
};
use multiversx_chain_vm_executor::{BreakpointValue, ExecutorError, Instance, MemLength, MemPtr};
use multiversx_sc::chain_core::types::ReturnCode;

use super::{
    catch_tx_panic, ContractContainer, ContractContainerRef, ContractDebugStack, StaticVarData,
};

/// Used as a flag to check the instance under lambda calls.
/// Since it is an invalid function name, any other instance should reject it.
const FUNC_CONTEXT_PUSH: &str = "<ContractDebugInstance-PushContext>";
const FUNC_CONTEXT_POP: &str = "<ContractDebugInstance-PopContext>";

#[derive(Clone, Debug)]
pub struct ContractDebugInstance {
    pub tx_context_ref: TxContextRef,
    pub contract_container_ref: ContractContainerRef,
    pub static_var_ref: Rc<StaticVarData>,
}

impl ContractDebugInstance {
    pub fn new(tx_context_ref: TxContextRef, contract_container: ContractContainerRef) -> Self {
        ContractDebugInstance {
            tx_context_ref,
            contract_container_ref: contract_container,
            static_var_ref: Default::default(),
        }
    }

    /// Dummy instance for tests where no proper context is created on stack.
    pub fn dummy() -> Self {
        ContractDebugInstance {
            tx_context_ref: TxContextRef::dummy(),
            contract_container_ref: ContractContainerRef::new(ContractContainer::dummy()),
            static_var_ref: Default::default(),
        }
    }

    /// Interprets the input as a regular pointer.
    ///
    /// ## Safety
    ///
    /// The offset and the length must point to valid memory.
    pub unsafe fn main_memory_load(mem_ptr: MemPtr, mem_length: MemLength) -> &'static [u8] {
        unsafe {
            let bytes_ptr =
                std::ptr::slice_from_raw_parts(mem_ptr as *const u8, mem_length as usize);
            &*bytes_ptr
        }
    }

    /// Interprets the input as a regular pointer and writes to current memory.
    ///
    /// ## Safety
    ///
    /// The offset and the length must point to valid memory.
    pub unsafe fn main_memory_store(offset: MemPtr, data: &[u8]) {
        unsafe {
            std::ptr::copy(data.as_ptr(), offset as *mut u8, data.len());
        }
    }

    pub fn main_memory_ptr(bytes: &[u8]) -> (MemPtr, MemLength) {
        (bytes.as_ptr() as MemPtr, bytes.len() as MemLength)
    }

    pub fn main_memory_mut_ptr(bytes: &mut [u8]) -> (MemPtr, MemLength) {
        (bytes.as_ptr() as MemPtr, bytes.len() as MemLength)
    }

    pub fn wrap_lambda_call<F>(
        panic_message_flag: bool,
        instance_call: RuntimeInstanceCall<'_>,
        f: F,
    ) where
        F: FnOnce(),
    {
        // assert!(
        //     instance_call.func_name == TxFunctionName::WHITEBOX_CALL.as_str()
        //         || instance_call.func_name == "init", // TODO make it also WHITEBOX_CALL or some whitebox init
        //     "misconfigured whitebox call: {}",
        //     instance_call.func_name,
        // );

        assert!(
            instance_call.instance.has_function(FUNC_CONTEXT_PUSH),
            "lambda call is not running on top of a DebugSCInstance instance"
        );

        let _ = instance_call.instance.call(FUNC_CONTEXT_PUSH);

        let result = catch_tx_panic(panic_message_flag, || {
            f();
            Ok(())
        });

        if let Err(tx_panic) = result {
            ContractDebugStack::static_peek()
                .tx_context_ref
                .replace_tx_result_with_error(tx_panic);
        }

        let _ = instance_call.instance.call(FUNC_CONTEXT_POP);
    }

    fn call_endpoint(&self, func_name: &str) -> Result<(), String> {
        let tx_func_name = TxFunctionName::from(func_name);

        ContractDebugStack::static_push(self.clone());

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

        ContractDebugStack::static_pop();

        Ok(())
    }
}

impl Instance for ContractDebugInstance {
    fn call(&self, func_name: &str) -> Result<(), String> {
        match func_name {
            FUNC_CONTEXT_PUSH => {
                ContractDebugStack::static_push(self.clone());
                Ok(())
            },
            FUNC_CONTEXT_POP => {
                ContractDebugStack::static_pop();
                Ok(())
            },
            _ => self.call_endpoint(func_name),
        }
    }

    fn check_signatures(&self) -> bool {
        true
    }

    fn has_function(&self, func_name: &str) -> bool {
        match func_name {
            FUNC_CONTEXT_PUSH => true,
            FUNC_CONTEXT_POP => true,
            _ => self.contract_container_ref.has_function(func_name),
        }
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

    fn memory_load(&self, mem_ptr: MemPtr, mem_length: MemLength) -> Result<&[u8], ExecutorError> {
        let data = unsafe { Self::main_memory_load(mem_ptr, mem_length) };
        Ok(data)
    }

    fn memory_store(&self, mem_ptr: MemPtr, data: &[u8]) -> Result<(), ExecutorError> {
        unsafe {
            Self::main_memory_store(mem_ptr, data);
        }
        Ok(())
    }

    fn memory_grow(&self, _by_num_pages: u32) -> Result<u32, ExecutorError> {
        panic!("ContractContainerRef memory_grow not supported")
    }

    fn set_breakpoint_value(&self, breakpoint_value: BreakpointValue) -> Result<(), String> {
        std::panic::panic_any(breakpoint_value)
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

#[cfg(test)]
#[allow(deprecated)]
mod test {
    use super::*;

    #[test]
    fn test_mem_ptr_conversion() {
        assert_mem_load_sound(vec![]);
        assert_mem_load_sound(vec![1]);
        assert_mem_load_sound(vec![1, 2, 3]);

        assert_mem_store_sound(vec![]);
        assert_mem_store_sound(vec![1]);
        assert_mem_store_sound(vec![1, 2, 3]);
    }

    fn assert_mem_load_sound(data: Vec<u8>) {
        let (offset, length) = ContractDebugInstance::main_memory_ptr(&data);
        let re_slice = unsafe { ContractDebugInstance::main_memory_load(offset, length) };
        let cloned = re_slice.to_vec();
        assert_eq!(data, cloned);
    }

    fn assert_mem_store_sound(mut data: Vec<u8>) {
        let new_data: Vec<u8> = data.iter().map(|x| x * 2).collect();
        let (offset, length) = ContractDebugInstance::main_memory_mut_ptr(&mut data);
        assert_eq!(length, data.len() as isize);
        unsafe {
            ContractDebugInstance::main_memory_store(offset, &new_data);
        }
        assert_eq!(data, new_data);
    }
}
