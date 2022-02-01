use crate::{
    api::{
        BlockchainApi, BlockchainApiImpl, ErrorApi, ManagedTypeApi, StorageReadApi, StorageWriteApi,
    },
    contract_base::ManagedSerializer,
    storage::StorageKey,
    storage_clear, storage_get, storage_set,
    types::{ManagedBuffer, ManagedType},
    ContractCallArg, ManagedResultArgLoader,
};
use elrond_codec::elrond_codec_derive::{TopDecode, TopEncode};

use super::ManagedArgBuffer;

pub const CALLBACK_CLOSURE_STORAGE_BASE_KEY: &[u8] = b"CB_CLOSURE";

#[derive(TopEncode, TopDecode)]
pub struct CallbackClosure<M: ManagedTypeApi + ErrorApi> {
    callback_name: ManagedBuffer<M>,
    closure_args: ManagedArgBuffer<M>,
}

/// Syntactical sugar to help macros to generate code easier.
/// Unlike calling `CallbackClosure::<SA, R>::new`, here types can be inferred from the context.
pub fn new_callback_call<A>(callback_name_slice: &'static [u8]) -> CallbackClosure<A>
where
    A: ManagedTypeApi + ErrorApi,
{
    let callback_name = ManagedBuffer::new_from_bytes(callback_name_slice);
    CallbackClosure::new(callback_name)
}

impl<M: ManagedTypeApi + ErrorApi> CallbackClosure<M> {
    pub fn new(callback_name: ManagedBuffer<M>) -> Self {
        let arg_buffer = ManagedArgBuffer::new_empty();
        CallbackClosure {
            callback_name,
            closure_args: arg_buffer,
        }
    }

    /// Used by callback_raw.
    /// TODO: avoid creating any new managed buffers.
    pub fn new_empty() -> Self {
        CallbackClosure {
            callback_name: ManagedBuffer::new(),
            closure_args: ManagedArgBuffer::new_empty(),
        }
    }

    pub fn push_endpoint_arg<D: ContractCallArg>(&mut self, endpoint_arg: D) {
        endpoint_arg.push_dyn_arg(&mut self.closure_args);
    }

    pub fn save_to_storage<A: BlockchainApi + StorageWriteApi>(&self) {
        let storage_key = cb_closure_storage_key::<A>();
        storage_set(storage_key.as_ref(), self);
    }

    pub fn storage_load_and_clear<A: BlockchainApi + StorageReadApi + StorageWriteApi>(
    ) -> Option<Self> {
        let storage_key = cb_closure_storage_key::<A>();
        let storage_value_raw: ManagedBuffer<A> = storage_get(storage_key.as_ref());
        if !storage_value_raw.is_empty() {
            let serializer = ManagedSerializer::<A>::new();
            let closure = serializer.top_decode_from_managed_buffer(&storage_value_raw);
            storage_clear(storage_key.as_ref());
            Some(closure)
        } else {
            None
        }
    }

    pub fn matcher<const CB_NAME_MAX_LENGTH: usize>(
        &self,
    ) -> CallbackClosureMatcher<CB_NAME_MAX_LENGTH> {
        CallbackClosureMatcher::new(&self.callback_name)
    }

    pub fn into_arg_loader(self) -> ManagedResultArgLoader<M> {
        ManagedResultArgLoader::new(self.closure_args.data)
    }
}

pub(super) fn cb_closure_storage_key<A: BlockchainApi>() -> StorageKey<A> {
    let tx_hash = A::blockchain_api_impl().get_tx_hash::<A>();
    let mut storage_key = StorageKey::new(CALLBACK_CLOSURE_STORAGE_BASE_KEY);
    storage_key.append_managed_buffer(tx_hash.as_managed_buffer());
    storage_key
}

/// Helps the callback macro expansion to perform callback name matching more efficiently.
/// The current implementation hashes by callback name length,
/// but in principle further optimizations are possible.
pub struct CallbackClosureMatcher<const CB_NAME_MAX_LENGTH: usize> {
    name_len: usize,
    compare_buffer: [u8; CB_NAME_MAX_LENGTH],
}

impl<const CB_NAME_MAX_LENGTH: usize> CallbackClosureMatcher<CB_NAME_MAX_LENGTH> {
    pub fn new<M: ManagedTypeApi + ErrorApi>(callback_name: &ManagedBuffer<M>) -> Self {
        let mut compare_buffer = [0u8; CB_NAME_MAX_LENGTH];
        let name_len = callback_name.len();
        let _ = callback_name.load_slice(0, &mut compare_buffer[..name_len]);
        CallbackClosureMatcher {
            name_len,
            compare_buffer,
        }
    }

    pub fn new_from_unmanaged(callback_name: &[u8]) -> Self {
        let mut compare_buffer = [0u8; CB_NAME_MAX_LENGTH];
        let name_len = callback_name.len();
        compare_buffer[..name_len].copy_from_slice(callback_name);
        CallbackClosureMatcher {
            name_len,
            compare_buffer,
        }
    }

    pub fn matches_empty(&self) -> bool {
        self.name_len == 0
    }

    pub fn name_matches(&self, name_match: &[u8]) -> bool {
        if self.name_len != name_match.len() {
            false
        } else {
            &self.compare_buffer[..self.name_len] == name_match
        }
    }
}
