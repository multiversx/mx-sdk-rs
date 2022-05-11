use core::marker::PhantomData;

use crate::{
    api::{BlockchainApi, ManagedTypeApi, StorageReadApi, StorageWriteApi},
    io::BytesArgLoader,
    storage_clear, storage_get, storage_get_len,
    types::{heap::BoxedBytes, ManagedBuffer, ManagedType},
};
use alloc::vec::Vec;
use elrond_codec::{DecodeErrorHandler, NestedDecode, TopDecode, TopDecodeInput};

use super::CallbackClosureMatcher;

/// Temporary solution until gas costs of the managed version are reduced.
/// Only contains logic for deserializing and for being used in the callback handling macros.
pub struct CallbackClosureUnmanagedArgs<M: ManagedTypeApi> {
    callback_name: BoxedBytes,
    closure_args: Vec<BoxedBytes>,
    _phantom: PhantomData<M>,
}

impl<M: ManagedTypeApi> CallbackClosureUnmanagedArgs<M> {
    /// Used by callback_raw.
    pub fn new_empty() -> Self {
        CallbackClosureUnmanagedArgs {
            callback_name: BoxedBytes::empty(),
            closure_args: Vec::new(),
            _phantom: PhantomData,
        }
    }

    pub fn storage_load_and_clear<A: BlockchainApi + StorageReadApi + StorageWriteApi>(
    ) -> Option<Self> {
        let storage_key = super::callback_closure::cb_closure_storage_key::<A>();
        if storage_get_len(storage_key.as_ref()) > 0 {
            let closure = storage_get(storage_key.as_ref());
            storage_clear(storage_key.as_ref());
            Some(closure)
        } else {
            None
        }
    }

    pub fn matcher<const CB_NAME_MAX_LENGTH: usize>(
        &self,
    ) -> CallbackClosureMatcher<CB_NAME_MAX_LENGTH> {
        CallbackClosureMatcher::new_from_unmanaged(self.callback_name.as_slice())
    }

    pub fn into_arg_loader(self) -> BytesArgLoader<M> {
        BytesArgLoader::new(self.closure_args)
    }
}

impl<M: ManagedTypeApi> TopDecode for CallbackClosureUnmanagedArgs<M> {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        let managed_buffer: ManagedBuffer<M> = ManagedBuffer::top_decode_or_handle_err(input, h)?;
        let bytes_buffer = managed_buffer.to_boxed_bytes();
        let mut bytes_slice = bytes_buffer.as_slice();

        let callback_name = BoxedBytes::dep_decode_or_handle_err(&mut bytes_slice, h)?;
        let closure_args = Vec::<BoxedBytes>::dep_decode_or_handle_err(&mut bytes_slice, h)?;
        Ok(CallbackClosureUnmanagedArgs {
            callback_name,
            closure_args,
            _phantom: PhantomData,
        })
    }
}
