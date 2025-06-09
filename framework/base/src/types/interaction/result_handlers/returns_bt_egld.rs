use crate::{
    contract_base::BlockchainWrapper,
    types::{BigUint, RHListItem, RHListItemExec, TxEnv},
};

/// Indicates that back-transfers will be returned.
pub struct ReturnsBackTransfersEGLD;

impl<Env, Original> RHListItem<Env, Original> for ReturnsBackTransfersEGLD
where
    Env: TxEnv,
{
    type Returns = BigUint<Env::Api>;
}

impl<RawResult, Env, Original> RHListItemExec<RawResult, Env, Original> for ReturnsBackTransfersEGLD
where
    Env: TxEnv,
{
    fn item_process_result(self, _raw_result: &RawResult) -> Self::Returns {
        BlockchainWrapper::<Env::Api>::new()
            .get_back_transfers()
            .total_egld_amount
    }
}
