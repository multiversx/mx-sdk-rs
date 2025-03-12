use multiversx_chain_vm_executor::{BreakpointValue, ExecutorError, Instance, MemLength, MemPtr};

use std::rc::Rc;

#[derive(Clone)]
pub struct WrappedInstance {
    inner_instance_ref: Rc<Box<dyn Instance>>,
}

impl WrappedInstance {
    pub fn new(inner_instance_ref: Rc<Box<dyn Instance>>) -> Self {
        WrappedInstance { inner_instance_ref }
    }
}

impl Instance for WrappedInstance {
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

    fn set_points_used(&self, points: u64) -> Result<(), String> {
        self.inner_instance_ref.set_points_used(points)
    }

    fn get_points_used(&self) -> Result<u64, String> {
        self.inner_instance_ref.get_points_used()
    }

    fn memory_length(&self) -> Result<u64, String> {
        self.inner_instance_ref.memory_length()
    }

    fn memory_ptr(&self) -> Result<*mut u8, String> {
        self.inner_instance_ref.memory_ptr()
    }

    fn memory_load(&self, mem_ptr: MemPtr, mem_length: MemLength) -> Result<&[u8], ExecutorError> {
        self.inner_instance_ref.memory_load(mem_ptr, mem_length)
    }

    fn memory_store(&self, mem_ptr: MemPtr, data: &[u8]) -> Result<(), ExecutorError> {
        self.inner_instance_ref.memory_store(mem_ptr, data)
    }

    fn memory_grow(&self, by_num_pages: u32) -> Result<u32, ExecutorError> {
        self.inner_instance_ref.memory_grow(by_num_pages)
    }

    fn set_breakpoint_value(&self, value: BreakpointValue) -> Result<(), String> {
        self.inner_instance_ref.set_breakpoint_value(value)
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
