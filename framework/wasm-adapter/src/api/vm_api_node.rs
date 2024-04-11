use multiversx_sc::api::{CallTypeApi, HandleTypeInfo, StorageMapperApi, VMApi};

/// The reference to the API implementation based on Arwen hooks.
/// It continas no data, can be embedded at no cost.
/// Cloning it is a no-op.
pub struct VmApiImpl {}

impl CallTypeApi for VmApiImpl {}

impl StorageMapperApi for VmApiImpl {}

/// Should be no-op. The API implementation is zero-sized.
impl Clone for VmApiImpl {
    #[inline]
    fn clone(&self) -> Self {
        VmApiImpl {}
    }
}

impl PartialEq for VmApiImpl {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl Eq for VmApiImpl {}

impl VMApi for VmApiImpl {}

impl HandleTypeInfo for VmApiImpl {
    type ManagedBufferHandle = i32;

    type BigIntHandle = i32;

    type BigFloatHandle = i32;

    type EllipticCurveHandle = i32;

    type ManagedMapHandle = i32;
}
