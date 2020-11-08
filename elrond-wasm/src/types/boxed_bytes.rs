use alloc::alloc::{alloc, alloc_zeroed, Layout};
use alloc::boxed::Box;
use alloc::vec::Vec;
use elrond_codec::*;

/// Simple wrapper around a boxed byte slice,
/// but with a lot of optimized methods for manipulating it.
/// The focus is on readucing code size rather improving speed.
#[derive(Clone, PartialEq, Debug)]
pub struct BoxedBytes(Box<[u8]>);

impl BoxedBytes {
	pub fn empty() -> Self {
		BoxedBytes(Box::from([0u8; 0]))
	}

	pub fn zeros(len: usize) -> Self {
		unsafe {
			let layout = Layout::from_size_align(len, core::mem::align_of::<u8>()).unwrap();
			let bytes_ptr = alloc_zeroed(layout);
			let bytes_box = Box::from_raw(core::slice::from_raw_parts_mut(bytes_ptr, len));
			BoxedBytes(bytes_box)
		}
	}

	pub unsafe fn allocate(len: usize) -> Self {
		let layout = Layout::from_size_align(len, core::mem::align_of::<u8>()).unwrap();
		let bytes_ptr = alloc(layout);
		let bytes_box = Box::from_raw(core::slice::from_raw_parts_mut(bytes_ptr, len));
		BoxedBytes(bytes_box)
	}

	#[inline]
	pub fn as_mut_ptr(&mut self) -> *mut u8 {
		self.0.as_mut_ptr()
	}

	#[inline]
	pub fn len(&self) -> usize {
		self.0.len()
	}

	#[inline]
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}

	#[inline]
	pub fn into_box(self) -> Box<[u8]> {
		self.0
	}

	#[inline]
	pub fn as_slice(&self) -> &[u8] {
		&*self.0
	}

	/// Create new instance by concatenating several byte slices.
	pub fn from_concat(slices: &[&[u8]]) -> Self {
		let mut total_len = 0usize;
		let mut index = slices.len();
		while index > 0 {
			index -= 1;
			total_len += slices[index].len();
		}
		unsafe {
			let layout = Layout::from_size_align(total_len, core::mem::align_of::<u8>()).unwrap();
			let bytes_ptr = alloc(layout);
			let mut current_index = 0usize;
			for slice in slices.iter() {
				core::ptr::copy_nonoverlapping(
					slice.as_ptr(),
					bytes_ptr.offset(current_index as isize),
					slice.len(),
				);
				current_index += slice.len();
			}
			let bytes_box = Box::from_raw(core::slice::from_raw_parts_mut(bytes_ptr, total_len));
			BoxedBytes(bytes_box)
		}
	}
}

impl AsRef<[u8]> for BoxedBytes {
	#[inline]
	fn as_ref(&self) -> &[u8] {
		&*self.0
	}
}

impl<'a> From<&'a [u8]> for BoxedBytes {
	#[inline]
	fn from(byte_slice: &'a [u8]) -> Self {
		BoxedBytes(Box::from(byte_slice))
	}
}

impl From<Vec<u8>> for BoxedBytes {
	#[inline]
	fn from(v: Vec<u8>) -> Self {
		BoxedBytes(v.into_boxed_slice())
	}
}

impl NestedEncode for BoxedBytes {
	#[inline]
	fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
		dest.write(self.as_ref());
		Ok(())
	}

	#[inline]
	fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
		&self,
		dest: &mut O,
		_: ExitCtx,
		_: fn(ExitCtx, EncodeError) -> !,
	) {
		dest.write(self.as_ref());
	}
}

impl TopEncode for BoxedBytes {
	#[inline]
	fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
		output.set_slice_u8(self.as_ref());
		Ok(())
	}

	#[inline]
	fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
		&self,
		output: O,
		_: ExitCtx,
		_: fn(ExitCtx, EncodeError) -> !,
	) {
		output.set_slice_u8(self.as_ref());
	}
}

impl NestedDecode for BoxedBytes {
	fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
		let size = usize::dep_decode(input)?;
		let byte_slice = input.read_slice(size)?;
		Ok(byte_slice.into())
	}

	fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
		input: &mut I,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		let size = usize::dep_decode_or_exit(input, c.clone(), exit);
		let byte_slice = input.read_slice_or_exit(size, c, exit);
		byte_slice.into()
	}
}

impl TopDecode for BoxedBytes {
	fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
		Ok(BoxedBytes(input.into_boxed_slice_u8()))
	}

	fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
		input: I,
		_: ExitCtx,
		_: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		BoxedBytes(input.into_boxed_slice_u8())
	}
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_concat_1() {
		let bb = BoxedBytes::from_concat(&[&b"abc"[..], &b"def"[..]]);
		assert_eq!(bb, BoxedBytes::from(&b"abcdef"[..]));
	}

	#[test]
	fn test_concat_2() {
		let bb = BoxedBytes::from_concat(&[&b"abc"[..], &b""[..], &b"def"[..]]);
		assert_eq!(bb, BoxedBytes::from(&b"abcdef"[..]));
	}

	#[test]
	fn test_concat_empty_1() {
		let bb = BoxedBytes::from_concat(&[&b""[..], &b""[..], &b""[..]]);
		assert_eq!(bb, BoxedBytes::from(&b""[..]));
	}

	#[test]
	fn test_concat_empty_2() {
		let bb = BoxedBytes::from_concat(&[]);
		assert_eq!(bb, BoxedBytes::from(&b""[..]));
	}

	#[test]
	fn test_is_empty() {
		assert!(BoxedBytes::empty().is_empty());
	}
}
