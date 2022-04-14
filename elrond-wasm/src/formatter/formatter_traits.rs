use crate::{api::ManagedTypeApi, types::ManagedBuffer};

pub const HEX_VALUE_PREFIX: &[u8] = b"0x";
pub const BINARY_VALUE_PREFIX: &[u8] = b"0b";

pub trait FormatByteReceiver {
    fn append_bytes(&mut self, bytes: &[u8]);

    fn append_managed_buffer<M: ManagedTypeApi>(&mut self, item: &ManagedBuffer<M>);

    fn append_managed_buffer_lower_hex<M: ManagedTypeApi>(&mut self, item: &ManagedBuffer<M>);

    fn append_managed_buffer_binary<M: ManagedTypeApi>(&mut self, item: &ManagedBuffer<M>);
}

pub trait FormatBuffer: Default {
    fn append_ascii(&mut self, ascii: &[u8]);

    fn append_display<T: SCDisplay>(&mut self, item: &T);

    fn append_lower_hex<T: SCLowerHex>(&mut self, item: &T);

    fn append_bytes<T: SCBinary>(&mut self, item: &T);

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
    fn append_bytes<T: SCBinary>(&mut self, _item: &T) {}

    #[inline]
    fn append_codec<T: SCCodec>(&mut self, _item: &T) {}
}

pub trait SCDisplay {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F);
}

pub trait SCLowerHex {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F);
}

pub trait SCBinary {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F);
}

pub trait SCCodec {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F);
}
