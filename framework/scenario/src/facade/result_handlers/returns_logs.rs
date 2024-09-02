use multiversx_sc::types::RHListItemExec;

use crate::{
    multiversx_sc::types::{RHListItem, TxEnv},
    scenario_model::{Log, TxResponse},
};

pub struct ReturnsLogs;

impl<Env, Original> RHListItem<Env, Original> for ReturnsLogs
where
    Env: TxEnv,
{
    type Returns = Vec<Log>;
}

impl<Env, Original> RHListItemExec<TxResponse, Env, Original> for ReturnsLogs
where
    Env: TxEnv,
{
    fn item_process_result(self, raw_result: &TxResponse) -> Self::Returns {
        raw_result.logs.clone()
    }
}
