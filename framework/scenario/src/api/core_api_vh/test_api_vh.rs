use multiversx_sc::api::{TestApi, TestApiImpl};

use crate::api::{VMHooksApi, VMHooksApiBackend};

impl<VHB: VMHooksApiBackend> TestApi for VMHooksApi<VHB> {
    type TestApiImpl = Self;

    fn test_api_impl() -> Self::TestApiImpl {
        Self::api_impl()
    }
}

impl<VHB: VMHooksApiBackend> TestApiImpl for VMHooksApi<VHB> {}
