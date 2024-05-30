use crate::{
    contract_base::BlockchainWrapper,
    types::{BackTransfers, RHListItem, RHListItemExec, TxEnv},
};

/// Indicates that back-transfers will be returned.
pub struct ReturnsBackTransfers;

impl<Env, Original> RHListItem<Env, Original> for ReturnsBackTransfers
where
    Env: TxEnv,
{
    type Returns = BackTransfers<Env::Api>;
}

impl<RawResult, Env, Original> RHListItemExec<RawResult, Env, Original> for ReturnsBackTransfers
where
    Env: TxEnv,
{
    fn item_process_result(self, _raw_result: &RawResult) -> Self::Returns {
        BlockchainWrapper::<Env::Api>::new().get_back_transfers()
    }
}
