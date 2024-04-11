use crate::api::{CallValueApi, CallValueApiImpl};

use super::UncallableApi;

impl CallValueApi for UncallableApi {
    type CallValueApiImpl = UncallableApi;

    fn call_value_api_impl() -> Self::CallValueApiImpl {
        unreachable!()
    }
}

impl CallValueApiImpl for UncallableApi {
    fn check_not_payable(&self) {
        unreachable!()
    }

    fn load_egld_value(&self, _dest: Self::BigIntHandle) {
        unreachable!()
    }

    fn load_all_esdt_transfers(&self, _dest_handle: Self::ManagedBufferHandle) {
        unreachable!()
    }

    fn esdt_num_transfers(&self) -> usize {
        unreachable!()
    }
}
