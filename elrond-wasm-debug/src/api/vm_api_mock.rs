use elrond_wasm::{
    api::{CallTypeApi, ManagedTypeErrorApi, VMApi},
    elrond_codec::TryStaticCast,
};

use crate::DebugApi;

impl TryStaticCast for DebugApi {}

impl ManagedTypeErrorApi for DebugApi {
    type ManagedTypeErrorApiImpl = DebugApi;

    fn managed_type_error_api() -> Self::ManagedTypeErrorApiImpl {
        DebugApi::new_from_static()
    }
}

impl CallTypeApi for DebugApi {}

impl VMApi for DebugApi {}
