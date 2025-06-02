use multiversx_sc::types::{RHListItem, RHListItemExec, TxEnv};

use crate::{facade::expr::Bech32Address, scenario_model::TxResponse};

/// Indicates that the newly deployed address will be returned after a deploy.
pub struct ReturnsNewBech32Address;

impl<Env, Original> RHListItem<Env, Original> for ReturnsNewBech32Address
where
    Env: TxEnv,
{
    type Returns = Bech32Address;
}

impl<Env, Original> RHListItemExec<TxResponse, Env, Original> for ReturnsNewBech32Address
where
    Env: TxEnv,
{
    fn item_process_result(self, tx_response: &TxResponse) -> Self::Returns {
        let new_address = tx_response
            .new_deployed_address
            .clone()
            .expect("missing returned address");

        new_address.into()
    }
}
