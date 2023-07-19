use core::marker::PhantomData;

use crate::codec::{NestedDecode, NestedEncode, TopDecode, TopEncode};

use super::super::StorageMapper;
use crate::{
    api::{ErrorApiImpl, ManagedTypeApi, StorageMapperApi},
    storage::{storage_clear, storage_get, storage_get_len, storage_set, StorageKey},
    types::{ManagedType, TokenIdentifier},
};

const MAPPING_SUFFIX: &[u8] = b".mapping";
const COUNTER_SUFFIX: &[u8] = b".counter";
const ATTR_SUFFIX: &[u8] = b".attr";
const NONCE_SUFFIX: &[u8] = b".nonce";

const VALUE_ALREADY_SET_ERROR_MESSAGE: &[u8] = b"A value was already set";

const UNKNOWN_TOKEN_ID_ERROR_MESSAGE: &[u8] = b"Unknown token id";

const VALUE_NOT_PREVIOUSLY_SET_ERROR_MESSAGE: &[u8] = b"A value was not previously set";

const COUNTER_OVERFLOW_ERROR_MESSAGE: &[u8] =
    b"Counter overflow. This module can hold evidence for maximum u8::MAX different token IDs";

pub struct TokenAttributesMapper<SA>
where
    SA: StorageMapperApi,
{
    _phantom_api: PhantomData<SA>,
    base_key: StorageKey<SA>,
}

impl<SA> StorageMapper<SA> for TokenAttributesMapper<SA>
where
    SA: StorageMapperApi,
{
    fn new(base_key: StorageKey<SA>) -> Self {
        TokenAttributesMapper {
            _phantom_api: PhantomData,
            base_key,
        }
    }
}

impl<SA> TokenAttributesMapper<SA>
where
    SA: StorageMapperApi,
{
    pub fn set<T: TopEncode + TopDecode + NestedEncode + NestedDecode, M: ManagedTypeApi>(
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
                SA::error_api_impl().signal_error(COUNTER_OVERFLOW_ERROR_MESSAGE);
            }

            counter += 1;
            self.set_mapping_value(token_id, counter);
            self.set_counter_value(counter);
            counter
        };

        let has_value = self.has_token_attributes_value(mapping, token_nonce);
        if has_value {
            SA::error_api_impl().signal_error(VALUE_ALREADY_SET_ERROR_MESSAGE);
        }

        self.set_token_attributes_value(mapping, token_nonce, attributes);
        self.set_attributes_to_nonce_mapping(mapping, attributes, token_nonce);
    }

    ///Use carefully. Update should be used mainly when backed up by the protocol.
    pub fn update<T: TopEncode + TopDecode + NestedEncode + NestedDecode, M: ManagedTypeApi>(
        &self,
        token_id: &TokenIdentifier<M>,
        token_nonce: u64,
        attributes: &T,
    ) {
        let has_mapping = self.has_mapping_value(token_id);
        if !has_mapping {
            SA::error_api_impl().signal_error(UNKNOWN_TOKEN_ID_ERROR_MESSAGE);
        }

        let mapping = self.get_mapping_value(token_id);
        let has_value = self.has_token_attributes_value(mapping, token_nonce);
        if !has_value {
            SA::error_api_impl().signal_error(VALUE_NOT_PREVIOUSLY_SET_ERROR_MESSAGE);
        }

        let old_attr = self.get_token_attributes_value::<T>(mapping, token_nonce);
        self.clear_attributes_to_nonce_mapping(mapping, &old_attr);

        self.set_token_attributes_value(mapping, token_nonce, attributes);
        self.set_attributes_to_nonce_mapping(mapping, attributes, token_nonce);
    }

    pub fn clear<T: TopEncode + TopDecode + NestedEncode + NestedDecode, M: ManagedTypeApi>(
        &self,
        token_id: &TokenIdentifier<M>,
        token_nonce: u64,
    ) {
        let has_mapping = self.has_mapping_value(token_id);
        if !has_mapping {
            return;
        }

        let mapping = self.get_mapping_value(token_id);
        let has_value = self.has_token_attributes_value(mapping, token_nonce);
        if !has_value {
            return;
        }

        let attr: T = self.get_token_attributes_value(mapping, token_nonce);
        self.clear_token_attributes_value(mapping, token_nonce);
        self.clear_attributes_to_nonce_mapping(mapping, &attr);
    }

    pub fn has_attributes<M: ManagedTypeApi>(
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

    pub fn has_nonce<T: TopEncode + TopDecode + NestedEncode + NestedDecode, M: ManagedTypeApi>(
        &self,
        token_id: &TokenIdentifier<M>,
        attr: &T,
    ) -> bool {
        let has_mapping = self.has_mapping_value(token_id);
        if !has_mapping {
            return true;
        }

        let mapping = self.get_mapping_value(token_id);
        self.is_empty_attributes_to_nonce_mapping(mapping, attr)
    }

    pub fn get_attributes<
        T: TopEncode + TopDecode + NestedEncode + NestedDecode,
        M: ManagedTypeApi,
    >(
        &self,
        token_id: &TokenIdentifier<M>,
        token_nonce: u64,
    ) -> T {
        let has_mapping = self.has_mapping_value(token_id);
        if !has_mapping {
            SA::error_api_impl().signal_error(UNKNOWN_TOKEN_ID_ERROR_MESSAGE);
        }

        let mapping = self.get_mapping_value(token_id);
        let has_value = self.has_token_attributes_value(mapping, token_nonce);
        if !has_value {
            SA::error_api_impl().signal_error(VALUE_NOT_PREVIOUSLY_SET_ERROR_MESSAGE);
        }

        self.get_token_attributes_value(mapping, token_nonce)
    }

    pub fn get_nonce<T: TopEncode + TopDecode + NestedEncode + NestedDecode, M: ManagedTypeApi>(
        &self,
        token_id: &TokenIdentifier<M>,
        attr: &T,
    ) -> u64 {
        let has_mapping = self.has_mapping_value(token_id);
        if !has_mapping {
            SA::error_api_impl().signal_error(UNKNOWN_TOKEN_ID_ERROR_MESSAGE);
        }

        let mapping = self.get_mapping_value(token_id);
        let has_value = self.has_attr_to_nonce_mapping::<T>(mapping, attr);
        if !has_value {
            SA::error_api_impl().signal_error(VALUE_NOT_PREVIOUSLY_SET_ERROR_MESSAGE);
        }

        self.get_attributes_to_nonce_mapping(mapping, attr)
    }

    fn has_mapping_value<M: ManagedTypeApi>(&self, token_id: &TokenIdentifier<M>) -> bool {
        !self.is_empty_mapping_value(token_id)
    }

    fn has_token_attributes_value(&self, mapping: u8, token_nonce: u64) -> bool {
        !self.is_empty_token_attributes_value(mapping, token_nonce)
    }

    fn has_attr_to_nonce_mapping<T: TopEncode + TopDecode + NestedEncode + NestedDecode>(
        &self,
        mapping: u8,
        attr: &T,
    ) -> bool {
        !self.is_empty_attributes_to_nonce_mapping(mapping, attr)
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

    fn build_key_attr_to_nonce_mapping<T: TopEncode + TopDecode + NestedEncode + NestedDecode>(
        &self,
        mapping: u8,
        attr: &T,
    ) -> StorageKey<SA> {
        let mut key = self.base_key.clone();
        key.append_bytes(NONCE_SUFFIX);
        key.append_item(&mapping);
        key.append_item(attr);
        key
    }

    fn get_counter_value(&self) -> u8 {
        storage_get(self.build_key_token_id_counter().as_ref())
    }

    fn set_counter_value(&self, value: u8) {
        storage_set(self.build_key_token_id_counter().as_ref(), &value);
    }

    fn get_mapping_value<M: ManagedTypeApi>(&self, token_id: &TokenIdentifier<M>) -> u8 {
        storage_get(self.build_key_token_id_mapping(token_id).as_ref())
    }

    fn set_mapping_value<M: ManagedTypeApi>(&self, token_id: &TokenIdentifier<M>, value: u8) {
        storage_set(self.build_key_token_id_mapping(token_id).as_ref(), &value);
    }

    fn is_empty_mapping_value<M: ManagedTypeApi>(&self, token_id: &TokenIdentifier<M>) -> bool {
        storage_get_len(self.build_key_token_id_mapping(token_id).as_ref()) == 0
    }

    fn get_attributes_to_nonce_mapping<T: TopEncode + TopDecode + NestedEncode + NestedDecode>(
        &self,
        mapping: u8,
        attr: &T,
    ) -> u64 {
        storage_get(self.build_key_attr_to_nonce_mapping(mapping, attr).as_ref())
    }

    fn set_attributes_to_nonce_mapping<T: TopEncode + TopDecode + NestedEncode + NestedDecode>(
        &self,
        mapping: u8,
        attr: &T,
        token_nonce: u64,
    ) {
        storage_set(
            self.build_key_attr_to_nonce_mapping(mapping, attr).as_ref(),
            &token_nonce,
        );
    }

    fn is_empty_attributes_to_nonce_mapping<
        T: TopEncode + TopDecode + NestedEncode + NestedDecode,
    >(
        &self,
        mapping: u8,
        attr: &T,
    ) -> bool {
        storage_get_len(self.build_key_attr_to_nonce_mapping(mapping, attr).as_ref()) == 0
    }

    fn clear_attributes_to_nonce_mapping<T: TopEncode + TopDecode + NestedEncode + NestedDecode>(
        &self,
        mapping: u8,
        attr: &T,
    ) {
        storage_clear(self.build_key_attr_to_nonce_mapping(mapping, attr).as_ref());
    }

    fn get_token_attributes_value<T: TopEncode + TopDecode + NestedEncode + NestedDecode>(
        &self,
        mapping: u8,
        token_nonce: u64,
    ) -> T {
        storage_get(
            self.build_key_token_attr_value(mapping, token_nonce)
                .as_ref(),
        )
    }

    fn set_token_attributes_value<T: TopEncode + TopDecode + NestedEncode + NestedDecode>(
        &self,
        mapping: u8,
        token_nonce: u64,
        value: &T,
    ) {
        storage_set(
            self.build_key_token_attr_value(mapping, token_nonce)
                .as_ref(),
            value,
        );
    }

    fn is_empty_token_attributes_value(&self, mapping: u8, token_nonce: u64) -> bool {
        storage_get_len(
            self.build_key_token_attr_value(mapping, token_nonce)
                .as_ref(),
        ) == 0
    }

    fn clear_token_attributes_value(&self, mapping: u8, token_nonce: u64) {
        storage_clear(
            self.build_key_token_attr_value(mapping, token_nonce)
                .as_ref(),
        );
    }
}
