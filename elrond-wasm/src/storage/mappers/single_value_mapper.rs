use super::StorageMapper;
use crate::{
    abi::{TypeAbi, TypeDescriptionContainer, TypeName},
    api::{EndpointFinishApi, ErrorApi, ManagedTypeApi, StorageReadApi, StorageWriteApi},
    io::EndpointResult,
    storage::{storage_clear, storage_get, storage_get_len, storage_set, StorageKey},
};
use core::marker::PhantomData;
use elrond_codec::{TopDecode, TopEncode};

/// Manages a single serializable item in storage.
pub struct SingleValueMapper<SA, T>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
    T: TopEncode + TopDecode + 'static,
{
    api: SA,
    key: StorageKey<SA>,
    _phantom: core::marker::PhantomData<T>,
}

impl<SA, T> StorageMapper<SA> for SingleValueMapper<SA, T>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
    T: TopEncode + TopDecode,
{
    fn new(api: SA, base_key: StorageKey<SA>) -> Self {
        SingleValueMapper {
            api,
            key: base_key,
            _phantom: PhantomData,
        }
    }
}

impl<SA, T> SingleValueMapper<SA, T>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
    T: TopEncode + TopDecode,
{
    /// Retrieves current value from storage.
    pub fn get(&self) -> T {
        storage_get(self.api.clone(), &self.key)
    }

    /// Returns whether the storage managed by this mapper is empty.
    pub fn is_empty(&self) -> bool {
        self.raw_byte_length() == 0
    }

    /// Saves argument to storage.
    pub fn set(&self, new_value: &T) {
        storage_set(self.api.clone(), &self.key, new_value);
    }

    /// Saves argument to storage only if the storage is empty.
    /// Does nothing otherwise.
    pub fn set_if_empty(&self, value: &T) {
        if self.is_empty() {
            self.set(value);
        }
    }

    /// Clears the storage for this mapper.
    pub fn clear(&self) {
        storage_clear(self.api.clone(), &self.key);
    }

    /// Syntactic sugar, to more compactly express a get, update and set in one line.
    /// Takes whatever lies in storage, apples the given closure and saves the final value back to storage.
    /// Propagates the return value of the given function.
    pub fn update<R, F: FnOnce(&mut T) -> R>(&self, f: F) -> R {
        let mut value = self.get();
        let result = f(&mut value);
        self.set(&value);
        result
    }

    pub fn raw_byte_length(&self) -> usize {
        storage_get_len(self.api.clone(), &self.key)
    }
}

impl<SA, T> EndpointResult for SingleValueMapper<SA, T>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
    T: TopEncode + TopDecode + EndpointResult,
{
    type DecodeAs = T::DecodeAs;

    fn finish<FA>(&self, api: FA)
    where
        FA: ManagedTypeApi + EndpointFinishApi + Clone + 'static,
    {
        self.get().finish(api);
    }
}

impl<SA, T> TypeAbi for SingleValueMapper<SA, T>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
    T: TopEncode + TopDecode + TypeAbi,
{
    fn type_name() -> TypeName {
        T::type_name()
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        T::provide_type_descriptions(accumulator)
    }
}
