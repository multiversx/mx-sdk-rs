use multiversx_sc_codec::TopDecodeMulti;

use crate::{
    proxy_imports::SyncCallRawResult,
    types::{
        interaction::contract_call_exec::decode_result, ManagedBuffer, ManagedVec, RHListItem,
        RHListItemExec, TxEnv,
    },
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
