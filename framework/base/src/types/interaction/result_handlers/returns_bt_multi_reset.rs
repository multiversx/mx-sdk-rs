use crate::{
    contract_base::BlockchainWrapper,
    types::{EgldOrEsdtTokenPayment, ManagedVec, RHListItem, RHListItemExec, TxEnv},
};

/// Indicates that all back-transfers, as list of EGLD and ESDT payments.
pub struct ReturnsBackTransfersMultiReset;

impl<Env, Original> RHListItem<Env, Original> for ReturnsBackTransfersMultiReset
where
    Env: TxEnv,
{
    type Returns = ManagedVec<Env::Api, EgldOrEsdtTokenPayment<Env::Api>>;
}

impl<RawResult, Env, Original> RHListItemExec<RawResult, Env, Original>
    for ReturnsBackTransfersMultiReset
where
    Env: TxEnv,
{
    fn item_preprocessing(&self, prev: Env::RHExpect) -> Env::RHExpect {
        BlockchainWrapper::<Env::Api>::new().reset_back_transfers();

        prev
    }

    fn item_process_result(self, _raw_result: &RawResult) -> Self::Returns {
        BlockchainWrapper::<Env::Api>::new().get_back_transfers_multi()
    }
}
