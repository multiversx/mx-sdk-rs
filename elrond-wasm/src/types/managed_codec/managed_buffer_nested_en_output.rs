use elrond_codec::{NestedEncodeOutput, TryStaticCast};

use crate::{api::ManagedTypeApi, types::ManagedBuffer};

impl<M: ManagedTypeApi> NestedEncodeOutput for ManagedBuffer<M> {
    fn write(&mut self, bytes: &[u8]) {
        self.append_bytes(bytes);
    }

    fn push_specialized<T: TryStaticCast>(&mut self, value: &T) -> bool {
        if let Some(managed_buffer) = value.try_cast_ref::<ManagedBuffer<M>>() {
            let mb_len = managed_buffer.len() as u32;
            self.append_bytes(&mb_len.to_be_bytes()[..]);
            self.append(managed_buffer);
            true
        } else {
            false
        }
    }
}
