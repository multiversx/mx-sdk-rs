use elrond_wasm::api::SendApi;

use crate::VmApiImpl;

impl SendApi for VmApiImpl {
    type SendApiImpl = VmApiImpl;

    fn send_api_impl() -> Self::SendApiImpl {
        VmApiImpl {}
    }
}
