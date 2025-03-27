use multiversx_chain_vm_executor::{BreakpointValue, Instance, InstanceFull};
use multiversx_chain_vm_executor_wasmer::WasmerInstance;

use std::rc::Rc;

#[derive(Clone)]
pub struct WasmerAltInstance {
    inner_instance_ref: Rc<WasmerInstance>,
}

impl WasmerAltInstance {
    pub fn new(inner_instance_ref: Rc<WasmerInstance>) -> Self {
        WasmerAltInstance { inner_instance_ref }
    }
}

impl Instance for WasmerAltInstance {
    fn call(&self, func_name: &str) -> Result<(), String> {
        self.inner_instance_ref.call(func_name)
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

    fn set_points_limit(&self, limit: u64) -> Result<(), String> {
        self.inner_instance_ref.set_points_limit(limit)
    }

    fn get_points_used(&self) -> Result<u64, String> {
        self.inner_instance_ref.get_points_used()
    }

    fn get_breakpoint_value(&self) -> Result<BreakpointValue, String> {
        self.inner_instance_ref.get_breakpoint_value()
    }

    fn reset(&self) -> Result<(), String> {
        self.inner_instance_ref.reset()
    }

    fn cache(&self) -> Result<Vec<u8>, String> {
        self.inner_instance_ref.cache()
    }
}

