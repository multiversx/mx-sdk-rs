use alloc::vec::Vec;
use crate::num_conv::top_encode_number_to_output;

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
}

impl TopEncodeOutput for &mut Vec<u8> {
	fn set_slice_u8(self, bytes: &[u8]) {
        self.extend_from_slice(bytes);
    }
}
