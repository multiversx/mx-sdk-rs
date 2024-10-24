use multiversx_chain_vm::types::H256;
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
    type Returns = H256;
}

impl<Env, Original> RHListItemExec<TxResponse, Env, Original> for ReturnsTxHash
where
    Env: TxEnv,
{
    fn item_process_result(self, raw_result: &TxResponse) -> Self::Returns {
        raw_result.tx_hash.clone().expect("missing tx hash")
    }
}
