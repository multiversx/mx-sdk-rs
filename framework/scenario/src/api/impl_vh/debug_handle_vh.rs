use std::sync::Arc;

use multiversx_chain_vm::tx_mock::TxContext;
use multiversx_sc::{
    api::{HandleConstraints, RawHandle},
    codec::TryStaticCast,
};

use crate::debug_executor::TxContextStack;

#[derive(Clone)]
pub struct DebugHandle {
    /// TODO: would be nice to be an actual TxContextRef,
    /// but that requires changing the debugger scripts
    pub(crate) context: Arc<TxContext>,
    raw_handle: RawHandle,
}

impl DebugHandle {
    pub fn is_on_current_context(&self) -> bool {
        Arc::ptr_eq(&self.context, &TxContextStack::static_peek().into_ref())
    }

    pub fn is_on_same_context(&self, other: &DebugHandle) -> bool {
        Arc::ptr_eq(&self.context, &other.context)
    }

    pub fn assert_current_context(&self) {
        assert!(
            self.is_on_current_context(),
            "Managed value not used in original context"
        );
    }
}

impl core::fmt::Debug for DebugHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        RawHandle::fmt(&self.raw_handle, f)
    }
}

impl HandleConstraints for DebugHandle {
    fn new(handle: multiversx_sc::api::RawHandle) -> Self {
        Self {
            context: TxContextStack::static_peek().into_ref(),
            raw_handle: handle,
        }
    }

    fn to_be_bytes(&self) -> [u8; 4] {
        self.assert_current_context();
        self.raw_handle.to_be_bytes()
    }

    fn get_raw_handle(&self) -> RawHandle {
        self.assert_current_context();
        self.raw_handle
    }

    fn get_raw_handle_unchecked(&self) -> RawHandle {
        self.raw_handle
    }
}

impl PartialEq<RawHandle> for DebugHandle {
    fn eq(&self, other: &RawHandle) -> bool {
        &self.raw_handle == other
    }
}

impl PartialEq<DebugHandle> for DebugHandle {
    fn eq(&self, other: &DebugHandle) -> bool {
        Arc::ptr_eq(&self.context, &other.context) && self.raw_handle == other.raw_handle
    }
}

impl From<i32> for DebugHandle {
    fn from(handle: i32) -> Self {
        DebugHandle::new(handle)
    }
}

impl TryStaticCast for DebugHandle {}
