use multiversx_sc::api::{uncallable::UncallableApi, StorageReadApi, StorageWriteApi};

use super::{VMHooksApi, VMHooksBackendType};

impl<const BACKEND_TYPE: VMHooksBackendType> StorageReadApi for VMHooksApi<BACKEND_TYPE> {
    type StorageReadApiImpl = UncallableApi;

    fn storage_read_api_impl() -> Self::StorageReadApiImpl {
        unreachable!()
    }
}

impl<const BACKEND_TYPE: VMHooksBackendType> StorageWriteApi for VMHooksApi<BACKEND_TYPE> {
    type StorageWriteApiImpl = UncallableApi;

    fn storage_write_api_impl() -> Self::StorageWriteApiImpl {
        unreachable!()
    }
}
