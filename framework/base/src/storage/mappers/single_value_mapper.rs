use core::{borrow::Borrow, marker::PhantomData};

use super::StorageMapper;
use crate::{
    abi::{TypeAbi, TypeDescriptionContainer, TypeName},
    api::StorageMapperApi,
    codec::{
        multi_types::PlaceholderOutput, CodecFrom, CodecFromSelf, DecodeErrorHandler,
        EncodeErrorHandler, TopDecode, TopDecodeInput, TopEncode, TopEncodeMulti,
        TopEncodeMultiOutput, TopEncodeOutput,
    },
    storage::{
        storage_clear, storage_get, storage_get_from_address, storage_get_len, storage_set,
        StorageKey,
    },
    types::{ManagedAddress, ManagedType},
};
use storage_get_from_address::storage_get_len_from_address;

/// Manages a single serializable item in storage.
pub struct SingleValueMapper<SA, T>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode + 'static,
{
    key: StorageKey<SA>,
    _phantom_api: PhantomData<SA>,
    _phantom_item: PhantomData<T>,
}

impl<SA, T> StorageMapper<SA> for SingleValueMapper<SA, T>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode,
{
    #[inline]
    fn new(base_key: StorageKey<SA>) -> Self {
        SingleValueMapper {
            key: base_key,
            _phantom_api: PhantomData,
            _phantom_item: PhantomData,
        }
    }
}

impl<SA, T> SingleValueMapper<SA, T>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode,
{
    /// Retrieves current value from storage.
    pub fn get(&self) -> T {
        storage_get(self.key.as_ref())
    }

    /// Gets the value from the given address. Both adresses have to be in the same shard.
    pub fn get_from_address(&self, address: &ManagedAddress<SA>) -> T {
        storage_get_from_address(address.as_ref(), self.key.as_ref())
    }

    /// Returns whether the storage managed by this mapper is empty.
    pub fn is_empty(&self) -> bool {
        self.raw_byte_length() == 0
    }

    /// Returns whether the storage at the given key is empty at the given address.
    /// Both adresses have to be in the same shard.
    pub fn is_empty_at_address(&self, address: &ManagedAddress<SA>) -> bool {
        let len = storage_get_len_from_address(address.as_ref(), self.key.as_ref());
        len == 0
    }

    /// Saves argument to storage.
    ///
    /// Accepts owned item of type `T`, or any borrowed form of it, such as `&T`.
    #[inline]
    pub fn set<BT>(&self, new_value: BT)
    where
        BT: Borrow<T>,
    {
        storage_set(self.key.as_ref(), new_value.borrow());
    }

    /// Saves argument to storage only if the storage is empty.
    /// Does nothing otherwise.
    pub fn set_if_empty<BT>(&self, value: BT)
    where
        BT: Borrow<T>,
    {
        if self.is_empty() {
            self.set(value);
        }
    }

    /// Clears the storage for this mapper.
    pub fn clear(&self) {
        storage_clear(self.key.as_ref());
    }

    /// Syntactic sugar, to more compactly express a get, update and set in one line.
    /// Takes whatever lies in storage, apples the given closure and saves the final value back to storage.
    /// Propagates the return value of the given function.
    pub fn update<R, F: FnOnce(&mut T) -> R>(&self, f: F) -> R {
        let mut value = self.get();
        let result = f(&mut value);
        self.set(value);
        result
    }

    pub fn raw_byte_length(&self) -> usize {
        storage_get_len(self.key.as_ref())
    }

    /// Takes the value out of the storage, clearing it in the process.
    pub fn take(&self) -> T {
        let value = self.get();
        self.clear();
        value
    }

    // Replaces the actual value in the storage by the value given in parameter, returning the old value.
    pub fn replace<BT>(&self, new_value: BT) -> T
    where
        BT: Borrow<T>,
    {
        let value = self.get();
        self.set(new_value);
        value
    }
}

impl<SA, T> TopEncodeMulti for SingleValueMapper<SA, T>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode,
{
    fn multi_encode_or_handle_err<O, H>(&self, output: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeMultiOutput,
        H: EncodeErrorHandler,
    {
        output.push_single_value(&self.get(), h)
    }
}

/// Intermediary type for deserializing the result of an endpoint that returns a `SingleValueMapper`.
///
/// Necessary because we cannot implement `CodecFrom` directly on `T`.
pub struct SingleValue<T: TopDecode>(T);

impl<T: TopEncode + TopDecode> TopEncode for SingleValue<T> {
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.0.top_encode_or_handle_err(output, h)
    }
}

impl<T: TopDecode> TopDecode for SingleValue<T> {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(SingleValue(T::top_decode_or_handle_err(input, h)?))
    }
}

impl<T: TopDecode> From<T> for SingleValue<T> {
    fn from(value: T) -> Self {
        SingleValue(value)
    }
}

impl<T: TopDecode> SingleValue<T> {
    #[inline]
    pub fn into(self) -> T {
        self.0
    }
}

impl<SA, T> !CodecFromSelf for SingleValueMapper<SA, T>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode,
{
}

impl<SA, T, R> CodecFrom<SingleValueMapper<SA, T>> for SingleValue<R>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode,
    R: TopDecode + CodecFrom<T>,
{
}

impl<SA, T> CodecFrom<SingleValueMapper<SA, T>> for PlaceholderOutput
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode,
{
}

impl<SA, T> TypeAbi for SingleValueMapper<SA, T>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode + TypeAbi,
{
    fn type_name() -> TypeName {
        T::type_name()
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        T::provide_type_descriptions(accumulator)
    }
}
