use crate::num_conv::top_encode_number_to_output;
use alloc::vec::Vec;

/// Specifies objects that can receive the result of a TopEncode computation.

/// in principle from NestedEncode performed on nested items.
///
/// All methods consume the object, so they can only be called once.
///
/// The trait is used in 3 scenarios:
/// - SC results
/// - `#[storage_set(...)]`
/// - Serialize async call.
pub trait TopEncodeOutput: Sized {
	fn set_slice_u8(self, bytes: &[u8]);

	fn set_u64(self, value: u64) {
		let mut buffer = Vec::<u8>::with_capacity(8);
		top_encode_number_to_output(&mut buffer, value, false);
		self.set_slice_u8(&buffer[..]);
	}

	fn set_i64(self, value: i64) {
		let mut buffer = Vec::<u8>::with_capacity(8);
		top_encode_number_to_output(&mut buffer, value as u64, true);
		self.set_slice_u8(&buffer[..]);
	}

	/// The unit type `()` is serializable, but some TopEncodeOutput implementations might want to treat it differently.
	/// For instance, SC function result units do not cause `finish` to be called, no empty result produced.
	#[doc(hidden)]
	#[inline]
	fn set_unit(self) {
		self.set_slice_u8(&[]);
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
	///
	/// Note: The byte representation is required as a lambda, so it is computed lazily.
	/// It should not be computed whenever the handle is present.
	#[doc(hidden)]
	#[inline]
	fn set_big_int_handle_or_bytes<F: FnOnce() -> Vec<u8>>(self, _handle: i32, else_bytes: F) {
		self.set_slice_u8(else_bytes().as_slice());
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
	/// Note: The byte representation is required as a lambda, so it is computed lazily.
	/// It should not be computed whenever the handle is present.
	#[doc(hidden)]
	#[inline]
	fn set_big_uint_handle_or_bytes<F: FnOnce() -> Vec<u8>>(self, _handle: i32, else_bytes: F) {
		self.set_slice_u8(else_bytes().as_slice());
	}
}

impl TopEncodeOutput for &mut Vec<u8> {
	fn set_slice_u8(self, bytes: &[u8]) {
		self.extend_from_slice(bytes);
	}
}
