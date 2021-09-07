use elrond_codec::{TopDecode, TopEncode};

use super::StorageMapper;
use crate::api::{ErrorApi, ManagedTypeApi, StorageReadApi, StorageWriteApi};
use crate::storage::{storage_clear, storage_get, storage_get_len, storage_set, StorageKey};
use crate::types::TokenIdentifier;

const MAPPING_SUFFIX: &[u8] = b".mapping";
const COUNTER_SUFFIX: &[u8] = b".counter";
const ATTR_SUFFIX: &[u8] = b".attr";

const VALUE_ALREADY_SET_ERROR_MESSAGE: &[u8] =
    b"A value was already set for this token ID and token nonce";

const UNKNOWN_TOKEN_ID_ERROR_MESSAGE: &[u8] =
    b"Unknown token id. No attributes were set for this token ID";

const VALUE_NOT_PREVIOUSLY_SET_ERROR_MESSAGE: &[u8] =
    b"A value was not previously set fot this token ID and token nonce";

const COUNTER_OVERFLOW_ERROR_MESSAGE: &[u8] =
    b"Counter overflow. This module can hold evidence for maximum u8::MAX different token IDs";

pub struct TokenAttributesMapper<SA>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
{
    api: SA,
    base_key: StorageKey<SA>,
}

impl<SA> StorageMapper<SA> for TokenAttributesMapper<SA>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
{
    fn new(api: SA, base_key: StorageKey<SA>) -> Self {
        TokenAttributesMapper { api, base_key }
    }
}

impl<SA> TokenAttributesMapper<SA>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
{
    pub fn set<T: TopEncode + TopDecode, M: ManagedTypeApi>(
        &self,
        token_id: &TokenIdentifier<M>,
        token_nonce: u64,
        attributes: &T,
    ) {
        let has_mapping = self.has_mapping_value(token_id);

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

        let has_value = self.has_token_attributes_value(mapping, token_nonce);
        if has_value {
            self.api.signal_error(VALUE_ALREADY_SET_ERROR_MESSAGE);
        }

        self.set_token_attributes_value(mapping, token_nonce, attributes);
    }

    ///Use carefully. Update should be used mainly when backed up by the protocol.
    pub fn update<T: TopEncode + TopDecode, M: ManagedTypeApi>(
        &self,
        token_id: &TokenIdentifier<M>,
        token_nonce: u64,
        attributes: &T,
    ) {
        let has_mapping = self.has_mapping_value(token_id);
        if !has_mapping {
            self.api.signal_error(UNKNOWN_TOKEN_ID_ERROR_MESSAGE);
        }

        let mapping = self.get_mapping_value(token_id);
        let has_value = self.has_token_attributes_value(mapping, token_nonce);
        if !has_value {
            self.api
                .signal_error(VALUE_NOT_PREVIOUSLY_SET_ERROR_MESSAGE);
        }

        self.set_token_attributes_value(mapping, token_nonce, attributes);
    }

    pub fn clear<M: ManagedTypeApi>(&self, token_id: &TokenIdentifier<M>, token_nonce: u64) {
        let has_mapping = self.has_mapping_value(token_id);
        if !has_mapping {
            return;
        }

        let mapping = self.get_mapping_value(token_id);
        let has_value = self.has_token_attributes_value(mapping, token_nonce);
        if !has_value {
            return;
        }

        self.clear_token_attributes_value(mapping, token_nonce);
    }

    pub fn is_empty<M: ManagedTypeApi>(
        &self,
        token_id: &TokenIdentifier<M>,
        token_nonce: u64,
    ) -> bool {
        let has_mapping = self.has_mapping_value(token_id);
        if !has_mapping {
            return true;
        }

        let mapping = self.get_mapping_value(token_id);
        self.is_empty_token_attributes_value(mapping, token_nonce)
    }

    pub fn get<T: TopEncode + TopDecode, M: ManagedTypeApi>(
        &self,
        token_id: &TokenIdentifier<M>,
        token_nonce: u64,
    ) -> T {
        let has_mapping = self.has_mapping_value(token_id);
        if !has_mapping {
            self.api.signal_error(UNKNOWN_TOKEN_ID_ERROR_MESSAGE);
        }

        let mapping = self.get_mapping_value(token_id);
        let has_value = self.has_token_attributes_value(mapping, token_nonce);
        if !has_value {
            self.api
                .signal_error(VALUE_NOT_PREVIOUSLY_SET_ERROR_MESSAGE);
        }

        self.get_token_attributes_value(mapping, token_nonce)
    }

    fn has_mapping_value<M: ManagedTypeApi>(&self, token_id: &TokenIdentifier<M>) -> bool {
        !self.is_empty_mapping_value(token_id)
    }

    fn has_token_attributes_value(&self, mapping: u8, token_nonce: u64) -> bool {
        !self.is_empty_token_attributes_value(mapping, token_nonce)
    }

    fn build_key_token_id_counter(&self) -> StorageKey<SA> {
        let mut key = self.base_key.clone();
        key.append_bytes(COUNTER_SUFFIX);
        key
    }

    fn build_key_token_id_mapping<M: ManagedTypeApi>(
        &self,
        token_id: &TokenIdentifier<M>,
    ) -> StorageKey<SA> {
        let mut key = self.base_key.clone();
        key.append_bytes(MAPPING_SUFFIX);
        key.append_item(token_id);
        key
    }

    fn build_key_token_attr_value(&self, mapping: u8, token_nonce: u64) -> StorageKey<SA> {
        let mut key = self.base_key.clone();
        key.append_bytes(ATTR_SUFFIX);
        key.append_item(&mapping);
        key.append_item(&token_nonce);
        key
    }

    fn get_counter_value(&self) -> u8 {
        storage_get(self.api.clone(), &self.build_key_token_id_counter())
    }

    fn set_counter_value(&self, value: u8) {
        storage_set(self.api.clone(), &self.build_key_token_id_counter(), &value);
    }

    fn get_mapping_value<M: ManagedTypeApi>(&self, token_id: &TokenIdentifier<M>) -> u8 {
        storage_get(self.api.clone(), &self.build_key_token_id_mapping(token_id))
    }

    fn set_mapping_value<M: ManagedTypeApi>(&self, token_id: &TokenIdentifier<M>, value: u8) {
        storage_set(
            self.api.clone(),
            &self.build_key_token_id_mapping(token_id),
            &value,
        );
    }

    fn is_empty_mapping_value<M: ManagedTypeApi>(&self, token_id: &TokenIdentifier<M>) -> bool {
        storage_get_len(self.api.clone(), &self.build_key_token_id_mapping(token_id)) == 0
    }

    fn get_token_attributes_value<T: TopEncode + TopDecode>(
        &self,
        mapping: u8,
        token_nonce: u64,
    ) -> T {
        storage_get(
            self.api.clone(),
            &self.build_key_token_attr_value(mapping, token_nonce),
        )
    }

    fn set_token_attributes_value<T: TopEncode + TopDecode>(
        &self,
        mapping: u8,
        token_nonce: u64,
        value: &T,
    ) {
        storage_set(
            self.api.clone(),
            &self.build_key_token_attr_value(mapping, token_nonce),
            value,
        );
    }

    fn is_empty_token_attributes_value(&self, mapping: u8, token_nonce: u64) -> bool {
        storage_get_len(
            self.api.clone(),
            &self.build_key_token_attr_value(mapping, token_nonce),
        ) == 0
    }

    fn clear_token_attributes_value(&self, mapping: u8, token_nonce: u64) {
        storage_clear(
            self.api.clone(),
            &self.build_key_token_attr_value(mapping, token_nonce),
        );
    }
}
