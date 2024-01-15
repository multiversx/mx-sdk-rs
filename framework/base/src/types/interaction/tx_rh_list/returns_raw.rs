use crate::types::{ManagedBuffer, ManagedVec, RHListItemSync, TxEnv};

use super::RHListItem;

pub struct ReturnsRaw;

impl<Env, Original> RHListItem<Env, Original> for ReturnsRaw
where
    Env: TxEnv,
{
    type Returns = ManagedVec<Env::Api, ManagedBuffer<Env::Api>>;
}

impl<Env, Original> RHListItemSync<Env, Original> for ReturnsRaw
where
    Env: TxEnv,
{
    fn item_sync_call_result(
        self,
        raw_results: &ManagedVec<Env::Api, ManagedBuffer<Env::Api>>,
    ) -> Self::Returns {
        raw_results.clone()
    }
}
