use crate::{
    contract_base::BlockchainWrapper,
    types::{BackTransfers, ManagedBuffer, ManagedVec, RHListItemSync, TxEnv},
};

use super::RHListItem;

pub struct ReturnsBackTransfers;

impl<Env, Original> RHListItem<Env, Original> for ReturnsBackTransfers
where
    Env: TxEnv,
{
    type Returns = BackTransfers<Env::Api>;
}

impl<Env, Original> RHListItemSync<Env, Original> for ReturnsBackTransfers
where
    Env: TxEnv,
{
    fn item_sync_call_result(
        self,
        _raw_results: &ManagedVec<Env::Api, ManagedBuffer<Env::Api>>,
    ) -> Self::Returns {
        BlockchainWrapper::<Env::Api>::new().get_back_transfers()
    }
}
