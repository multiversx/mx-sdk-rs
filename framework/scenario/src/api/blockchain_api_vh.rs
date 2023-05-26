use multiversx_sc::api::{uncallable::UncallableApi, BlockchainApi};

use super::StaticApi;

impl BlockchainApi for StaticApi {
    type BlockchainApiImpl = UncallableApi;

    fn blockchain_api_impl() -> Self::BlockchainApiImpl {
        unreachable!()
    }
}
