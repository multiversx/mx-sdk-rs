use crate::{
    contract_base::BlockchainWrapper,
    types::{BackTransfersLegacy, RHListItem, RHListItemExec, TxEnv},
};

/// Indicates that back-transfers will be returned, old implementation.
///
/// Back-transfers are reset before a call, to avoid unwanted interferences.
#[deprecated(
    since = "0.59.0",
    note = "Does not handle multi-transfers properly, use ReturnsBackTransfersReset instead"
)]
pub struct ReturnsBackTransfersLegacyReset;

impl<Env, Original> RHListItem<Env, Original> for ReturnsBackTransfersLegacyReset
where
    Env: TxEnv,
{
    type Returns = BackTransfersLegacy<Env::Api>;
}

impl<RawResult, Env, Original> RHListItemExec<RawResult, Env, Original>
    for ReturnsBackTransfersLegacyReset
where
    Env: TxEnv,
{
    fn item_preprocessing(&self, prev: Env::RHExpect) -> Env::RHExpect {
        BlockchainWrapper::<Env::Api>::new().reset_back_transfers();

        prev
    }

    fn item_process_result(self, _raw_result: &RawResult) -> Self::Returns {
        BlockchainWrapper::<Env::Api>::new().get_back_transfers_legacy()
    }
}
