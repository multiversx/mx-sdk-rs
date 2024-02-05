use crate::{
    contract_base::BlockchainWrapper,
    types::{
        interaction::tx_call_deploy::RHListItemDeploy, BackTransfers, ManagedAddress,
        ManagedBuffer, ManagedVec, RHListItemSync, TxEnv,
    },
};

use super::RHListItem;

pub struct ReturnsNewAddress;

impl<Env, Original> RHListItem<Env, Original> for ReturnsNewAddress
where
    Env: TxEnv,
{
    type Returns = ManagedAddress<Env::Api>;
}

impl<Env, Original> RHListItemDeploy<Env, Original> for ReturnsNewAddress
where
    Env: TxEnv,
{
    fn item_deploy_result(
        self,
        new_address: &ManagedAddress<Env::Api>,
        _raw_results: &ManagedVec<Env::Api, ManagedBuffer<Env::Api>>,
    ) -> Self::Returns {
        new_address.clone()
    }
}
