use core::fmt::Debug;

pub trait HandleTypeInfo {
    type ManagedBufferHandle: HandleConstraints;
    type BigIntHandle: HandleConstraints;
    type BigFloatHandle: HandleConstraints;
    type EllipticCurveHandle: HandleConstraints;
    type ManagedMapHandle: HandleConstraints;
}

use crate::codec::TryStaticCast;

use crate::{
    api::{ErrorApi, ErrorApiImpl},
    types::ManagedVecItem,
};

pub type RawHandle = i32;

pub trait HandleConstraints:
    ManagedVecItem + TryStaticCast + Debug + Clone + From<RawHandle> + PartialEq + PartialEq<RawHandle>
{
    fn new(handle: RawHandle) -> Self;
    fn to_be_bytes(&self) -> [u8; 4];
    fn get_raw_handle(&self) -> RawHandle;

    fn cast_or_signal_error<E: ErrorApi, U: TryStaticCast>(self) -> U {
        if let Some(other) = self.try_cast() {
            other
        } else {
            E::error_api_impl().signal_error(b"Cast type mismatch")
        }
    }

    fn get_raw_handle_unchecked(&self) -> RawHandle {
        self.get_raw_handle()
    }
}

pub fn use_raw_handle<H>(handle: RawHandle) -> H
where
    H: HandleConstraints,
{
    H::new(handle)
}

impl HandleConstraints for i32 {
    fn new(handle: RawHandle) -> Self {
        handle
    }

    fn to_be_bytes(&self) -> [u8; 4] {
        i32::to_be_bytes(*self)
    }

    fn get_raw_handle(&self) -> RawHandle {
        *self
    }
}

pub fn handle_to_be_bytes<H: HandleConstraints>(handle: H) -> [u8; 4] {
    HandleConstraints::to_be_bytes(&handle)
}
