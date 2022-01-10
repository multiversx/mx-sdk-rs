elrond_wasm::imports!();

#[elrond_wasm::module]
pub trait ManagedBufferFeatures {
    #[endpoint]
    fn mbuffer_new(&self) -> ManagedBuffer {
        ManagedBuffer::new()
    }

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
    fn mbuffer_concat_1(&self, mb1: ManagedBuffer, mb2: ManagedBuffer) -> ManagedBuffer {
        let mut result = mb1;
        result.append(&mb2);
        result
    }

    #[endpoint]
    fn mbuffer_concat_2(&self, mb: ManagedBuffer, bytes: &[u8]) -> ManagedBuffer {
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
    ) -> OptionalResult<BoxedBytes> {
        let mut result = BoxedBytes::zeros(slice_len);
        if mb
            .load_slice(starting_position, result.as_mut_slice())
            .is_ok()
        {
            OptionalResult::Some(result)
        } else {
            OptionalResult::None
        }
    }

    #[endpoint]
    fn mbuffer_set_slice(
        &self,
        mb: ManagedBuffer,
        index: usize,
        item: &[u8],
    ) -> SCResult<ManagedBuffer> {
        let mut result = mb;
        result
            .set_slice(index, item)
            .map_err(|_| "index out of bounds")?;
        Ok(result)
    }

    #[endpoint]
    fn mbuffer_copy_slice(
        &self,
        mb: ManagedBuffer,
        starting_position: usize,
        slice_len: usize,
    ) -> OptionalResult<ManagedBuffer> {
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
    fn managed_address_from(&self, bytes: &[u8; 32]) -> ManagedAddress {
        ManagedAddress::from(bytes)
    }

    #[endpoint]
    fn managed_address_eq(&self, mb1: ManagedAddress, mb2: ManagedAddress) -> bool {
        mb1 == mb2
    }

    #[endpoint]
    fn dynamic_message(&self, bytes: ManagedBuffer) {
        signal_error!("Got this buffer: {:x}. I don't like it, ERROR!", bytes);
    }

    #[payable("*")]
    #[endpoint]
    fn dynamic_message_multiple(
        &self,
        #[payment_token] token_id: TokenIdentifier,
        #[payment_nonce] nonce: u64,
        #[payment_amount] amount: BigUint,
    ) {
        signal_error!(
            "Got token {:x}, with nonce {:x}, amount {:x}. I prefer EGLD. ERROR!",
            token_id,
            nonce,
            amount
        );
    }

    #[payable("*")]
    #[endpoint]
    fn dynamic_message_ascii(
        &self,
        #[payment_token] token_id: TokenIdentifier,
        #[payment_nonce] nonce: u64,
        #[payment_amount] amount: BigUint,
    ) {
        signal_error!(
            "Got token {}, with nonce {:x}, amount {:x}. I prefer EGLD. ERROR!",
            token_id,
            nonce,
            amount
        );
    }
}
