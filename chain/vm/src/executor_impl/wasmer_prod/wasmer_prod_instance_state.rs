use multiversx_chain_vm_executor::{
    BreakpointValue, ExecutorError, InstanceFull, InstanceState, MemLength, MemPtr,
};
use multiversx_chain_vm_executor_wasmer::WasmerInstance;

use std::rc::{Rc, Weak};

#[derive(Clone)]
pub struct WasmerProdInstanceState {
    inner_instance_ref: Weak<WasmerInstance>,
}

impl WasmerProdInstanceState {
    pub fn new(inner_instance_ref: Weak<WasmerInstance>) -> Self {
        WasmerProdInstanceState { inner_instance_ref }
    }

    fn instance_rc(&self) -> Result<Rc<WasmerInstance>, String> {
        self.inner_instance_ref
            .upgrade()
            .map_or_else(|| Err("bad wasmer instance pointer".to_owned()), Ok)
    }
}

impl InstanceState for WasmerProdInstanceState {
    fn get_points_limit(&self) -> Result<u64, String> {
        // InstanceState::get_points_limit(&self)
        // self.instance_rc()?.get_points_limit()
        todo!()
    }

    fn set_points_used(&self, points: u64) -> Result<(), String> {
        self.instance_rc()?.set_points_used(points)
    }

    fn get_points_used(&self) -> Result<u64, String> {
        self.instance_rc()?.get_points_used()
    }

    fn memory_length(&self) -> Result<u64, String> {
        self.instance_rc()?.memory_length()
    }

    fn memory_ptr(&self) -> Result<*mut u8, String> {
        self.instance_rc()?.memory_ptr()
    }

    fn memory_load(&self, mem_ptr: MemPtr, mem_length: MemLength) -> Result<&[u8], ExecutorError> {
        // TODO: return error
        assert!(
            self.inner_instance_ref.strong_count() > 0,
            "instance reference dropped"
        );
        unsafe {
            let box_ref = self
                .inner_instance_ref
                .as_ptr()
                .as_ref()
                .expect("null instance pointer");
            box_ref.memory_load(mem_ptr, mem_length)
        }
    }

    fn memory_store(&self, mem_ptr: MemPtr, data: &[u8]) -> Result<(), ExecutorError> {
        self.instance_rc()?.memory_store(mem_ptr, data)
    }

    fn memory_grow(&self, by_num_pages: u32) -> Result<u32, ExecutorError> {
        self.instance_rc()?.memory_grow(by_num_pages)
    }

    fn set_breakpoint_value(&self, value: BreakpointValue) -> Result<(), String> {
        self.instance_rc()?.set_breakpoint_value(value)
    }
}
