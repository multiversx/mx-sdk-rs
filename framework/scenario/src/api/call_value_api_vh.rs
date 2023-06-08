use multiversx_sc::api::{uncallable::UncallableApi, CallValueApi};

use super::{VMHooksApi, VMHooksBackendType};

impl<const BACKEND_TYPE: VMHooksBackendType> CallValueApi for VMHooksApi<BACKEND_TYPE> {
    type CallValueApiImpl = UncallableApi;

    fn call_value_api_impl() -> Self::CallValueApiImpl {
        unreachable!()
    }
}
