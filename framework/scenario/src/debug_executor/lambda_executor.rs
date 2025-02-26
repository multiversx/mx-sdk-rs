use std::sync::Mutex;

use multiversx_chain_vm_executor::{
    CompilationOptions, Executor, ExecutorError, Instance, OpcodeCost,
};

pub const LAMBDA_CODE_MARKER: &str = "<LAMBDA>";

pub struct LambdaExecutor<F>
where
    F: FnOnce(),
{
    pub lambda: Mutex<Option<F>>,
}

impl<F> LambdaExecutor<F>
where
    F: FnOnce(),
{
    pub fn new(f: F) -> Self {
        LambdaExecutor {
            lambda: Mutex::new(Some(f)),
        }
    }
}

impl<F> Executor for LambdaExecutor<F>
where
    F: FnOnce(),
{
    fn set_vm_hooks_ptr(
        &mut self,
        _vm_hooks_ptr: *mut std::ffi::c_void,
    ) -> Result<(), ExecutorError> {
        todo!()
    }

    fn set_opcode_cost(&mut self, _opcode_cost: &OpcodeCost) -> Result<(), ExecutorError> {
        todo!()
    }

    fn new_instance(
        &self,
        _wasm_bytes: &[u8],
        _compilation_options: &CompilationOptions,
    ) -> Result<Box<dyn Instance>, ExecutorError> {
        // if wasm_bytes == LAMBDA_CODE_MARKER.as_bytes() {
        //     let lambda = self
        //         .lambda
        //         .lock()
        //         .unwrap()
        //         .take()
        //         .expect("lambda function already used or not initialized");
        //     Ok(Box::new(LambdaInstance::new(lambda)))
        // } else {
        //     Err(Box::new(ExecutorDeclinedError))
        // }
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
