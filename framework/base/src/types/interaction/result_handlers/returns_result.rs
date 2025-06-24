use multiversx_sc_codec::TopDecodeMulti;

use crate::{
    contract_base::SyncCallRawResult,
    types::{interaction::decode_result, DeployRawResult, RHListItem, RHListItemExec, TxEnv},
};

/// Indicates that result will be returned.
///
/// Value will be decoded according to the type defined in the smart contract.
pub struct ReturnsResult;

impl<Env, Original> RHListItem<Env, Original> for ReturnsResult
where
    Env: TxEnv,
{
    type Returns = Original;
}

impl<Env, Original> RHListItemExec<SyncCallRawResult<Env::Api>, Env, Original> for ReturnsResult
where
    Env: TxEnv,
    Original: TopDecodeMulti,
{
    fn item_process_result(self, raw_result: &SyncCallRawResult<Env::Api>) -> Original {
        decode_result::<Env::Api, Original>(raw_result.0.clone())
    }
}

impl<Env, Original> RHListItemExec<DeployRawResult<Env::Api>, Env, Original> for ReturnsResult
where
    Env: TxEnv,
    Original: TopDecodeMulti,
{
    fn item_process_result(self, raw_result: &DeployRawResult<Env::Api>) -> Original {
        decode_result::<Env::Api, Original>(raw_result.raw_results.clone())
    }
}
