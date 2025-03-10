use multiversx_chain_vm_executor::{MemLength, MemPtr};

/// TODO: delete.
#[derive(Debug)]
pub struct DebugApiVMHooksHandler;

impl DebugApiVMHooksHandler {
    /// Interprets the input as a regular pointer.
    ///
    /// ## Safety
    ///
    /// Thr offset and the length must point to valid memory.
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
}
