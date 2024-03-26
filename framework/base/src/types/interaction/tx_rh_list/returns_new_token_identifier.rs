use crate::{
    contract_base::BlockchainWrapper,
    proxy_imports::TokenIdentifier,
    types::{
        interaction::tx_call_deploy::RHListItemDeploy, BackTransfers, ManagedAddress,
        ManagedBuffer, ManagedVec, RHListItem, RHListItemSync, TxEnv,
    },
};

pub struct ReturnsNewTokenIdentifier;

impl<Env, Original> RHListItem<Env, Original> for ReturnsNewTokenIdentifier
where
    Env: TxEnv,
{
    type Returns = TokenIdentifier<Env::Api>;
}
