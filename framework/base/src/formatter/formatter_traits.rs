use crate::codec::TopEncode;

use crate::{
    api::ManagedTypeApi, contract_base::ExitCodecErrorHandler, err_msg, types::ManagedBuffer,
};

pub const HEX_VALUE_PREFIX: &[u8] = b"0x";
pub const BINARY_VALUE_PREFIX: &[u8] = b"0b";

pub trait FormatByteReceiver {
    type Api: ManagedTypeApi;

    fn append_bytes(&mut self, bytes: &[u8]);

    fn append_managed_buffer(&mut self, item: &ManagedBuffer<Self::Api>);

    fn append_managed_buffer_lower_hex(&mut self, item: &ManagedBuffer<Self::Api>);

    fn append_managed_buffer_binary(&mut self, item: &ManagedBuffer<Self::Api>);
}

pub trait FormatBuffer: Default {
    fn append_ascii(&mut self, ascii: &[u8]);

    fn append_display<T: SCDisplay>(&mut self, item: &T);

    fn append_lower_hex<T: SCLowerHex>(&mut self, item: &T);

    fn append_binary<T: SCBinary>(&mut self, item: &T);

    fn append_codec<T: SCCodec>(&mut self, item: &T);
}

#[derive(Default)]
pub struct FormatBufferIgnore;

impl FormatBuffer for FormatBufferIgnore {
    #[inline]
    fn append_ascii(&mut self, _ascii: &[u8]) {}

    #[inline]
    fn append_display<T: SCDisplay>(&mut self, _item: &T) {}

    #[inline]
    fn append_lower_hex<T: SCLowerHex>(&mut self, _item: &T) {}

    #[inline]
    fn append_binary<T: SCBinary>(&mut self, _item: &T) {}

    #[inline]
    fn append_codec<T: SCCodec>(&mut self, _item: &T) {}
}

pub trait SCDisplay {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F);
}

impl<T: SCDisplay> SCDisplay for &T {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        SCDisplay::fmt(*self, f)
    }
}

pub trait SCLowerHex {
    fn fmt<F>(&self, f: &mut F)
    where
        F: FormatByteReceiver;
}

impl<T: SCLowerHex> SCLowerHex for &T {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        SCLowerHex::fmt(*self, f)
    }
}

pub trait SCBinary {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F);
}

impl<T: SCBinary> SCBinary for &T {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        SCBinary::fmt(*self, f)
    }
}

pub trait SCCodec {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F);
}

impl<T: TopEncode> SCCodec for T {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        let mut encoded = ManagedBuffer::<F::Api>::new();
        let Ok(()) = self.top_encode_or_handle_err(
            &mut encoded,
            ExitCodecErrorHandler::<F::Api>::from(err_msg::FORMATTER_ENCODE_ERROR),
        );
        SCLowerHex::fmt(&encoded, f);
    }
}
