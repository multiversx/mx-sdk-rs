use crate::{
    contract_base::BlockchainWrapper,
    types::{BigUint, RHListItem, RHListItemExec, TxEnv},
};

/// Returns the amount of EGLD transfered.
///
/// More precisely, it returns the sum of the EGLD transfer amounts,
/// since multiple EGLD transfers are possible in a multi-transfer.
///
/// It is non-exclusive, i. e. it is possible to get other tokens alongside the EGLD.
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
            .egld_sum()
    }
}
