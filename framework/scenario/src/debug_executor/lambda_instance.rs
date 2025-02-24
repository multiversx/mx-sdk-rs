use std::sync::Mutex;

use multiversx_chain_vm_executor::{
    BreakpointValue, CompilationOptions, Executor, ExecutorError, Instance, MemLength, MemPtr,
    OpcodeCost,
};

pub const LAMBDA_FUNCTION_MARKER: &str = "<LAMBDA>";

pub struct LambdaInstance<F>
where
    F: FnOnce(),
{
    pub lambda: Mutex<Option<F>>,
}

impl<F> LambdaInstance<F>
where
    F: FnOnce(),
{
    pub fn new(f: F) -> Self {
        LambdaInstance {
            lambda: Mutex::new(Some(f)),
        }
    }
}

impl<F> Instance for LambdaInstance<F>
where
    F: FnOnce(),
{
    fn call(&self, func_name: &str) -> Result<(), String> {
        if func_name == LAMBDA_FUNCTION_MARKER {
            let lambda = self
                .lambda
                .lock()
                .unwrap()
                .take()
                .expect("lambda function already used or not initialized");
            lambda();
            Ok(())
        } else {
            Err("LambdaInstance incorrectly used".to_owned())
        }
    }

    fn check_signatures(&self) -> bool {
        true
    }

    fn has_function(&self, _func_name: &str) -> bool {
        false
    }

    fn get_exported_function_names(&self) -> Vec<String> {
        Vec::new()
    }

    fn set_points_limit(&self, _limit: u64) -> Result<(), String> {
        panic!("LambdaInstance set_points_limit not supported")
    }

    fn set_points_used(&self, _points: u64) -> Result<(), String> {
        panic!("LambdaInstance set_points_used not supported")
    }

    fn get_points_used(&self) -> Result<u64, String> {
        panic!("LambdaInstance get_points_used not supported")
    }

    fn memory_length(&self) -> Result<u64, String> {
        panic!("LambdaInstance memory_length not supported")
    }

    fn memory_ptr(&self) -> Result<*mut u8, String> {
        panic!("LambdaInstance memory_ptr not supported")
    }

    fn memory_load(
        &self,
        _mem_ptr: MemPtr,
        _mem_length: MemLength,
    ) -> Result<&[u8], ExecutorError> {
        panic!("LambdaInstance memory_load not supported")
    }

    fn memory_store(&self, _mem_ptr: MemPtr, _data: &[u8]) -> Result<(), ExecutorError> {
        panic!("LambdaInstance memory_store not supported")
    }

    fn memory_grow(&self, _by_num_pages: u32) -> Result<u32, ExecutorError> {
        panic!("LambdaInstance memory_grow not supported")
    }

    fn set_breakpoint_value(&self, _value: BreakpointValue) -> Result<(), String> {
        panic!("LambdaInstance set_breakpoint_value not supported")
    }

    fn get_breakpoint_value(&self) -> Result<BreakpointValue, String> {
        panic!("LambdaInstance get_breakpoint_value not supported")
    }

    fn reset(&self) -> Result<(), String> {
        panic!("LambdaInstance reset not supported")
    }

    fn cache(&self) -> Result<Vec<u8>, String> {
        panic!("LambdaInstance cache not supported")
    }
}
