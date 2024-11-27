use multiversx_chain_vm::tx_mock::{TxContext, TxContextStack};
use multiversx_sc::{
    api::{use_raw_handle, HandleConstraints, RawHandle},
    codec::TryStaticCast,
    types::ManagedVecItem,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct DebugHandle {
    pub(crate) context: Arc<TxContext>,
    raw_handle: RawHandle,
}

impl DebugHandle {
    pub fn is_on_current_context(&self) -> bool {
        Arc::ptr_eq(&self.context, &TxContextStack::static_peek())
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
            context: TxContextStack::static_peek(),
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

impl ManagedVecItem for DebugHandle {
    type PAYLOAD = <RawHandle as ManagedVecItem>::PAYLOAD;

    const SKIPS_RESERIALIZATION: bool = <RawHandle as ManagedVecItem>::SKIPS_RESERIALIZATION;

    type Ref<'a> = Self;

    fn read_from_payload(payload: &Self::PAYLOAD) -> Self {
        use_raw_handle(RawHandle::read_from_payload(payload))
    }

    unsafe fn borrow_from_payload<'a>(payload: &Self::PAYLOAD) -> Self::Ref<'a> {
        Self::read_from_payload(payload)
    }

    fn into_byte_writer<R, Writer: FnMut(&[u8]) -> R>(self, writer: Writer) -> R {
        RawHandle::into_byte_writer(self.get_raw_handle(), writer)
    }

    fn save_to_payload(self, payload: &mut Self::PAYLOAD) {
        self.get_raw_handle().save_to_payload(payload);
    }
}

impl TryStaticCast for DebugHandle {}
