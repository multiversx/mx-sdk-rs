use crate::api::{CallTypeApi, ManagedTypeErrorApi, VMApi};

use super::UncallableApi;

// impl ManagedTypeErrorApiImpl for UncallableApi {}

impl ManagedTypeErrorApi for UncallableApi {
    type ManagedTypeErrorApiImpl = UncallableApi;

    fn managed_type_error_api() -> Self::ManagedTypeErrorApiImpl {
        UncallableApi
    }
}

impl CallTypeApi for UncallableApi {}

impl VMApi for UncallableApi {}
