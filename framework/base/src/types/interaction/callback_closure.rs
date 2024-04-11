use crate::{
    api::{BlockchainApi, ErrorApi, ManagedTypeApi, StorageReadApi, StorageWriteApi},
    codec::{
        self,
        derive::{TopDecode, TopEncode},
        TopEncodeMulti,
    },
    contract_base::{BlockchainWrapper, ExitCodecErrorHandler, ManagedSerializer},
    err_msg,
    io::ManagedResultArgLoader,
    storage::StorageKey,
    storage_clear, storage_get, storage_set,
    types::{ManagedBuffer, ManagedType},
};

use super::ManagedArgBuffer;

pub const CALLBACK_CLOSURE_STORAGE_BASE_KEY: &[u8] = b"CB_CLOSURE";

/// Object that encodes full async callback data.
///
/// Should not be created manually, we have auto-generated call proxies
/// that will create this object in a type-safe manner.
///
/// How it functions:
/// - With the old async call mechanism, this data is serialized to storage.
/// - With the new promises framework, the VM handles this data.
///
/// In both cases the framework hides all the magic, the developer shouldn't worry about it.
#[derive(TopEncode)]
pub struct CallbackClosure<'a, M: ManagedTypeApi<'a> + ErrorApi> {
    pub(super) callback_name: &'static str,
    pub(super) closure_args: ManagedArgBuffer<'a, M>,
}

/// Syntactical sugar to help macros to generate code easier.
/// Unlike calling `CallbackClosure::<'a, SA, R>::new`, here types can be inferred from the context.
pub fn new_callback_call<'a, A>(callback_name: &'static str) -> CallbackClosure<'a, A>
where
    A: ManagedTypeApi<'a> + ErrorApi,
{
    CallbackClosure::new(callback_name)
}

impl<'a, M: ManagedTypeApi<'a> + ErrorApi> CallbackClosure<'a, M> {
    pub fn new(callback_name: &'static str) -> Self {
        CallbackClosure {
            callback_name,
            closure_args: ManagedArgBuffer::new(),
        }
    }

    pub fn push_endpoint_arg<T: TopEncodeMulti>(&mut self, endpoint_arg: &T) {
        let h = ExitCodecErrorHandler::<'a, M>::from(err_msg::CONTRACT_CALL_ENCODE_ERROR);
        let Ok(()) = endpoint_arg.multi_encode_or_handle_err(&mut self.closure_args, h);
    }

    pub fn save_to_storage<A: BlockchainApi<'a> + StorageWriteApi>(&self) {
        let storage_key = cb_closure_storage_key::<'a, A>();
        storage_set(storage_key.as_ref(), self);
    }
}

pub(super) fn cb_closure_storage_key<'a, A: BlockchainApi<'a>>() -> StorageKey<'a, A> {
    let tx_hash = BlockchainWrapper::<'a, A>::new().get_tx_hash();
    let mut storage_key = StorageKey::new(CALLBACK_CLOSURE_STORAGE_BASE_KEY);
    storage_key.append_managed_buffer(tx_hash.as_managed_buffer());
    storage_key
}

/// Similar object to `CallbackClosure`, but only used for deserializing from storage
/// the callback data with the old async call mechanism.
///
/// Should not be visible to the developer.
///
/// It is a separate type from `CallbackClosure`, because we want a different representation of the endpoint name.
#[derive(TopDecode)]
pub struct CallbackClosureForDeser<'a, M: ManagedTypeApi<'a> + ErrorApi> {
    callback_name: ManagedBuffer<'a, M>,
    closure_args: ManagedArgBuffer<'a, M>,
}

impl<'a, M: ManagedTypeApi<'a> + ErrorApi> CallbackClosureForDeser<'a, M> {
    /// Used by callback_raw.
    /// TODO: avoid creating any new managed buffers.
    pub fn no_callback() -> Self {
        CallbackClosureForDeser {
            callback_name: ManagedBuffer::new(),
            closure_args: ManagedArgBuffer::new(),
        }
    }

    pub fn storage_load_and_clear<A: BlockchainApi<'a> + StorageReadApi + StorageWriteApi>(
    ) -> Option<Self> {
        let storage_key = cb_closure_storage_key::<'a, A>();
        let storage_value_raw: ManagedBuffer<'a, A> = storage_get(storage_key.as_ref());
        if !storage_value_raw.is_empty() {
            let serializer = ManagedSerializer::<'a, A>::new();
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

    pub fn into_arg_loader(self) -> ManagedResultArgLoader<'a, M> {
        ManagedResultArgLoader::new(self.closure_args.data)
    }
}

/// Helps the callback macro expansion to perform callback name matching more efficiently.
/// The current implementation hashes by callback name length,
/// but in principle further optimizations are possible.
pub struct CallbackClosureMatcher<const CB_NAME_MAX_LENGTH: usize> {
    name_len: usize,
    compare_buffer: [u8; CB_NAME_MAX_LENGTH],
}

impl<const CB_NAME_MAX_LENGTH: usize> CallbackClosureMatcher<CB_NAME_MAX_LENGTH> {
    pub fn new<'a, M: ManagedTypeApi<'a> + ErrorApi>(callback_name: &ManagedBuffer<'a, M>) -> Self {
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
