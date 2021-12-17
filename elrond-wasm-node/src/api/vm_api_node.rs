use elrond_wasm::{
    api::{CallTypeApi, ManagedTypeErrorApi, VMApi},
    elrond_codec::TryStaticCast,
};

/// The reference to the API implementation based on Arwen hooks.
/// It continas no data, can be embedded at no cost.
/// Cloning it is a no-op.
pub struct VmApiImpl {}

impl TryStaticCast for VmApiImpl {}

impl ManagedTypeErrorApi for VmApiImpl {
    type ManagedTypeErrorApiImpl = VmApiImpl;

    fn managed_type_error_api() -> Self::ManagedTypeErrorApiImpl {
        VmApiImpl {}
    }
}

impl CallTypeApi for VmApiImpl {}

impl VMApi for VmApiImpl {}

/// Should be no-op. The API implementation is zero-sized.
impl Clone for VmApiImpl {
    #[inline]
    fn clone(&self) -> Self {
        VmApiImpl {}
    }
}
