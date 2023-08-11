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
    const PAYLOAD_SIZE: usize = <RawHandle as ManagedVecItem>::PAYLOAD_SIZE;

    const SKIPS_RESERIALIZATION: bool = <RawHandle as ManagedVecItem>::SKIPS_RESERIALIZATION;

    type Ref<'a> = Self;

    fn from_byte_reader<Reader: FnMut(&mut [u8])>(reader: Reader) -> Self {
        use_raw_handle(RawHandle::from_byte_reader(reader))
    }

    unsafe fn from_byte_reader_as_borrow<'a, Reader: FnMut(&mut [u8])>(
        reader: Reader,
    ) -> Self::Ref<'a> {
        use_raw_handle(RawHandle::from_byte_reader(reader))
    }

    fn to_byte_writer<R, Writer: FnMut(&[u8]) -> R>(&self, writer: Writer) -> R {
        RawHandle::to_byte_writer(&self.get_raw_handle(), writer)
    }
}

impl TryStaticCast for DebugHandle {}
