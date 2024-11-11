use crate::{
    contract_base::BlockchainWrapper,
    types::{EsdtTokenPayment, ManagedVec, RHListItem, RHListItemExec, TxEnv},
};

/// Indicates that back-transfers will be returned.
pub struct ReturnsBackTransfersMultiESDT;

impl<Env, Original> RHListItem<Env, Original> for ReturnsBackTransfersMultiESDT
where
    Env: TxEnv,
{
    type Returns = ManagedVec<Env::Api, EsdtTokenPayment<Env::Api>>;
}

impl<RawResult, Env, Original> RHListItemExec<RawResult, Env, Original>
    for ReturnsBackTransfersMultiESDT
where
    Env: TxEnv,
{
    fn item_process_result(self, _raw_result: &RawResult) -> Self::Returns {
        BlockchainWrapper::<Env::Api>::new()
            .get_back_transfers()
            .esdt_payments
    }
}
