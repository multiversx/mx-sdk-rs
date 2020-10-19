use alloc::vec::Vec;
use crate::nested_ser_output::OutputBuffer;
use crate::num_conv::encode_number_to_output;

/// Specifies objects that can receive the result of a TopEncode computation.
/// 
/// It also models a buffer, where results can be accumulated,
/// in principle from NestedEncode performed on nested items.
/// 
/// All methods (other than buffer_ref, which is just a reference to the underlying buffer)
/// consume the object, so they can only be called once.
/// 
/// The trait is used in 3 scenarios:
/// - SC results
/// - `#[storage_set(...)]`
/// - Serialize async call.
pub trait TopEncodeOutput<'o, B: OutputBuffer>: Sized {
    fn set_slice_u8(self, bytes: &[u8]);

    fn buffer_ref<'r>(&'r mut self) -> &'r mut B
    where 'o: 'r;

    fn flush_buffer(self);

    fn set_u64(mut self, value: u64) {
        let buffer = self.buffer_ref();
        encode_number_to_output(buffer, value, 64, false, true);
        self.flush_buffer();
    }

    fn set_i64(mut self, value: i64) {
        let buffer = self.buffer_ref();
        encode_number_to_output(buffer, value as u64, 64, true, true);
        self.flush_buffer();
    }
}

impl<'o> TopEncodeOutput<'o, Vec<u8>> for &'o mut Vec<u8> {
	fn set_slice_u8(self, bytes: &[u8]) {
        self.extend_from_slice(bytes);
    }

    fn buffer_ref<'r>(&'r mut self) -> &'r mut Vec<u8>
    where 'o: 'r
    {
        self
    }

    fn flush_buffer(self) {}
}
