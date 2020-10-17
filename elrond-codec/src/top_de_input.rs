use alloc::boxed::Box;
use crate::num_conv::bytes_to_number;

pub trait TopDecodeInput: Sized {
    fn byte_len(&self) -> usize;

    fn into_boxed_slice(self) -> Box<[u8]>;

    fn into_u64(self) -> u64 {
        bytes_to_number(&*self.into_boxed_slice(), false)
    }

    fn into_i64(self) -> i64 {
        bytes_to_number(&*self.into_boxed_slice(), true) as i64
    }
}

impl TopDecodeInput for Box<[u8]> {
    fn byte_len(&self) -> usize {
        self.len()
    }

    fn into_boxed_slice(self) -> Box<[u8]> {
        self
    }
}

impl<'a> TopDecodeInput for &'a [u8] {
    fn byte_len(&self) -> usize {
        self.len()
    }

    fn into_boxed_slice(self) -> Box<[u8]> {
        Box::from(self)
    }
}
