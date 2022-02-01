use crate::api::{CallTypeApi, StorageMapperApi, VMApi};

use super::UncallableApi;

impl CallTypeApi for UncallableApi {}

impl StorageMapperApi for UncallableApi {}

impl PartialEq for UncallableApi {
    fn eq(&self, _: &Self) -> bool {
        unreachable!()
    }
}

impl Eq for UncallableApi {}

impl VMApi for UncallableApi {}
