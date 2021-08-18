use super::StorageMapper;
use crate::api::{ErrorApi, ManagedTypeApi, StorageReadApi, StorageWriteApi};
use crate::storage::{storage_get, storage_set};
use crate::types::BoxedBytes;
use crate::types::TokenIdentifier;
use alloc::vec::Vec;
use elrond_codec::NestedEncode;

const MAPPING_SUFFIX: &[u8] = b".mapping";
const COUNTER_SUFFIX: &[u8] = b".counter";

const VALUE_ALREADY_SET_ERROR_MESSAGE: &[u8] =
    b"A value was already set for this token ID and token nonce";

const UNKNOWN_TOKEN_ID_ERROR_MESSAGE: &[u8] =
    b"Unknown token id. No attributes were set for this token ID";

const VALUE_NOT_PREVIOUSLY_SET_ERROR_MESSAGE: &[u8] =
    b"A value was not previously set fot this token ID and token nonce";

const COUNTER_OVERFLOW_ERROR_MESSAGE: &[u8] =
    b"Counter overflow. This module can hold evidence for maximum (u8::MAX - 1) different token IDs";

pub struct TokenAttributesMapper<SA>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
{
    api: SA,
    main_key: BoxedBytes,
}

impl<SA> StorageMapper<SA> for TokenAttributesMapper<SA>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
{
    fn new(api: SA, main_key: BoxedBytes) -> Self {
        TokenAttributesMapper { api, main_key }
    }
}

impl<SA> TokenAttributesMapper<SA>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
{
    pub fn set<T: elrond_codec::TopEncode + elrond_codec::TopDecode>(
        &self,
        token_id: &TokenIdentifier,
        token_nonce: u64,
        attributes: &T,
    ) {
        let has_mapping = !self.is_empty_mapping_value(token_id);

        let mapping = if has_mapping {
            self.get_mapping_value(token_id)
        } else {
            let mut counter = self.get_counter_value();
            if counter == u8::MAX {
                self.api.signal_error(COUNTER_OVERFLOW_ERROR_MESSAGE);
            }

            counter += 1;
            self.set_mapping_value(token_id, counter);
            self.set_counter_value(counter);
            counter
        };

        let has_value = !self.is_empty_token_attributes_value(mapping, token_nonce);
        if has_value {
            self.api.signal_error(VALUE_ALREADY_SET_ERROR_MESSAGE);
        }

        let mut value = Vec::<u8>::new();
        attributes.top_encode(&mut value).unwrap();
        self.set_token_attributes_value(mapping, token_nonce, value);
    }

    ///Use carefully. Update should be used mainly when backed up by the protocol.
    pub fn update<T: elrond_codec::TopEncode + elrond_codec::TopDecode>(
        &self,
        token_id: &TokenIdentifier,
        token_nonce: u64,
        attributes: &T,
    ) {
        let has_mapping = !self.is_empty_mapping_value(token_id);
        if !has_mapping {
            self.api.signal_error(UNKNOWN_TOKEN_ID_ERROR_MESSAGE);
        }

        let mapping = self.get_mapping_value(token_id);
        let has_value = !self.is_empty_token_attributes_value(mapping, token_nonce);
        if !has_value {
            self.api
                .signal_error(VALUE_NOT_PREVIOUSLY_SET_ERROR_MESSAGE);
        }

        let mut value = Vec::<u8>::new();
        attributes.top_encode(&mut value).unwrap();
        self.set_token_attributes_value(mapping, token_nonce, value)
    }

    pub fn clear(&self, token_id: &TokenIdentifier, token_nonce: u64) {
        let has_mapping = !self.is_empty_mapping_value(token_id);
        if !has_mapping {
            self.api.signal_error(UNKNOWN_TOKEN_ID_ERROR_MESSAGE);
        }

        let mapping = self.get_mapping_value(token_id);
        let has_value = !self.is_empty_token_attributes_value(mapping, token_nonce);
        if !has_value {
            self.api
                .signal_error(VALUE_NOT_PREVIOUSLY_SET_ERROR_MESSAGE);
        }

        self.clear_token_attributes_value(mapping, token_nonce);
    }

    pub fn is_empty(&self, token_id: &TokenIdentifier, token_nonce: u64) -> bool {
        let has_mapping = self.is_empty_mapping_value(token_id);
        let mapping = self.get_mapping_value(token_id);

        !has_mapping || self.is_empty_token_attributes_value(mapping, token_nonce)
    }

    pub fn get<T: elrond_codec::TopEncode + elrond_codec::TopDecode>(
        &self,
        token_id: &TokenIdentifier,
        token_nonce: u64,
    ) -> T {
        let has_mapping = !self.is_empty_mapping_value(token_id);
        if !has_mapping {
            self.api.signal_error(UNKNOWN_TOKEN_ID_ERROR_MESSAGE);
        }

        let mapping = self.get_mapping_value(token_id);
        let has_value = !self.is_empty_token_attributes_value(mapping, token_nonce);
        if !has_value {
            self.api
                .signal_error(VALUE_NOT_PREVIOUSLY_SET_ERROR_MESSAGE);
        }

        let value = self.get_token_attributes_value(mapping, token_nonce);
        T::top_decode(value).unwrap()
    }

    fn build_key_token_id_counter(&self) -> Vec<u8> {
        let mut bytes = self.main_key.as_slice().to_vec();
        bytes.extend_from_slice(COUNTER_SUFFIX);
        bytes
    }

    fn build_key_token_id_mapping(&self, token_id: &TokenIdentifier) -> Vec<u8> {
        let mut bytes = self.main_key.as_slice().to_vec();
        bytes.extend_from_slice(MAPPING_SUFFIX);
        if let Result::Err(encode_error) = token_id.dep_encode(&mut bytes) {
            self.api.signal_error(encode_error.message_bytes());
        }
        bytes
    }

    fn build_key_token_attr_value(&self, mapping: u8, token_nonce: u64) -> Vec<u8> {
        let mut bytes = self.main_key.as_slice().to_vec();
        if let Result::Err(encode_error) = mapping.dep_encode(&mut bytes) {
            self.api.signal_error(encode_error.message_bytes());
        }
        if let Result::Err(encode_error) = token_nonce.dep_encode(&mut bytes) {
            self.api.signal_error(encode_error.message_bytes());
        }
        bytes
    }

    fn get_counter_value(&self) -> u8 {
        storage_get(
            self.api.clone(),
            self.build_key_token_id_counter().as_slice(),
        )
    }

    fn set_counter_value(&self, value: u8) {
        storage_set(
            self.api.clone(),
            self.build_key_token_id_counter().as_slice(),
            &value,
        );
    }

    fn get_mapping_value(&self, token_id: &TokenIdentifier) -> u8 {
        storage_get(
            self.api.clone(),
            self.build_key_token_id_mapping(token_id).as_slice(),
        )
    }

    fn set_mapping_value(&self, token_id: &TokenIdentifier, value: u8) {
        storage_set(
            self.api.clone(),
            self.build_key_token_id_mapping(token_id).as_slice(),
            &value,
        );
    }

    fn is_empty_mapping_value(&self, token_id: &TokenIdentifier) -> bool {
        self.api
            .storage_load_len(self.build_key_token_id_mapping(token_id).as_slice())
            == 0
    }

    fn get_token_attributes_value(&self, mapping: u8, token_nonce: u64) -> Vec<u8> {
        storage_get(
            self.api.clone(),
            self.build_key_token_attr_value(mapping, token_nonce)
                .as_slice(),
        )
    }

    fn set_token_attributes_value(&self, mapping: u8, token_nonce: u64, value: Vec<u8>) {
        storage_set(
            self.api.clone(),
            self.build_key_token_attr_value(mapping, token_nonce)
                .as_slice(),
            &value,
        );
    }

    fn is_empty_token_attributes_value(&self, mapping: u8, token_nonce: u64) -> bool {
        self.api.storage_load_len(
            self.build_key_token_attr_value(mapping, token_nonce)
                .as_slice(),
        ) == 0
    }

    fn clear_token_attributes_value(&self, mapping: u8, token_nonce: u64) {
        storage_set(
            self.api.clone(),
            self.build_key_token_attr_value(mapping, token_nonce)
                .as_slice(),
            &BoxedBytes::empty(),
        );
    }
}
