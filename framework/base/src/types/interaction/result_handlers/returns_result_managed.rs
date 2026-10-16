use multiversx_sc_codec::TopDecodeMulti;

use crate::{
    contract_base::SyncCallRawResult,
    types::{DeployRawResult, RHListItem, RHListItemExec, TxEnv, interaction::decode_result},
};

use super::HasManaged;

/// Indicates that the managed version of the result will be returned, reconstructed for the
/// current environment's `ManagedTypeApi`.
///
/// The complement of [`ReturnsResultUnmanaged`](super::ReturnsResultUnmanaged): most useful
/// when `Original` is a pure ABI marker type, e.g. when calling another contract purely by its
/// ABI, with no dependency on its Rust types — `Original::Managed` gives back a proper, usable
/// managed value instead of the marker itself.
pub struct ReturnsResultManaged;

impl<Env, Original> RHListItem<Env, Original> for ReturnsResultManaged
where
    Env: TxEnv,
    Original: HasManaged<Env::Api>,
    Original::Managed: TopDecodeMulti,
{
    type Returns = Original::Managed;
}

impl<Env, Original> RHListItemExec<SyncCallRawResult<Env::Api>, Env, Original>
    for ReturnsResultManaged
where
    Env: TxEnv,
    Original: HasManaged<Env::Api>,
    Original::Managed: TopDecodeMulti,
{
    fn item_process_result(self, raw_result: &SyncCallRawResult<Env::Api>) -> Self::Returns {
        decode_result::<Env::Api, Original::Managed>(raw_result.0.clone())
    }
}

impl<Env, Original> RHListItemExec<DeployRawResult<Env::Api>, Env, Original>
    for ReturnsResultManaged
where
    Env: TxEnv,
    Original: HasManaged<Env::Api>,
    Original::Managed: TopDecodeMulti,
{
    fn item_process_result(self, raw_result: &DeployRawResult<Env::Api>) -> Self::Returns {
        decode_result::<Env::Api, Original::Managed>(raw_result.raw_results.clone())
    }
}
