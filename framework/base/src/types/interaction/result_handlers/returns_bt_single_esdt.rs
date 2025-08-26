use crate::{
    contract_base::BlockchainWrapper,
    types::{EsdtTokenPayment, RHListItem, RHListItemExec, TxEnv},
};

/// Requests a single ESDT to be returned as back-transfer. Will fail otherwise.
pub struct ReturnsBackTransfersSingleESDT;

impl<Env, Original> RHListItem<Env, Original> for ReturnsBackTransfersSingleESDT
where
    Env: TxEnv,
{
    type Returns = EsdtTokenPayment<Env::Api>;
}

impl<RawResult, Env, Original> RHListItemExec<RawResult, Env, Original>
    for ReturnsBackTransfersSingleESDT
where
    Env: TxEnv,
{
    fn item_process_result(self, _raw_result: &RawResult) -> Self::Returns {
        BlockchainWrapper::<Env::Api>::new()
            .get_back_transfers()
            .to_single_esdt()
    }
}
