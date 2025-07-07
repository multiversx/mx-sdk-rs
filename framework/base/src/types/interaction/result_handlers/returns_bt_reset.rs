use crate::{
    contract_base::BlockchainWrapper,
    types::{BackTransfers, RHListItem, RHListItemExec, TxEnv},
};

/// Indicates that all back-transfers, as list of EGLD and ESDT payments.
///
/// Back-transfers are reset before a call, to avoid unwanted interferences.
pub struct ReturnsBackTransfersReset;

impl<Env, Original> RHListItem<Env, Original> for ReturnsBackTransfersReset
where
    Env: TxEnv,
{
    type Returns = BackTransfers<Env::Api>;
}

impl<RawResult, Env, Original> RHListItemExec<RawResult, Env, Original>
    for ReturnsBackTransfersReset
where
    Env: TxEnv,
{
    fn item_preprocessing(&self, prev: Env::RHExpect) -> Env::RHExpect {
        BlockchainWrapper::<Env::Api>::new().reset_back_transfers();

        prev
    }

    fn item_process_result(self, _raw_result: &RawResult) -> Self::Returns {
        BlockchainWrapper::<Env::Api>::new().get_back_transfers()
    }
}
