use crate::{
    contract_base::BlockchainWrapper,
    proxy_imports::TokenIdentifier,
    types::{BackTransfers, ManagedAddress, ManagedBuffer, ManagedVec, RHListItem, TxEnv},
};

pub struct ReturnsNewTokenIdentifier;

impl<Env, Original> RHListItem<Env, Original> for ReturnsNewTokenIdentifier
where
    Env: TxEnv,
{
    type Returns = TokenIdentifier<Env::Api>;
}
