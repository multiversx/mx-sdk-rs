multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait ManagedBufferFeatures {
    #[endpoint]
    fn mbuffer_new(&self) -> ManagedBuffer {
        ManagedBuffer::new()
    }

    #[endpoint]
    fn mbuffer_concat(&self, mb1: ManagedBuffer, mb2: ManagedBuffer) -> ManagedBuffer {
        let mut result = mb1;
        result.append(&mb2);
        result
    }

    #[endpoint]
    fn mbuffer_copy_slice(
        &self,
        mb: ManagedBuffer,
        starting_position: usize,
        slice_len: usize,
    ) -> OptionalValue<ManagedBuffer> {
        mb.copy_slice(starting_position, slice_len).into()
    }

    #[endpoint]
    fn mbuffer_set_random(&self, nr_bytes: usize) -> ManagedBuffer {
        ManagedBuffer::new_random(nr_bytes)
    }

    #[endpoint]
    fn mbuffer_eq(&self, mb1: ManagedBuffer, mb2: ManagedBuffer) -> bool {
        mb1 == mb2
    }

    #[endpoint]
    fn managed_address_zero(&self) -> ManagedAddress {
        ManagedAddress::zero()
    }

    #[endpoint]
    fn managed_address_eq(&self, mb1: ManagedAddress, mb2: ManagedAddress) -> bool {
        mb1 == mb2
    }
}
