use crate::{
    api::ManagedTypeApi,
    codec::TopEncodeOutput,
    types::{heap::BoxedBytes, ManagedArgBuffer},
};
use alloc::vec::Vec;

/// Helper structure for providing arguments to all SC call functions other than async_call_raw.
/// It keeps argument lengths separately from the argument data itself.
/// Argument data is concatenated into a single byte buffer.
#[must_use]
pub struct ArgBuffer {
    arg_lengths: Vec<usize>,
    arg_data: Vec<u8>,
}

#[allow(clippy::return_self_not_must_use)]
impl ArgBuffer {
    pub fn new() -> Self {
        ArgBuffer {
            arg_lengths: Vec::new(),
            arg_data: Vec::new(),
        }
    }

    pub fn push_argument_bytes(&mut self, arg_bytes: &[u8]) {
        self.arg_lengths.push(arg_bytes.len());
        self.arg_data.extend_from_slice(arg_bytes);
    }

    pub fn num_args(&self) -> usize {
        self.arg_lengths.len()
    }

    pub fn arg_lengths_bytes_ptr(&self) -> *const u8 {
        self.arg_lengths.as_ptr() as *const u8
    }

    pub fn arg_data_ptr(&self) -> *const u8 {
        self.arg_data.as_ptr()
    }

    /// returns the raw arg data
    pub fn arg_data(&self) -> &[u8] {
        self.arg_data.as_slice()
    }

    /// returns the raw arg data lengths
    pub fn arg_lengths(&self) -> &[usize] {
        self.arg_lengths.as_slice()
    }

    /// Quick for-each using closures.
    /// TODO: also write an Iterator at some point, but beware of wasm bloat.
    pub fn for_each_arg<F: FnMut(&[u8])>(&self, mut f: F) {
        let mut data_offset = 0;
        for &arg_length in self.arg_lengths.iter() {
            let next_data_offset = data_offset + arg_length;
            f(&self.arg_data[data_offset..next_data_offset]);
            data_offset = next_data_offset;
        }
    }

    pub fn is_empty(&self) -> bool {
        self.arg_lengths.is_empty()
    }

    /// Concatenates 2 ArgBuffer. Consumes both arguments in the process.
    pub fn concat(mut self, mut other: ArgBuffer) -> Self {
        self.arg_lengths.append(&mut other.arg_lengths);
        self.arg_data.append(&mut other.arg_data);
        self
    }
}

impl From<&[BoxedBytes]> for ArgBuffer {
    fn from(raw_args: &[BoxedBytes]) -> Self {
        let mut arg_buffer = ArgBuffer::new();
        for bytes in raw_args {
            arg_buffer.push_argument_bytes(bytes.as_slice());
        }
        arg_buffer
    }
}

impl<M: ManagedTypeApi> From<&ManagedArgBuffer<M>> for ArgBuffer
where
    M: ManagedTypeApi + 'static,
{
    fn from(managed_arg_buffer: &ManagedArgBuffer<M>) -> Self {
        let mut result = Self::new();
        for m_arg in managed_arg_buffer.data.into_iter() {
            result.push_argument_bytes(m_arg.to_boxed_bytes().as_slice());
        }
        result
    }
}

impl Default for ArgBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for ArgBuffer {
    fn clone(&self) -> Self {
        Self {
            arg_lengths: self.arg_lengths.clone(),
            arg_data: self.arg_data.clone(),
        }
    }
}

impl TopEncodeOutput for &mut ArgBuffer {
    type NestedBuffer = Vec<u8>;

    fn set_slice_u8(self, bytes: &[u8]) {
        self.arg_lengths.push(bytes.len());
        self.arg_data.extend_from_slice(bytes);
    }

    fn start_nested_encode(&self) -> Self::NestedBuffer {
        Vec::<u8>::new()
    }

    fn finalize_nested_encode(self, nb: Self::NestedBuffer) {
        self.set_slice_u8(nb.as_slice());
    }
}
