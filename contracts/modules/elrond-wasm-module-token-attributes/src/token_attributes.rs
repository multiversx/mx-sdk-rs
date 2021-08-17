#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

const VALUE_ALREADY_SET_ERROR_MESSAGE: &[u8] =
    b"A value was already set for this token ID and token nonce";

const UNKNOWN_TOKEN_ID_ERROR_MESSAGE: &[u8] =
    b"Unknown token id. No attributes were set for this token ID";

const VALUE_NOT_PREVIOUSLY_SET_ERROR_MESSAGE: &[u8] =
    b"A value was not previously set fot this token ID and token nonce";

const COUNTER_OVERFLOW_ERROR_MESSAGE: &[u8] =
    b"Counter overflow. This module can hold evidence for maximum (u8::MAX - 1) different token IDs";

#[elrond_wasm::module]
pub trait TokenAttributesModule {
    fn token_attributes_set<T: elrond_codec::TopEncode + elrond_codec::TopDecode>(
        &self,
        token_id: &TokenIdentifier,
        token_nonce: u64,
        attributes: &T,
    ) -> SCResult<()> {
        let has_mapping = !self.token_attributes_mapping(token_id).is_empty();

        let mapping = if has_mapping {
            self.token_attributes_mapping(token_id).get()
        } else {
            let mut counter = self.token_attributes_counter().get();
            require!(counter < u8::MAX, COUNTER_OVERFLOW_ERROR_MESSAGE);

            counter += 1;
            self.token_attributes_mapping(token_id).set(&counter);
            self.token_attributes_counter().set(&counter);
            counter
        };

        let has_value = !self.token_attributes(mapping, token_nonce).is_empty();
        require!(!has_value, VALUE_ALREADY_SET_ERROR_MESSAGE);

        let mut token_attributes_serialized = Vec::<u8>::new();
        attributes.top_encode(&mut token_attributes_serialized)?;
        self.token_attributes(mapping, token_nonce)
            .set(&token_attributes_serialized);

        Ok(())
    }

    ///Use carefully. Update should be used mainly when backed up by the protocol.
    fn token_attributes_update<T: elrond_codec::TopEncode + elrond_codec::TopDecode>(
        &self,
        token_id: &TokenIdentifier,
        token_nonce: u64,
        attributes: &T,
    ) -> SCResult<()> {
        let has_mapping = !self.token_attributes_mapping(token_id).is_empty();
        require!(has_mapping, UNKNOWN_TOKEN_ID_ERROR_MESSAGE);

        let mapping = self.token_attributes_mapping(token_id).get();
        let has_value = !self.token_attributes(mapping, token_nonce).is_empty();
        require!(has_value, VALUE_NOT_PREVIOUSLY_SET_ERROR_MESSAGE);

        let mut token_attributes_serialized = Vec::<u8>::new();
        attributes.top_encode(&mut token_attributes_serialized)?;
        self.token_attributes(mapping, token_nonce)
            .set(&token_attributes_serialized);

        Ok(())
    }

    fn token_attributes_clear(&self, token_id: &TokenIdentifier, token_nonce: u64) -> SCResult<()> {
        let has_mapping = !self.token_attributes_mapping(token_id).is_empty();
        require!(has_mapping, UNKNOWN_TOKEN_ID_ERROR_MESSAGE);

        let mapping = self.token_attributes_mapping(token_id).get();
        let has_value = !self.token_attributes(mapping, token_nonce).is_empty();
        require!(has_value, VALUE_NOT_PREVIOUSLY_SET_ERROR_MESSAGE);

        self.token_attributes(mapping, token_nonce).clear();

        Ok(())
    }

    fn token_attributes_is_empty(&self, token_id: &TokenIdentifier, token_nonce: u64) -> bool {
        let has_mapping = !self.token_attributes_mapping(token_id).is_empty();
        let mapping = self.token_attributes_mapping(token_id).get();

        !has_mapping || self.token_attributes(mapping, token_nonce).is_empty()
    }

    fn token_attributes_get<T: elrond_codec::TopEncode + elrond_codec::TopDecode>(
        &self,
        token_id: &TokenIdentifier,
        token_nonce: u64,
    ) -> SCResult<T> {
        let has_mapping = !self.token_attributes_mapping(token_id).is_empty();
        require!(has_mapping, UNKNOWN_TOKEN_ID_ERROR_MESSAGE);

        let mapping = self.token_attributes_mapping(token_id).get();
        let has_value = !self.token_attributes(mapping, token_nonce).is_empty();
        require!(has_value, VALUE_NOT_PREVIOUSLY_SET_ERROR_MESSAGE);

        let token_attributes_serialized = self.token_attributes(mapping, token_nonce).get();
        SCResult::from(T::top_decode(token_attributes_serialized))
    }

    #[storage_mapper("TokenAttributes:counter")]
    fn token_attributes_counter(&self) -> SingleValueMapper<Self::Storage, u8>;

    #[storage_mapper("TA")]
    fn token_attributes(
        &self,
        mapping: u8,
        nonce: u64,
    ) -> SingleValueMapper<Self::Storage, Vec<u8>>;

    #[storage_mapper("TokenAttributes:mapping")]
    fn token_attributes_mapping(
        &self,
        token_id: &TokenIdentifier,
    ) -> SingleValueMapper<Self::Storage, u8>;
}
