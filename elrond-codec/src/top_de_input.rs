use crate::num_conv::bytes_to_number;
use crate::transmute::vec_into_boxed_slice;
use alloc::boxed::Box;
use alloc::vec::Vec;

/// Trait that abstracts away an underlying API for a top-level object deserializer.
/// The underlying API can provide pre-parsed i64/u64 or pre-bundled boxed slices.
pub trait TopDecodeInput: Sized {
	/// Length of the underlying data, in bytes.
	fn byte_len(&self) -> usize;

	/// Provides the underlying data as an owned byte slice box.
	/// Consumes the input object in the process.
	fn into_boxed_slice_u8(self) -> Box<[u8]>;

	/// Retrieves the underlying data as a pre-parsed u64.
	/// Expected to panic if the conversion is not possible.
	///
	/// Consumes the input object in the process.
	fn into_u64(self) -> u64 {
		bytes_to_number(&*self.into_boxed_slice_u8(), false)
	}

	/// Retrieves the underlying data as a pre-parsed i64.
	/// Expected to panic if the conversion is not possible.
	///
	/// Consumes the input object in the process.
	fn into_i64(self) -> i64 {
		bytes_to_number(&*self.into_boxed_slice_u8(), true) as i64
	}

	/// Unless you're developing elrond-wasm, please ignore.
	///
	/// Shortcut for sending a BigInt managed by the API to the API directly via its handle.
	///
	/// - ArwenBigInt + finish API
	/// - ArwenBigInt + set storage
	/// Not used for:
	/// - RustBigInt
	/// - async call
	#[doc(hidden)]
	#[inline]
	fn try_get_big_int_handle(&self) -> (bool, i32) {
		(false, -1)
	}

	/// Unless you're developing elrond-wasm, please ignore.
	///
	/// Shortcut for sending a BigUint managed by the API to the API directly via its handle.
	///
	/// Used for:
	/// - ArwenBigUint + finish API
	/// - ArwenBigUint + set storage
	/// Not used for:
	/// - RustBigUint
	/// - async call
	/// - anything else
	///
	#[doc(hidden)]
	#[inline]
	fn try_get_big_uint_handle(&self) -> (bool, i32) {
		(false, -1)
	}
}

impl TopDecodeInput for Box<[u8]> {
	fn byte_len(&self) -> usize {
		self.len()
	}

	fn into_boxed_slice_u8(self) -> Box<[u8]> {
		self
	}
}

impl TopDecodeInput for Vec<u8> {
	fn byte_len(&self) -> usize {
		self.len()
	}

	fn into_boxed_slice_u8(self) -> Box<[u8]> {
		vec_into_boxed_slice(self)
	}
}

impl<'a> TopDecodeInput for &'a [u8] {
	fn byte_len(&self) -> usize {
		self.len()
	}

	fn into_boxed_slice_u8(self) -> Box<[u8]> {
		Box::from(self)
	}
}
