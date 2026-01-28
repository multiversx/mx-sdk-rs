use core::marker::PhantomData;

use crate::{
    codec::{NestedDecode, NestedEncode, TopDecode, TopEncode},
    storage::mappers::{
        StorageMapperFromAddress,
        source::{CurrentStorage, StorageAddress},
    },
    types::ManagedAddress,
};

use super::super::StorageMapper;
use crate::{
    api::{ErrorApiImpl, ManagedTypeApi, StorageMapperApi},
    storage::{StorageKey, storage_clear, storage_get, storage_get_len, storage_set},
    types::{EsdtTokenIdentifier, ManagedType},
};

const MAPPING_SUFFIX: &str = ".mapping";
const COUNTER_SUFFIX: &str = ".counter";
const ATTR_SUFFIX: &str = ".attr";
const NONCE_SUFFIX: &str = ".nonce";

const VALUE_ALREADY_SET_ERROR_MESSAGE: &str = "A value was already set";

const UNKNOWN_TOKEN_ID_ERROR_MESSAGE: &str = "Unknown token id";

const VALUE_NOT_PREVIOUSLY_SET_ERROR_MESSAGE: &str = "A value was not previously set";

const COUNTER_OVERFLOW_ERROR_MESSAGE: &str =
    "Counter overflow. This module can hold evidence for maximum u8::MAX different token IDs";

/// Specialized mapper for managing NFT/SFT token attributes with efficient storage
/// and bidirectional lookup capabilities. Maps token attributes to nonces and vice versa,
/// enabling efficient queries by both token nonce and attribute values.
///
/// # Storage Layout
///
/// The mapper uses a sophisticated multi-key storage layout for efficient lookups:
///
/// ```text
/// base_key + ".counter" → u8                    // Global token ID counter
/// base_key + ".mapping" + token_id → u8         // Token ID to internal mapping ID
/// base_key + ".attr" + mapping + nonce → T     // Token attributes by nonce
/// base_key + ".nonce" + mapping + attr → u64   // Nonce lookup by attributes
/// ```
///
/// # Main Operations
///
/// ## Attribute Management
/// - **Set**: Store attributes for token nonce with `set()`
/// - **Update**: Modify existing attributes with `update()`
/// - **Clear**: Remove attributes with `clear()`
/// - **Get**: Retrieve attributes by nonce with `get_attributes()`
///
/// ## Bidirectional Lookup
/// - **Nonce by Attributes**: Find nonce for specific attributes with `get_nonce()`
/// - **Attributes by Nonce**: Find attributes for specific nonce with `get_attributes()`
/// - **Existence Checks**: Verify presence with `has_attributes()`, `has_nonce()`
///
/// ## Internal Mapping
/// - **Space Optimization**: Uses u8 internal IDs to reduce storage keys
/// - **Counter Management**: Automatically assigns mapping IDs up to 255 tokens
/// - **Collision Avoidance**: Each token ID gets unique mapping space
///
/// # Trade-offs
///
/// **Advantages:**
/// - Bidirectional lookup (nonce ↔ attributes) in O(1) time
/// - Space-efficient storage with internal mapping compression
/// - Prevents duplicate attribute sets per token
/// - Supports any encodable attribute type
///
/// **Limitations:**
/// - Maximum 255 different token IDs per mapper instance
/// - Attributes cannot be changed once set (use update carefully)
/// - Complex storage layout increases implementation overhead
/// - No iteration capabilities over stored mappings
pub struct TokenAttributesMapper<SA, A = CurrentStorage>
where
    SA: StorageMapperApi,
{
    _phantom_api: PhantomData<SA>,
    base_key: StorageKey<SA>,
    address: A,
}

impl<SA> StorageMapper<SA> for TokenAttributesMapper<SA, CurrentStorage>
where
    SA: StorageMapperApi,
{
    fn new(base_key: StorageKey<SA>) -> Self {
        TokenAttributesMapper {
            _phantom_api: PhantomData,
            base_key,
            address: CurrentStorage,
        }
    }
}

impl<SA> StorageMapperFromAddress<SA> for TokenAttributesMapper<SA, ManagedAddress<SA>>
where
    SA: StorageMapperApi,
{
    fn new_from_address(address: ManagedAddress<SA>, base_key: StorageKey<SA>) -> Self {
        Self {
            _phantom_api: PhantomData,
            base_key,
            address,
        }
    }
}

impl<SA> TokenAttributesMapper<SA, CurrentStorage>
where
    SA: StorageMapperApi,
{
    pub fn set<T: TopEncode + TopDecode + NestedEncode + NestedDecode, M: ManagedTypeApi>(
        &self,
        token_id: &EsdtTokenIdentifier<M>,
        token_nonce: u64,
        attributes: &T,
    ) {
        let has_mapping = self.has_mapping_value(token_id);

        let mapping = if has_mapping {
            self.get_mapping_value(token_id)
        } else {
            let mut counter = self.get_counter_value();
            if counter == u8::MAX {
                SA::error_api_impl().signal_error(COUNTER_OVERFLOW_ERROR_MESSAGE.as_bytes());
            }

            counter += 1;
            self.set_mapping_value(token_id, counter);
            self.set_counter_value(counter);
            counter
        };

        let has_value = self.has_token_attributes_value(mapping, token_nonce);
        if has_value {
            SA::error_api_impl().signal_error(VALUE_ALREADY_SET_ERROR_MESSAGE.as_bytes());
        }

        self.set_token_attributes_value(mapping, token_nonce, attributes);
        self.set_attributes_to_nonce_mapping(mapping, attributes, token_nonce);
    }

    ///Use carefully. Update should be used mainly when backed up by the protocol.
    pub fn update<T: TopEncode + TopDecode + NestedEncode + NestedDecode, M: ManagedTypeApi>(
        &self,
        token_id: &EsdtTokenIdentifier<M>,
        token_nonce: u64,
        attributes: &T,
    ) {
        let has_mapping = self.has_mapping_value(token_id);
        if !has_mapping {
            SA::error_api_impl().signal_error(UNKNOWN_TOKEN_ID_ERROR_MESSAGE.as_bytes());
        }

        let mapping = self.get_mapping_value(token_id);
        let has_value = self.has_token_attributes_value(mapping, token_nonce);
        if !has_value {
            SA::error_api_impl().signal_error(VALUE_NOT_PREVIOUSLY_SET_ERROR_MESSAGE.as_bytes());
        }

        let old_attr = self.get_token_attributes_value::<T>(mapping, token_nonce);
        self.clear_attributes_to_nonce_mapping(mapping, &old_attr);

        self.set_token_attributes_value(mapping, token_nonce, attributes);
        self.set_attributes_to_nonce_mapping(mapping, attributes, token_nonce);
    }

    pub fn clear<T: TopEncode + TopDecode + NestedEncode + NestedDecode, M: ManagedTypeApi>(
        &self,
        token_id: &EsdtTokenIdentifier<M>,
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

    fn set_counter_value(&self, value: u8) {
        storage_set(self.build_key_token_id_counter().as_ref(), &value);
    }

    fn set_mapping_value<M: ManagedTypeApi>(&self, token_id: &EsdtTokenIdentifier<M>, value: u8) {
        storage_set(self.build_key_token_id_mapping(token_id).as_ref(), &value);
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

    fn clear_attributes_to_nonce_mapping<T: TopEncode + TopDecode + NestedEncode + NestedDecode>(
        &self,
        mapping: u8,
        attr: &T,
    ) {
        storage_clear(self.build_key_attr_to_nonce_mapping(mapping, attr).as_ref());
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

    fn clear_token_attributes_value(&self, mapping: u8, token_nonce: u64) {
        storage_clear(
            self.build_key_token_attr_value(mapping, token_nonce)
                .as_ref(),
        );
    }
}

impl<SA, A> TokenAttributesMapper<SA, A>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
{
    pub fn has_attributes<M: ManagedTypeApi>(
        &self,
        token_id: &EsdtTokenIdentifier<M>,
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
        token_id: &EsdtTokenIdentifier<M>,
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
        token_id: &EsdtTokenIdentifier<M>,
        token_nonce: u64,
    ) -> T {
        let has_mapping = self.has_mapping_value(token_id);
        if !has_mapping {
            SA::error_api_impl().signal_error(UNKNOWN_TOKEN_ID_ERROR_MESSAGE.as_bytes());
        }

        let mapping = self.get_mapping_value(token_id);
        let has_value = self.has_token_attributes_value(mapping, token_nonce);
        if !has_value {
            SA::error_api_impl().signal_error(VALUE_NOT_PREVIOUSLY_SET_ERROR_MESSAGE.as_bytes());
        }

        self.get_token_attributes_value(mapping, token_nonce)
    }

    pub fn get_nonce<T: TopEncode + TopDecode + NestedEncode + NestedDecode, M: ManagedTypeApi>(
        &self,
        token_id: &EsdtTokenIdentifier<M>,
        attr: &T,
    ) -> u64 {
        let has_mapping = self.has_mapping_value(token_id);
        if !has_mapping {
            SA::error_api_impl().signal_error(UNKNOWN_TOKEN_ID_ERROR_MESSAGE.as_bytes());
        }

        let mapping = self.get_mapping_value(token_id);
        let has_value = self.has_attr_to_nonce_mapping::<T>(mapping, attr);
        if !has_value {
            SA::error_api_impl().signal_error(VALUE_NOT_PREVIOUSLY_SET_ERROR_MESSAGE.as_bytes());
        }

        self.get_attributes_to_nonce_mapping(mapping, attr)
    }

    fn has_mapping_value<M: ManagedTypeApi>(&self, token_id: &EsdtTokenIdentifier<M>) -> bool {
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
        key.append_bytes(COUNTER_SUFFIX.as_bytes());
        key
    }

    fn build_key_token_id_mapping<M: ManagedTypeApi>(
        &self,
        token_id: &EsdtTokenIdentifier<M>,
    ) -> StorageKey<SA> {
        let mut key = self.base_key.clone();
        key.append_bytes(MAPPING_SUFFIX.as_bytes());
        key.append_item(token_id);
        key
    }

    fn build_key_token_attr_value(&self, mapping: u8, token_nonce: u64) -> StorageKey<SA> {
        let mut key = self.base_key.clone();
        key.append_bytes(ATTR_SUFFIX.as_bytes());
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
        key.append_bytes(NONCE_SUFFIX.as_bytes());
        key.append_item(&mapping);
        key.append_item(attr);
        key
    }

    fn get_counter_value(&self) -> u8 {
        storage_get(self.build_key_token_id_counter().as_ref())
    }

    fn get_mapping_value<M: ManagedTypeApi>(&self, token_id: &EsdtTokenIdentifier<M>) -> u8 {
        storage_get(self.build_key_token_id_mapping(token_id).as_ref())
    }

    fn is_empty_mapping_value<M: ManagedTypeApi>(&self, token_id: &EsdtTokenIdentifier<M>) -> bool {
        storage_get_len(self.build_key_token_id_mapping(token_id).as_ref()) == 0
    }

    fn get_attributes_to_nonce_mapping<T: TopEncode + TopDecode + NestedEncode + NestedDecode>(
        &self,
        mapping: u8,
        attr: &T,
    ) -> u64 {
        storage_get(self.build_key_attr_to_nonce_mapping(mapping, attr).as_ref())
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

    fn is_empty_token_attributes_value(&self, mapping: u8, token_nonce: u64) -> bool {
        storage_get_len(
            self.build_key_token_attr_value(mapping, token_nonce)
                .as_ref(),
        ) == 0
    }
}
