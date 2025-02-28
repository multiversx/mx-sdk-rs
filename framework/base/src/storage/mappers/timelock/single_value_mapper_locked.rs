use core::{borrow::Borrow, marker::PhantomData};

pub use crate::storage::mappers::{
    single_value_mapper::{SingleValue, SingleValueMapper},
    source::{CurrentStorage, CurrentStorageLocked},
    StorageMapper, StorageMapperWithTimelock,
};
use crate::{
    abi::{TypeAbi, TypeAbiFrom, TypeDescriptionContainer, TypeName},
    api::{BlockchainApi, BlockchainApiImpl, StorageMapperApi},
    codec::{
        multi_types::PlaceholderOutput, EncodeErrorHandler, TopDecode, TopEncode, TopEncodeMulti,
        TopEncodeMultiOutput,
    },
    storage::{storage_clear, storage_set, StorageKey},
    types::ManagedType,
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

impl<SA, T> StorageMapperWithTimelock<SA> for SingleValueMapperWithTimelock<SA, T>
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

impl<SA, T> SingleValueMapperWithTimelock<SA, T>
where
    SA: StorageMapperApi + BlockchainApi,
    T: TopEncode + TopDecode,
{
    /// Removes the timelock component of a single value mapper.
    pub fn unlock(self) -> SingleValueMapper<SA, T, CurrentStorage> {
        SingleValueMapper {
            address: CurrentStorage,
            unlock_timestamp: 0u64,
            key: self.key,
            _phantom_api: PhantomData,
            _phantom_item: PhantomData,
        }
    }

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
    /// Otherwise, the function returns the previous value.
    pub fn update_if_unlocked<F: FnOnce(&mut T)>(&self, f: F) -> T {
        let mut value = self.get();
        f(&mut value);

        if self.set_if_unlocked(&value) {
            return value;
        }

        self.get()
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
