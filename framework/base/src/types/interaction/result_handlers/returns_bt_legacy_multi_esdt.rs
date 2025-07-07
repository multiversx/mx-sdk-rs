use crate::{
    contract_base::BlockchainWrapper,
    types::{EsdtTokenPayment, ManagedVec, RHListItem, RHListItemExec, TxEnv},
};

/// Indicates that back-transfers will be returned, old implementation.
#[deprecated(
    since = "0.59.0",
    note = "Does not handle multi-transfers properly, use ReturnsBackTransfers instead"
)]
pub struct ReturnsBackTransfersLegacyMultiESDT;

impl<Env, Original> RHListItem<Env, Original> for ReturnsBackTransfersLegacyMultiESDT
where
    Env: TxEnv,
{
    type Returns = ManagedVec<Env::Api, EsdtTokenPayment<Env::Api>>;
}

impl<RawResult, Env, Original> RHListItemExec<RawResult, Env, Original>
    for ReturnsBackTransfersLegacyMultiESDT
where
    Env: TxEnv,
{
    fn item_process_result(self, _raw_result: &RawResult) -> Self::Returns {
        BlockchainWrapper::<Env::Api>::new()
            .get_back_transfers_legacy()
            .esdt_payments
    }
}
