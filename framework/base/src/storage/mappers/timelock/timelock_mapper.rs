use core::{borrow::Borrow, marker::PhantomData};

pub use crate::storage::mappers::{
    single_value_mapper::SingleValue, source::CurrentStorage, StorageMapper,
};
use crate::{
    abi::{TypeAbi, TypeAbiFrom, TypeDescriptionContainer, TypeName},
    api::{BlockchainApi, BlockchainApiImpl, StorageMapperApi},
    codec::{
        multi_types::PlaceholderOutput, EncodeErrorHandler, TopDecode, TopEncode, TopEncodeMulti,
        TopEncodeMultiOutput,
    },
    imports::StorageMapperFromAddress,
    storage::{
        mappers::source::StorageAddress, storage_clear, storage_overwrite, storage_set, StorageKey,
    },
    storage_get,
    types::{ManagedAddress, ManagedType},
};

const UNLOCK_TIMESTAMP_KEY: &[u8] = b"unlock_timestamp";
const FUTURE_VALUE_KEY: &[u8] = b"future_value";

pub struct TimelockMapper<SA, T, A = CurrentStorage>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
    T: TopEncode + TopDecode + 'static,
{
    address: A,
    key: StorageKey<SA>,
    _phantom_api: PhantomData<SA>,
    _phantom_item: PhantomData<T>,
}

impl<SA, T> StorageMapper<SA> for TimelockMapper<SA, T>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode,
{
    #[inline]
    fn new(base_key: StorageKey<SA>) -> Self {
        TimelockMapper {
            address: CurrentStorage,
            key: base_key,
            _phantom_api: PhantomData,
            _phantom_item: PhantomData,
        }
    }
}

impl<SA, T> TimelockMapper<SA, T>
where
    SA: StorageMapperApi + BlockchainApi,
    T: TopEncode + TopDecode,
{
    /// Sets the `current value` without taking into account the timelock component.
    /// Meant to be used in constructors.
    pub fn set<BT>(&self, new_current_value: BT)
    where
        BT: Borrow<T>,
    {
        storage_set(self.key.as_ref(), new_current_value.borrow());
    }

    /// Updates current value entry with future value if unlock timestamp has passed.
    pub fn commit(&self) -> bool {
        let now = SA::blockchain_api_impl().get_block_timestamp();
        let unlock_timestamp: u64 = storage_get(self.get_unlock_timestamp_key().as_ref());

        if now >= unlock_timestamp {
            storage_overwrite(self.get_future_value_key().as_ref(), self.key.as_ref());
            storage_clear(self.get_future_value_key().as_ref());
            return true;
        }

        false
    }

    /// Sets a value and an unlock timestamp for the value.
    /// Setup needs to be committed after the unlock timestamp has passed.
    /// Unlock timestamp represents the moment in time when the future value can be
    /// updated as current value.
    pub fn set_unlock_timestamp<BT>(&self, unlock_timestamp: u64, future_value: BT)
    where
        BT: Borrow<T>,
    {
        storage_set(self.get_unlock_timestamp_key().as_ref(), &unlock_timestamp);
        storage_set(self.get_future_value_key().as_ref(), future_value.borrow());
    }
}

impl<SA, T, A> TimelockMapper<SA, T, A>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode,
    A: StorageAddress<SA>,
{
    /// Retrieves the current value from storage.
    pub fn get(&self) -> T {
        self.address.address_storage_get(self.key.as_ref())
    }

    /// Retrieves the unlock timestamp from storage.
    pub fn get_unlock_timestamp(&self) -> u64 {
        self.address
            .address_storage_get(self.get_unlock_timestamp_key().as_ref())
    }

    /// Retrieves the future value from storage.
    pub fn get_future_value(&self) -> T {
        self.address
            .address_storage_get(self.get_future_value_key().as_ref())
    }

    fn get_unlock_timestamp_key(&self) -> StorageKey<SA> {
        let mut base_key = self.key.buffer.clone();
        base_key.append_bytes(UNLOCK_TIMESTAMP_KEY);

        StorageKey::from(base_key)
    }

    fn get_future_value_key(&self) -> StorageKey<SA> {
        let mut base_key = self.key.buffer.clone();
        base_key.append_bytes(FUTURE_VALUE_KEY);

        StorageKey::from(base_key)
    }
}

impl<SA, T> StorageMapperFromAddress<SA> for TimelockMapper<SA, T, ManagedAddress<SA>>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode,
{
    #[inline]
    fn new_from_address(address: ManagedAddress<SA>, base_key: StorageKey<SA>) -> Self {
        TimelockMapper {
            address,
            key: base_key,
            _phantom_api: PhantomData,
            _phantom_item: PhantomData,
        }
    }
}

impl<SA, T> TopEncodeMulti for TimelockMapper<SA, T>
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

impl<SA, T, R> TypeAbiFrom<TimelockMapper<SA, T>> for SingleValue<R>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode,
    R: TopDecode + TypeAbiFrom<T>,
{
}

impl<SA, T> TypeAbiFrom<TimelockMapper<SA, T>> for PlaceholderOutput
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode,
{
}

impl<SA, T> TypeAbiFrom<Self> for TimelockMapper<SA, T>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode + TypeAbi,
{
}

impl<SA, T> TypeAbi for TimelockMapper<SA, T>
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
