use core::fmt::Debug;

pub trait HandleTypeInfo {
    type ManagedBufferHandle: HandleConstraints;
    type BigIntHandle: HandleConstraints;
    type BigFloatHandle: HandleConstraints;
    type EllipticCurveHandle: HandleConstraints;
}

use elrond_codec::TryStaticCast;

use crate::types::ManagedVecItem;

pub type RawHandle = i32;

pub trait HandleConstraints:
    ManagedVecItem + TryStaticCast + Debug + Copy + From<i32> + Eq + PartialEq<i32>
{
    fn new(handle: RawHandle) -> Self;
    fn to_be_bytes(&self) -> [u8; 4];
    fn get_raw_handle(&self) -> RawHandle;
}

pub fn use_raw_handle<H>(handle: RawHandle) -> H
where
    H: HandleConstraints,
{
    H::new(handle)
}

impl HandleConstraints for i32 {
    fn new(handle: RawHandle) -> Self {
        handle as i32
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
