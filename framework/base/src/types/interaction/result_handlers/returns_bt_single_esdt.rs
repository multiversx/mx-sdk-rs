use crate::{
    api::{ErrorApi, ErrorApiImpl},
    contract_base::BlockchainWrapper,
    types::{EsdtTokenPayment, RHListItem, RHListItemExec, TxEnv},
};

/// Indicates that back-transfers will be returned.
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
        let esdt_payments = BlockchainWrapper::<Env::Api>::new()
            .get_back_transfers()
            .esdt_payments;

        if esdt_payments.len() != 1 {
            Env::Api::error_api_impl().signal_error(b"Back transfers expected to be a single ESDT")
        }

        let x = esdt_payments.get(0).clone();
        x
    }
}
