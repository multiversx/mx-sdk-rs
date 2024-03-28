use multiversx_sc::types::RHListItemExec;

use crate::{
    multiversx_sc::types::{RHListItem, TxEnv},
    scenario_model::TxResponse,
};

pub struct ReturnsNewTokenIdentifier;

impl<Env, Original> RHListItem<Env, Original> for ReturnsNewTokenIdentifier
where
    Env: TxEnv,
{
    type Returns = String;
}

impl<Env, Original> RHListItemExec<TxResponse, Env, Original> for ReturnsNewTokenIdentifier
where
    Env: TxEnv,
{
    fn item_process_result(self, raw_result: &TxResponse) -> Self::Returns {
        raw_result
            .new_issued_token_identifier
            .clone()
            .expect("missing returned token identifier")
    }
}
