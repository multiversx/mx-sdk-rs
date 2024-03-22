use crate::{
    contract_base::BlockchainWrapper, proxy_imports::TokenIdentifier, types::{
        interaction::tx_call_deploy::RHListItemDeploy, BackTransfers, ManagedAddress,
        ManagedBuffer, ManagedVec, RHListItemSync, TxEnv,
    }
};

use super::RHListItem;

pub struct ReturnsNewTokenIdentidier;

impl<Env, Original> RHListItem<Env, Original> for ReturnsNewTokenIdentidier
where
    Env: TxEnv,
{
    type Returns = TokenIdentifier<Env::Api>;
}


// impl<Env, Original> RHListItemScenario<Env, Original> for ReturnsNewTokenIdentidier
// where
//     Env: TxEnv,
// {
//     fn item_scenario_result(self, tx_response: &TxResponse) -> Self::Returns {
//         let new_token_id = tx_response
//             .new_issued_token_identifier
//             .clone()
//             .expect("missing returned token identifier");

//         new_token_id.into()
//     }
// }
