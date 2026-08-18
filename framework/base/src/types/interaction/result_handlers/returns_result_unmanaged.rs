use multiversx_sc_codec::TopDecodeMulti;

use crate::{
    abi::{HasUnmanaged, TypeAbi},
    contract_base::SyncCallRawResult,
    types::{RHListItem, RHListItemExec, TxEnv, interaction::decode_result},
};

/// Indicates that the unmanaged version of the result will be returned.
pub struct ReturnsResultUnmanaged;

impl<Env, Original> RHListItem<Env, Original> for ReturnsResultUnmanaged
where
    Env: TxEnv,
    Original: TypeAbi,
    Original::Abi: HasUnmanaged,
    <Original::Abi as HasUnmanaged>::Unmanaged: TopDecodeMulti,
{
    type Returns = <Original::Abi as HasUnmanaged>::Unmanaged;
}

impl<Env, Original> RHListItemExec<SyncCallRawResult<Env::Api>, Env, Original>
    for ReturnsResultUnmanaged
where
    Env: TxEnv,
    Original: TypeAbi,
    Original::Abi: HasUnmanaged,
    <Original::Abi as HasUnmanaged>::Unmanaged: TopDecodeMulti,
{
    fn item_process_result(self, raw_result: &SyncCallRawResult<Env::Api>) -> Self::Returns {
        decode_result::<Env::Api, <Original::Abi as HasUnmanaged>::Unmanaged>(raw_result.0.clone())
    }
}
