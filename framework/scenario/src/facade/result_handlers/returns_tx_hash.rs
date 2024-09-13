use multiversx_sc::types::RHListItemExec;

use crate::{
    multiversx_sc::types::{RHListItem, TxEnv},
    scenario_model::TxResponse,
};

pub struct ReturnsTxHash;

impl<Env, Original> RHListItem<Env, Original> for ReturnsTxHash
where
    Env: TxEnv,
{
    type Returns = String;
}

impl<Env, Original> RHListItemExec<TxResponse, Env, Original> for ReturnsTxHash
where
    Env: TxEnv,
{
    fn item_process_result(self, raw_result: &TxResponse) -> Self::Returns {
        raw_result.tx_hash.clone()
    }
}
