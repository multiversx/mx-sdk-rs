use alloc::vec::Vec;
use crate::output::NestedOutputBuffer;
use crate::num_conv::encode_number_to_output;

pub trait TopEncodeOutput<B: NestedOutputBuffer>: Sized {
    fn set_slice_u8(self, bytes: &[u8]);

    fn into_output_buffer(self) -> B;

    fn set_u64(self, value: u64);

    fn set_i64(self, value: i64);
}

impl<'o> TopEncodeOutput<&'o mut Vec<u8>> for &'o mut Vec<u8> {
	fn set_slice_u8(self, bytes: &[u8]) {
        self.extend_from_slice(bytes);
    }

    fn into_output_buffer(self) -> &'o mut Vec<u8> {
        self
    }

    fn set_u64(self, value: u64) {
        encode_number_to_output(self, value, 64, false, true);
    }

    fn set_i64(self, value: i64) {
        encode_number_to_output(self, value as u64, 64, true, true);
    }
}
