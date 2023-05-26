use multiversx_sc::api::{uncallable::UncallableApi, StorageReadApi, StorageWriteApi};

use super::StaticApi;

impl StorageReadApi for StaticApi {
    type StorageReadApiImpl = UncallableApi;

    fn storage_read_api_impl() -> Self::StorageReadApiImpl {
        unreachable!()
    }
}

impl StorageWriteApi for StaticApi {
    type StorageWriteApiImpl = UncallableApi;

    fn storage_write_api_impl() -> Self::StorageWriteApiImpl {
        unreachable!()
    }
}
