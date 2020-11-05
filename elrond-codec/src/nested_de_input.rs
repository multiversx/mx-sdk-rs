pub use crate::codec_err::DecodeError;

/// Trait that allows reading of data into a slice.
pub trait NestedDecodeInput {
	/// The remaining length of the input data.
	fn remaining_len(&mut self) -> usize;

	fn empty(&mut self) -> bool {
		self.remaining_len() == 0
	}

	/// Read the exact number of bytes required to fill the given buffer.
	fn read_into(&mut self, into: &mut [u8]) -> Result<(), DecodeError>;

	/// Read the exact number of bytes required to fill the given buffer.
	/// Exit early if there are not enough bytes to fill the result.
	fn read_into_or_exit<ExitCtx: Clone>(
		&mut self,
		into: &mut [u8],
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	);

	/// Read a single byte from the input.
	fn read_byte(&mut self) -> Result<u8, DecodeError> {
		let mut buf = [0u8];
		self.read_into(&mut buf[..])?;
		Ok(buf[0])
	}

	/// Read a single byte from the input.
	fn read_byte_or_exit<ExitCtx: Clone>(
		&mut self,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> u8 {
		let mut buf = [0u8];
		self.read_into_or_exit(&mut buf[..], c, exit);
		buf[0]
	}

	/// Read the exact number of bytes required to fill the given buffer.
	fn read_slice(&mut self, length: usize) -> Result<&[u8], DecodeError>;

	/// Read the exact number of bytes required to fill the given buffer.
	/// Exit directly if the input contains too few bytes.
	fn read_slice_or_exit<ExitCtx: Clone>(
		&mut self,
		length: usize,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> &[u8];

	/// Clears the input buffer and returns all remaining bytes.
	fn flush(&mut self) -> &[u8];
}

impl<'a> NestedDecodeInput for &'a [u8] {
	fn remaining_len(&mut self) -> usize {
		self.len()
	}

	fn read_into(&mut self, into: &mut [u8]) -> Result<(), DecodeError> {
		if into.len() > self.len() {
			return Err(DecodeError::INPUT_TOO_SHORT);
		}
		let len = into.len();
		into.copy_from_slice(&self[..len]);
		*self = &self[len..];
		Ok(())
	}

	fn read_into_or_exit<ExitCtx: Clone>(
		&mut self,
		into: &mut [u8],
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) {
		if into.len() > self.len() {
			exit(c, DecodeError::INPUT_TOO_SHORT);
		}
		let len = into.len();
		into.copy_from_slice(&self[..len]);
		*self = &self[len..];
	}

	fn read_slice(&mut self, length: usize) -> Result<&[u8], DecodeError> {
		if length > self.len() {
			return Err(DecodeError::INPUT_TOO_SHORT);
		}

		let (result, rest) = self.split_at(length);
		*self = rest;
		Ok(result)
	}

	fn read_slice_or_exit<ExitCtx: Clone>(
		&mut self,
		length: usize,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> &[u8] {
		if length > self.len() {
			exit(c, DecodeError::INPUT_TOO_SHORT);
		}

		let (result, rest) = self.split_at(length);
		*self = rest;
		result
	}

	fn flush(&mut self) -> &[u8] {
		let result = &self[..];
		*self = &[];
		result
	}
}
