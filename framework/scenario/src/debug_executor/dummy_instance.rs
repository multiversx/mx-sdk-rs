use multiversx_chain_vm_executor::{BreakpointValue, ExecutorError, Instance, MemLength, MemPtr};

/// Not used currently.
pub struct DummyInstance;

impl Instance for DummyInstance {
    fn call(&self, _func_name: &str) -> Result<(), String> {
        panic!("DummyInstance call not supported")
    }

    fn check_signatures(&self) -> bool {
        panic!("DummyInstance check_signatures not supported")
    }

    fn has_function(&self, _func_name: &str) -> bool {
        panic!("DummyInstance has_function not supported")
    }

    fn get_exported_function_names(&self) -> Vec<String> {
        panic!("DummyInstance get_exported_function_names not supported")
    }

    fn set_points_limit(&self, _limit: u64) -> Result<(), String> {
        panic!("DummyInstanceRef set_points_limit not supported")
    }

    fn set_points_used(&self, _points: u64) -> Result<(), String> {
        panic!("DummyInstanceRef set_points_used not supported")
    }

    fn get_points_used(&self) -> Result<u64, String> {
        panic!("DummyInstanceRef get_points_used not supported")
    }

    fn memory_length(&self) -> Result<u64, String> {
        panic!("DummyInstanceRef memory_length not supported")
    }

    fn memory_ptr(&self) -> Result<*mut u8, String> {
        panic!("DummyInstanceRef memory_ptr not supported")
    }

    fn memory_load(
        &self,
        _mem_ptr: MemPtr,
        _mem_length: MemLength,
    ) -> Result<&[u8], ExecutorError> {
        panic!("DummyInstanceRef memory_load not supported")
    }

    fn memory_store(&self, _mem_ptr: MemPtr, _data: &[u8]) -> Result<(), ExecutorError> {
        panic!("DummyInstanceRef memory_store not supported")
    }

    fn memory_grow(&self, _by_num_pages: u32) -> Result<u32, ExecutorError> {
        panic!("DummyInstanceRef memory_grow not supported")
    }

    fn set_breakpoint_value(&self, _value: BreakpointValue) -> Result<(), String> {
        panic!("DummyInstanceRef set_breakpoint_value not supported")
    }

    fn get_breakpoint_value(&self) -> Result<BreakpointValue, String> {
        panic!("DummyInstanceRef get_breakpoint_value not supported")
    }

    fn reset(&self) -> Result<(), String> {
        panic!("DummyInstanceRef reset not supported")
    }

    fn cache(&self) -> Result<Vec<u8>, String> {
        panic!("DummyInstanceRef cache not supported")
    }
}
