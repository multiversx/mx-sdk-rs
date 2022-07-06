use std::rc::Rc;

use elrond_wasm::{
    api::{
        use_raw_handle, CallTypeApi, HandleConstraints, HandleTypeInfo, RawHandle,
        StorageMapperApi, VMApi,
    },
    elrond_codec::TryStaticCast,
    types::ManagedVecItem,
};

use crate::{
    tx_mock::{TxContext, TxContextStack},
    DebugApi,
};

impl CallTypeApi for DebugApi {}

impl StorageMapperApi for DebugApi {}

impl PartialEq for DebugApi {
    fn eq(&self, _: &Self) -> bool {
        panic!("Comparing DebugApi/TxContextRef not allowed.")
    }
}

impl Eq for DebugApi {}

impl VMApi for DebugApi {}

#[derive(Clone, Debug)]
pub struct DebugHandle {
    pub(crate) context: Rc<TxContext>,
    raw_handle: RawHandle,
}

impl DebugHandle {
    fn assert_current_context(&self) {
        assert!(Rc::ptr_eq(&self.context, &TxContextStack::static_peek()));
    }

    pub(crate) fn get_raw_handle_unchecked(&self) -> RawHandle {
        self.raw_handle
    }
}

impl HandleConstraints for DebugHandle {
    fn new(handle: elrond_wasm::api::RawHandle) -> Self {
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
}

impl PartialEq<RawHandle> for DebugHandle {
    fn eq(&self, other: &RawHandle) -> bool {
        &self.raw_handle == other
    }
}

impl PartialEq<DebugHandle> for DebugHandle {
    fn eq(&self, other: &DebugHandle) -> bool {
        Rc::ptr_eq(&self.context, &other.context) && self.raw_handle == other.raw_handle
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

impl HandleTypeInfo for DebugApi {
    type ManagedBufferHandle = DebugHandle;

    type BigIntHandle = DebugHandle;

    type BigFloatHandle = DebugHandle;

    type EllipticCurveHandle = DebugHandle;
}
