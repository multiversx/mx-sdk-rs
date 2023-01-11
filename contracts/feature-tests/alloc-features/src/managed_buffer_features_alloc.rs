multiversx_sc::imports!();

/// ManagedBuffers interacting with the heap.
#[multiversx_sc::module]
pub trait ManagedBufferFeatures {
    #[endpoint]
    fn mbuffer_from_slice(&self, slice: &[u8]) -> ManagedBuffer {
        ManagedBuffer::from(slice)
    }

    #[endpoint]
    fn mbuffer_from_boxed_bytes(&self, boxed_bytes: BoxedBytes) -> ManagedBuffer {
        ManagedBuffer::from(boxed_bytes)
    }

    #[endpoint]
    fn mbuffer_overwrite(&self, mb1: ManagedBuffer, bytes: &[u8]) -> ManagedBuffer {
        let mut result = mb1;
        result.overwrite(bytes);
        result
    }

    #[endpoint]
    fn mbuffer_append_bytes(&self, mb: ManagedBuffer, bytes: &[u8]) -> ManagedBuffer {
        let mut result = mb;
        result.append_bytes(bytes);
        result
    }

    #[endpoint]
    fn mbuffer_load_slice(
        &self,
        mb: ManagedBuffer,
        starting_position: usize,
        slice_len: usize,
    ) -> OptionalValue<BoxedBytes> {
        let mut result = BoxedBytes::zeros(slice_len);
        if mb
            .load_slice(starting_position, result.as_mut_slice())
            .is_ok()
        {
            OptionalValue::Some(result)
        } else {
            OptionalValue::None
        }
    }

    #[endpoint]
    fn mbuffer_set_slice(&self, mb: ManagedBuffer, index: usize, item: &[u8]) -> ManagedBuffer {
        let mut result = mb;
        if result.set_slice(index, item).is_err() {
            sc_panic!("index out of bounds");
        }
        result
    }

    #[endpoint]
    fn managed_address_from(&self, bytes: &[u8; 32]) -> ManagedAddress {
        ManagedAddress::from(bytes)
    }
}
