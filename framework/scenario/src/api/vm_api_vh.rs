use multiversx_sc::api::{CallTypeApi, StorageMapperApi, VMApi};

use crate::api::{VMHooksApi, VMHooksBackendType};

impl<const BACKEND_TYPE: VMHooksBackendType> CallTypeApi for VMHooksApi<BACKEND_TYPE> {}

impl<const BACKEND_TYPE: VMHooksBackendType> StorageMapperApi for VMHooksApi<BACKEND_TYPE> {}

impl<const BACKEND_TYPE: VMHooksBackendType> VMApi for VMHooksApi<BACKEND_TYPE> {}
