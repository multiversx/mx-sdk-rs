use std::sync::Weak;

use multiversx_chain_vm::host::context::{TxContext, TxContextRef};
use multiversx_sc::{
    api::{HandleConstraints, RawHandle},
    codec::TryStaticCast,
};

use crate::executor::debug::ContractDebugStack;

#[derive(Clone)]
pub struct DebugHandle {
    /// Only keep a weak reference to the context, to avoid stray handles keeping the context from being released.
    /// Using the pointer after the context is released will panic.
    pub(crate) context: Weak<TxContext>,
    raw_handle: RawHandle,
}

impl DebugHandle {
    pub fn is_on_current_context(&self) -> bool {
        Weak::ptr_eq(
            &self.context,
            &ContractDebugStack::static_peek().tx_context_ref.downgrade(),
        )
    }

    pub fn is_on_same_context(&self, other: &DebugHandle) -> bool {
        Weak::ptr_eq(&self.context, &other.context)
    }

    pub fn assert_current_context(&self) {
        assert!(
            self.is_on_current_context(),
            "Managed value not used in original context"
        );
    }

    pub fn to_tx_context_ref(&self) -> TxContextRef {
        let tx_context_arc = self.context.upgrade().unwrap_or_else(|| {
            panic!(
                "TxContext is no longer valid for handle {}",
                self.raw_handle
            )
        });
        TxContextRef::new(tx_context_arc)
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
            context: ContractDebugStack::static_peek().tx_context_ref.downgrade(),
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
        Weak::ptr_eq(&self.context, &other.context) && self.raw_handle == other.raw_handle
    }
}

impl From<i32> for DebugHandle {
    fn from(handle: i32) -> Self {
        DebugHandle::new(handle)
    }
}

impl TryStaticCast for DebugHandle {}
