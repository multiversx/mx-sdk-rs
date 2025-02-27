use core::{borrow::Borrow, marker::PhantomData};

use super::set_mapper::CurrentStorageLocked;
pub use super::{
    set_mapper::{CurrentStorage, StorageAddress},
    StorageMapper, StorageMapperFromAddress, StorageMapperWithTimelock,
};
use crate::{
    abi::{TypeAbi, TypeAbiFrom, TypeDescriptionContainer, TypeName},
    api::{BlockchainApi, BlockchainApiImpl, StorageMapperApi},
    codec::{
        multi_types::PlaceholderOutput, DecodeErrorHandler, EncodeErrorHandler, TopDecode,
        TopDecodeInput, TopEncode, TopEncodeMulti, TopEncodeMultiOutput, TopEncodeOutput,
    },
    storage::{storage_clear, storage_set, StorageKey},
    types::{ManagedAddress, ManagedType},
};

pub type SingleValueMapperWithTimelock<SA, T> = SingleValueMapper<SA, T, CurrentStorageLocked>;

impl<SA, T> StorageMapper<SA> for SingleValueMapperWithTimelock<SA, T>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode,
{
    #[inline]
    fn new(base_key: StorageKey<SA>) -> Self {
        SingleValueMapper {
            address: CurrentStorageLocked,
            unlock_timestamp: 0u64,
            key: base_key,
            _phantom_api: PhantomData,
            _phantom_item: PhantomData,
        }
    }
}

/// Manages a single serializable item in storage.
pub struct SingleValueMapper<SA, T, A = CurrentStorage>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
    T: TopEncode + TopDecode + 'static,
{
    address: A,
    unlock_timestamp: u64,
    key: StorageKey<SA>,
    _phantom_api: PhantomData<SA>,
    _phantom_item: PhantomData<T>,
}

impl<SA, T> StorageMapper<SA> for SingleValueMapper<SA, T, CurrentStorage>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode,
{
    #[inline]
    fn new(base_key: StorageKey<SA>) -> Self {
        SingleValueMapper {
            address: CurrentStorage,
            unlock_timestamp: 0u64,
            key: base_key,
            _phantom_api: PhantomData,
            _phantom_item: PhantomData,
        }
    }
}

impl<SA, T> StorageMapperFromAddress<SA> for SingleValueMapper<SA, T, ManagedAddress<SA>>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode,
{
    #[inline]
    fn new_from_address(address: ManagedAddress<SA>, base_key: StorageKey<SA>) -> Self {
        SingleValueMapper {
            address,
            unlock_timestamp: 0u64,
            key: base_key,
            _phantom_api: PhantomData,
            _phantom_item: PhantomData,
        }
    }
}

impl<SA, T> StorageMapperWithTimelock<SA> for SingleValueMapper<SA, T, CurrentStorageLocked>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode,
{
    #[inline]
    fn new_locked(unlock_timestamp: u64, base_key: StorageKey<SA>) -> Self {
        SingleValueMapper {
            address: CurrentStorageLocked,
            unlock_timestamp,
            key: base_key,
            _phantom_api: PhantomData,
            _phantom_item: PhantomData,
        }
    }
}

impl<SA, T> SingleValueMapper<SA, T, CurrentStorage>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode,
{
    pub fn lock(self) -> SingleValueMapper<SA, T, CurrentStorageLocked> {
        SingleValueMapper {
            address: CurrentStorageLocked,
            unlock_timestamp: self.unlock_timestamp,
            key: self.key,
            _phantom_api: PhantomData,
            _phantom_item: PhantomData,
        }
    }
}

impl<SA, T> SingleValueMapper<SA, T, CurrentStorageLocked>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode,
{
    pub fn unlock(self) -> SingleValueMapper<SA, T, CurrentStorage> {
        SingleValueMapper {
            address: CurrentStorage,
            unlock_timestamp: 0u64,
            key: self.key,
            _phantom_api: PhantomData,
            _phantom_item: PhantomData,
        }
    }
}

impl<SA, T> SingleValueMapper<SA, T, CurrentStorageLocked>
where
    SA: StorageMapperApi + BlockchainApi,
    T: TopEncode + TopDecode,
{
    /// Saves an argument to storage.
    /// Returns `true` if the unlock timestamp has passed.
    pub fn set_if_unlocked<BT>(&self, new_value: BT) -> bool
    where
        BT: Borrow<T>,
    {
        let now = SA::blockchain_api_impl().get_block_timestamp();

        if now >= self.unlock_timestamp {
            storage_set(self.key.as_ref(), new_value.borrow());
            true
        } else {
            false
        }
    }

    /// Clears the storage for this mapper.
    /// Returns `true` if the unlock timestamp has passed.
    pub fn clear_if_unlocked(&self) -> bool {
        let now = SA::blockchain_api_impl().get_block_timestamp();

        if now >= self.unlock_timestamp {
            storage_clear(self.key.as_ref());
            true
        } else {
            false
        }
    }

    /// Sets a value for the unlock timestamp field.
    pub fn set_unlock_timestamp(&mut self, unlock_timestamp: u64) {
        self.unlock_timestamp = unlock_timestamp
    }

    /// Syntactic sugar, to more compactly express a get, update and set in one line.
    /// Takes whatever lies in storage, apples the given closure and saves the final value back to storage.
    /// If the update was successful, the function returns the new value.
    /// Otherwise, the function returns a default value of the same type.
    pub fn update_if_unlocked<R, F: FnOnce(&mut T) -> R>(&self, f: F) -> R {
        let mut value = self.get();
        let result = f(&mut value);

        assert!(
            self.set_if_unlocked(value),
            "Can not update, unlock timestamp has not passed"
        );
        // if self.set_if_unlocked(value) {
        //     return true;
        // }

        result
    }
}

impl<SA, T, A> SingleValueMapper<SA, T, A>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
    T: TopEncode + TopDecode,
{
    /// Retrieves current value from storage.
    pub fn get(&self) -> T {
        self.address.address_storage_get(self.key.as_ref())
    }

    /// Returns whether the storage managed by this mapper is empty.
    pub fn is_empty(&self) -> bool {
        self.raw_byte_length() == 0
    }

    pub fn raw_byte_length(&self) -> usize {
        self.address.address_storage_get_len(self.key.as_ref())
    }

    /// Returns the unlock_timestamp field value.
    pub fn get_unlock_timestamp(&self) -> u64 {
        self.unlock_timestamp
    }
}

impl<SA, T> SingleValueMapper<SA, T, CurrentStorage>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode,
{
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

impl<SA, T> TopEncodeMulti for SingleValueMapper<SA, T, CurrentStorage>
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

impl<SA, T> TopEncodeMulti for SingleValueMapperWithTimelock<SA, T>
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
/// Necessary because we cannot implement `TypeAbiFrom` directly on `T`.
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
        Ok(SingleValue::<T>(T::top_decode_or_handle_err(input, h)?))
    }
}

impl<T: TopDecode> From<T> for SingleValue<T> {
    fn from(value: T) -> Self {
        SingleValue::<T>(value)
    }
}

impl<T: TopDecode> SingleValue<T> {
    #[inline]
    pub fn into(self) -> T {
        self.0
    }
}

impl<SA, T, R> TypeAbiFrom<SingleValueMapper<SA, T, CurrentStorage>> for SingleValue<R>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode,
    R: TopDecode + TypeAbiFrom<T>,
{
}

impl<SA, T> TypeAbiFrom<SingleValueMapper<SA, T>> for PlaceholderOutput
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode,
{
}

impl<SA, T> TypeAbiFrom<Self> for SingleValueMapper<SA, T, CurrentStorage>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode + TypeAbi,
{
}

impl<SA, T> TypeAbi for SingleValueMapper<SA, T, CurrentStorage>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode + TypeAbi,
{
    type Unmanaged = T::Unmanaged;

    fn type_name() -> TypeName {
        T::type_name()
    }

    fn type_name_rust() -> TypeName {
        T::type_name_rust()
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        T::provide_type_descriptions(accumulator)
    }
}

impl<SA, T, R> TypeAbiFrom<SingleValueMapperWithTimelock<SA, T>> for SingleValue<R>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode,
    R: TopDecode + TypeAbiFrom<T>,
{
}

impl<SA, T> TypeAbiFrom<SingleValueMapperWithTimelock<SA, T>> for PlaceholderOutput
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode,
{
}

impl<SA, T> TypeAbiFrom<Self> for SingleValueMapperWithTimelock<SA, T>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode + TypeAbi,
{
}

impl<SA, T> TypeAbi for SingleValueMapperWithTimelock<SA, T>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode + TypeAbi,
{
    type Unmanaged = T::Unmanaged;

    fn type_name() -> TypeName {
        T::type_name()
    }

    fn type_name_rust() -> TypeName {
        T::type_name_rust()
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        T::provide_type_descriptions(accumulator)
    }
}
