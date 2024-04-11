use multiversx_sc::api::{CallTypeApi, StorageMapperApi<'a>, VMApi};

use crate::api::{VMHooksApi, VMHooksApiBackend};

impl<VHB: VMHooksApiBackend> CallTypeApi for VMHooksApi<VHB> {}

impl<VHB: VMHooksApiBackend> StorageMapperApi for VMHooksApi<VHB> {}

impl<VHB: VMHooksApiBackend> VMApi for VMHooksApi<VHB> {}
