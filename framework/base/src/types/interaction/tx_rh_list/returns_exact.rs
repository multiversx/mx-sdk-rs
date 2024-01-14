use multiversx_sc_codec::TopDecodeMulti;

use crate::types::{
    interaction::contract_call_exec::decode_result, ManagedBuffer, ManagedVec, RHListItemSync,
    TxEnv,
};

use super::RHListItem;

pub struct ReturnsExact;

impl<Env, Original> RHListItem<Env, Original> for ReturnsExact
where
    Env: TxEnv,
{
    type Returns = Original;
}

impl<Env, Original> RHListItemSync<Env, Original> for ReturnsExact
where
    Env: TxEnv,
    Original: TopDecodeMulti,
{
    fn item_sync_call_result(
        self,
        raw_results: &ManagedVec<Env::Api, ManagedBuffer<Env::Api>>,
    ) -> Self::Returns {
        decode_result::<Env::Api, Original>(raw_results.clone())
    }
}

// impl<Env> TxResultHandler<Env> for ReturnRaw where Env: TxEnv {}

// impl<Env> TxReturn<Env> for ReturnRaw
// where
//     Env: TxEnv,
// {
//     type Returned = ManagedVec<Env::Api, ManagedBuffer<Env::Api>>;

//     fn sync_call_result(
//         self,
//         raw_results: &ManagedVec<Env::Api, ManagedBuffer<Env::Api>>,
//     ) -> Self::Returned {
//         raw_results.clone()
//     }
// }
