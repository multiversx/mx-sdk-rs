use alloc::vec::Vec;
use crate::output::NestedOutputBuffer;
use crate::num_conv::encode_number_to_output;

pub trait TopEncodeBuffer: NestedOutputBuffer {
    fn save_buffer(self);
}

pub trait TopEncodeOutput<B: TopEncodeBuffer>: Sized {
    fn set_slice_u8(self, bytes: &[u8]);

    fn into_output_buffer(self) -> B;

    fn set_u64(self, value: u64) {
        let mut buffer = self.into_output_buffer();
        encode_number_to_output(&mut buffer, value, 64, false, true);
        buffer.save_buffer();
    }

    fn set_i64(self, value: i64) {
        let mut buffer = self.into_output_buffer();
        encode_number_to_output(&mut buffer, value as u64, 64, true, true);
        buffer.save_buffer();
    }
}

impl<'o> TopEncodeBuffer for &'o mut Vec<u8> {
    fn save_buffer(self){}
} 

impl<'o> TopEncodeOutput<&'o mut Vec<u8>> for &'o mut Vec<u8> {
	fn set_slice_u8(self, bytes: &[u8]) {
        self.extend_from_slice(bytes);
    }

    fn into_output_buffer(self) -> &'o mut Vec<u8> {
        self
    }
}
