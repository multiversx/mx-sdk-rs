use multiversx_sc::api::{uncallable::UncallableApi, SendApi};

use super::{VMHooksApi, VMHooksBackendType};

impl<const BACKEND_TYPE: VMHooksBackendType> SendApi for VMHooksApi<BACKEND_TYPE> {
    type SendApiImpl = UncallableApi;

    fn send_api_impl() -> Self::SendApiImpl {
        unreachable!()
    }
}
