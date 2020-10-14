use alloc::vec::Vec;

/// Trait that allows writing of data.
pub trait Output {
	/// Write to the output.
	fn write(&mut self, bytes: &[u8]);

	/// Write a single byte to the output.
	fn push_byte(&mut self, byte: u8) {
		self.write(&[byte]);
	}
}

impl Output for Vec<u8> {
	fn write(&mut self, bytes: &[u8]) {
		self.extend_from_slice(bytes)
	}
}
