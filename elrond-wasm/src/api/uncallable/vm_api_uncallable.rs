use crate::api::{CallTypeApi, StorageMapperApi, VMApi};

use super::UncallableApi;

impl CallTypeApi for UncallableApi {}

impl StorageMapperApi for UncallableApi {}

impl VMApi for UncallableApi {}
