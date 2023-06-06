use multiversx_sc::api::{uncallable::UncallableApi, BlockchainApi};

use super::{VMHooksApi, VMHooksBackendType};

impl<const BACKEND_TYPE: VMHooksBackendType> BlockchainApi for VMHooksApi<BACKEND_TYPE> {
    type BlockchainApiImpl = UncallableApi;

    fn blockchain_api_impl() -> Self::BlockchainApiImpl {
        unreachable!()
    }
}
