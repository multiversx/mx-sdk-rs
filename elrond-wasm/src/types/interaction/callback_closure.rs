use crate::{
    api::{BlockchainApi, ErrorApi, ManagedTypeApi, StorageReadApi, StorageWriteApi},
    storage::StorageKey,
    storage_clear, storage_get, storage_get_len, storage_set,
    types::{ManagedBuffer, ManagedType},
    ContractCallArg, ManagedResultArgLoader,
};
use elrond_codec::elrond_codec_derive::{TopDecode, TopEncode};

use super::ManagedArgBuffer;

#[derive(TopEncode, TopDecode)]
pub struct CallbackClosure<M: ManagedTypeApi> {
    callback_name: ManagedBuffer<M>,
    closure_args: ManagedArgBuffer<M>,
}

/// Syntactical sugar to help macros to generate code easier.
/// Unlike calling `CallbackClosure::<SA, R>::new`, here types can be inferred from the context.
pub fn new_callback_call<A>(api: A, callback_name_slice: &'static [u8]) -> CallbackClosure<A>
where
    A: ManagedTypeApi + ErrorApi,
{
    let callback_name = ManagedBuffer::new_from_bytes(api, callback_name_slice);
    CallbackClosure::new(callback_name)
}

impl<M: ManagedTypeApi> CallbackClosure<M> {
    pub fn new(callback_name: ManagedBuffer<M>) -> Self {
        let type_manager = callback_name.type_manager();
        let arg_buffer = ManagedArgBuffer::new_empty(type_manager);
        CallbackClosure {
            callback_name,
            closure_args: arg_buffer,
        }
    }

    /// Used by callback_raw.
    /// TODO: avoid creating any new managed buffers.
    pub fn new_empty(api: M) -> Self {
        CallbackClosure {
            callback_name: ManagedBuffer::new_empty(api.clone()),
            closure_args: ManagedArgBuffer::new_empty(api),
        }
    }

    pub fn push_endpoint_arg<D: ContractCallArg>(&mut self, endpoint_arg: D) {
        endpoint_arg.push_dyn_arg(&mut self.closure_args);
    }

    pub fn save_to_storage<A: BlockchainApi + StorageWriteApi>(&self, api: A) {
        let tx_hash = api.get_tx_hash_managed();
        storage_set(api, &tx_hash.into(), self);
    }

    pub fn storage_load_and_clear<A: BlockchainApi + StorageReadApi + StorageWriteApi>(
        api: A,
    ) -> Option<Self> {
        let tx_hash = api.get_tx_hash_managed();
        let storage_key = StorageKey::from(tx_hash);
        if storage_get_len(api.clone(), &storage_key) > 0 {
            let closure = storage_get(api.clone(), &storage_key);
            storage_clear(api, &storage_key);
            Some(closure)
        } else {
            None
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.callback_name.is_empty()
    }

    pub fn name_matches(&self, name_match: &[u8]) -> bool {
        &self.callback_name == name_match
    }

    pub fn into_arg_loader(self) -> ManagedResultArgLoader<M> {
        ManagedResultArgLoader::new(self.closure_args.data)
    }
}
