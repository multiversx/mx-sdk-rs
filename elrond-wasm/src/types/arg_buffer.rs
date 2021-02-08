use alloc::vec::Vec;
use elrond_codec::TopEncodeOutput;

/// Helper structure for providing arguments to all SC call functions other than async_call_raw.
/// It keeps argument lengths separately from the argument data itself.
/// Argument data is concatenated into a single byte buffer.
pub struct ArgBuffer {
	arg_lengths: Vec<usize>,
	arg_data: Vec<u8>,
}

impl ArgBuffer {
	pub fn new() -> Self {
		ArgBuffer {
			arg_lengths: Vec::new(),
			arg_data: Vec::new(),
		}
	}

	pub fn push_raw_arg(&mut self, arg_bytes: &[u8]) {
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
}

impl Default for ArgBuffer {
	fn default() -> Self {
		Self::new()
	}
}

impl TopEncodeOutput for &mut ArgBuffer {
	fn set_slice_u8(self, bytes: &[u8]) {
		self.arg_lengths.push(bytes.len());
		self.arg_data.extend_from_slice(bytes);
	}
}
