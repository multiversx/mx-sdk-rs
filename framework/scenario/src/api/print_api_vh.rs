use multiversx_sc::api::{uncallable::UncallableApi, PrintApi};

use super::{VMHooksApi, VMHooksBackendType};

impl<const BACKEND_TYPE: VMHooksBackendType> PrintApi for VMHooksApi<BACKEND_TYPE> {
    type PrintApiImpl = UncallableApi;

    fn print_api_impl() -> Self::PrintApiImpl {
        unreachable!()
    }
}
