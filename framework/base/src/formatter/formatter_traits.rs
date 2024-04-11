use crate::codec::TopEncode;

use crate::{
    api::ManagedTypeApi, contract_base::ExitCodecErrorHandler, err_msg, types::ManagedBuffer,
};

pub const HEX_VALUE_PREFIX: &[u8] = b"0x";
pub const BINARY_VALUE_PREFIX: &[u8] = b"0b";

pub trait FormatByteReceiver<'a> {
    type Api: ManagedTypeApi<'a>;

    fn append_bytes(&mut self, bytes: &[u8]);

    fn append_managed_buffer(&mut self, item: &ManagedBuffer<Self::Api>);

    fn append_managed_buffer_lower_hex(&mut self, item: &ManagedBuffer<Self::Api>);

    fn append_managed_buffer_binary(&mut self, item: &ManagedBuffer<Self::Api>);
}

pub trait FormatBuffer<'a>: Default {
    fn append_ascii(&mut self, ascii: &[u8]);

    fn append_display<T: SCDisplay<'a>>(&mut self, item: &T);

    fn append_lower_hex<T: SCLowerHex<'a>>(&mut self, item: &T);

    fn append_binary<T: SCBinary<'a>>(&mut self, item: &T);

    fn append_codec<T: SCCodec<'a>>(&mut self, item: &T);
}

#[derive(Default)]
pub struct FormatBufferIgnore;

impl<'a> FormatBuffer<'a> for FormatBufferIgnore {
    #[inline]
    fn append_ascii(&mut self, _ascii: &[u8]) {}

    #[inline]
    fn append_display<T: SCDisplay<'a>>(&mut self, _item: &T) {}

    #[inline]
    fn append_lower_hex<T: SCLowerHex<'a>>(&mut self, _item: &T) {}

    #[inline]
    fn append_binary<T: SCBinary<'a>>(&mut self, _item: &T) {}

    #[inline]
    fn append_codec<T: SCCodec<'a>>(&mut self, _item: &T) {}
}

pub trait SCDisplay<'a> {
    fn fmt<F: FormatByteReceiver<'a>>(&self, f: &mut F);
}

impl<'a, T: SCDisplay<'a>> SCDisplay<'a> for &T {
    fn fmt<F: FormatByteReceiver<'a>>(&self, f: &mut F) {
        SCDisplay::fmt(*self, f)
    }
}

pub trait SCLowerHex<'a> {
    fn fmt<F>(&self, f: &mut F)
    where
        F: FormatByteReceiver<'a>;
}

impl<'a, T: SCLowerHex<'a>> SCLowerHex<'a> for &T {
    fn fmt<F: FormatByteReceiver<'a>>(&self, f: &mut F) {
        SCLowerHex::fmt(*self, f)
    }
}

pub trait SCBinary<'a> {
    fn fmt<F: FormatByteReceiver<'a>>(&self, f: &mut F);
}

impl<'a, T: SCBinary<'a>> SCBinary<'a> for &T {
    fn fmt<F: FormatByteReceiver<'a>>(&self, f: &mut F) {
        SCBinary::fmt(*self, f)
    }
}

pub trait SCCodec<'a> {
    fn fmt<F: FormatByteReceiver<'a>>(&self, f: &mut F);
}

impl<'a, T: TopEncode> SCCodec<'a> for T {
    fn fmt<F: FormatByteReceiver<'a>>(&self, f: &mut F) {
        let mut encoded = ManagedBuffer::<F::Api>::new();
        let Ok(()) = self.top_encode_or_handle_err(
            &mut encoded,
            ExitCodecErrorHandler::<F::Api>::from(err_msg::FORMATTER_ENCODE_ERROR),
        );
        SCLowerHex::fmt(&encoded, f);
    }
}
