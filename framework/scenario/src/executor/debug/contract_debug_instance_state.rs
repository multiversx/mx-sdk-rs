use multiversx_chain_vm_executor::{
    BreakpointValue, ExecutorError, InstanceState, MemLength, MemPtr,
};

#[derive(Clone, Debug)]
pub struct ContractDebugInstanceState;

impl ContractDebugInstanceState {
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
}

impl InstanceState for ContractDebugInstanceState {
    fn get_points_limit(&self) -> Result<u64, String> {
        Ok(1)
    }

    fn set_points_used(&self, _points: u64) -> Result<(), String> {
        Ok(())
    }

    fn get_points_used(&self) -> Result<u64, String> {
        Ok(0)
    }

    fn memory_length(&self) -> Result<u64, String> {
        panic!("ContractDebugInstanceState memory_length not supported")
    }

    fn memory_ptr(&self) -> Result<*mut u8, String> {
        panic!("ContractDebugInstanceState memory_ptr not supported")
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
        panic!("ContractDebugInstanceState memory_grow not supported")
    }

    fn set_breakpoint_value(&self, breakpoint_value: BreakpointValue) -> Result<(), String> {
        std::panic::panic_any(breakpoint_value)
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
        let (offset, length) = ContractDebugInstanceState::main_memory_ptr(&data);
        let re_slice = unsafe { ContractDebugInstanceState::main_memory_load(offset, length) };
        let cloned = re_slice.to_vec();
        assert_eq!(data, cloned);
    }

    fn assert_mem_store_sound(mut data: Vec<u8>) {
        let new_data: Vec<u8> = data.iter().map(|x| x * 2).collect();
        let (offset, length) = ContractDebugInstanceState::main_memory_mut_ptr(&mut data);
        assert_eq!(length, data.len() as isize);
        unsafe {
            ContractDebugInstanceState::main_memory_store(offset, &new_data);
        }
        assert_eq!(data, new_data);
    }
}
