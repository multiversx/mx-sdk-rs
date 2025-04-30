use anyhow::anyhow;
use multiversx_chain_vm_executor::{
    BreakpointValue, ExecutorError, Instance, InstanceCallError, InstanceLegacy,
};
use multiversx_chain_vm_executor_wasmer::WasmerInstance;

use std::rc::Rc;

#[derive(Clone)]
pub struct WasmerProdInstance {
    inner_instance_ref: Rc<WasmerInstance>,
}

impl WasmerProdInstance {
    pub fn new(inner_instance_ref: Rc<WasmerInstance>) -> Self {
        WasmerProdInstance { inner_instance_ref }
    }
}

impl Instance for WasmerProdInstance {
    fn call(&self, func_name: &str) -> Result<(), InstanceCallError> {
        let result = self.inner_instance_ref.call(func_name);

        match result {
            Ok(()) => Ok(()),
            Err(err) => {
                if err == "function not found" {
                    return Err(InstanceCallError::FunctionNotFound);
                }

                let breakpoint_value =
                    self.inner_instance_ref
                        .get_breakpoint_value()
                        .map_err(|err| {
                            InstanceCallError::RuntimeError(
                                anyhow!("wrapped instance error: {err}").into(),
                            )
                        })?;

                if breakpoint_value != BreakpointValue::None {
                    return Err(InstanceCallError::Breakpoint(breakpoint_value));
                }

                Err(InstanceCallError::RuntimeError(
                    anyhow!("wrapped instance error: {err}").into(),
                ))
            },
        }
    }

    fn check_signatures(&self) -> bool {
        self.inner_instance_ref.check_signatures()
    }

    fn has_function(&self, func_name: &str) -> bool {
        self.inner_instance_ref.has_function(func_name)
    }

    fn get_exported_function_names(&self) -> Vec<String> {
        self.inner_instance_ref.get_exported_function_names()
    }

    fn set_points_limit(&self, limit: u64) -> Result<(), ExecutorError> {
        self.inner_instance_ref
            .set_points_limit(limit)
            .map_err(|err| anyhow!("wrapped instance error: {err}").into())
    }

    fn get_points_used(&self) -> Result<u64, ExecutorError> {
        self.inner_instance_ref
            .get_points_used()
            .map_err(|err| anyhow!("wrapped instance error: {err}").into())
    }

    fn get_breakpoint_value(&self) -> Result<BreakpointValue, ExecutorError> {
        self.inner_instance_ref
            .get_breakpoint_value()
            .map_err(|err| anyhow!("wrapped instance error: {err}").into())
    }

    fn reset(&self) -> Result<(), ExecutorError> {
        self.inner_instance_ref
            .reset()
            .map_err(|err| anyhow!("wrapped instance error: {err}").into())
    }

    fn cache(&self) -> Result<Vec<u8>, ExecutorError> {
        self.inner_instance_ref
            .cache()
            .map_err(|err| anyhow!("wrapped instance error: {err}").into())
    }
}
