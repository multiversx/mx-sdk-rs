use elrond_wasm::{
    api::{Handle, ManagedBufferApi},
    types::BoxedBytes,
};

use crate::TxContext;

impl ManagedBufferApi for TxContext {
    fn new_empty(&self) -> Handle {
        unreachable!()
    }

    fn new_from_bytes(&self, _bytes: &[u8]) -> Handle {
        unreachable!()
    }

    fn len(&self, _handle: Handle) -> usize {
        unreachable!()
    }

    fn overwrite(&self, _handle: Handle, _value: &[u8]) {
        unreachable!()
    }

    fn extend_from_slice(&self, _handle: Handle, _slice: &[u8]) {
        unreachable!()
    }

    fn to_boxed_bytes(&self, _handle: Handle) -> BoxedBytes {
        unreachable!()
    }
}
