use core::marker::PhantomData;

use crate::types::{
    interaction::tx_call_deploy::RHListItemDeploy, ManagedAddress, ManagedBuffer, ManagedVec,
    RHListItemSync, TxEnv,
};

use super::RHListItem;

pub struct WithResultNewAddress<Env, F>
where
    Env: TxEnv,
    F: FnOnce(&ManagedAddress<Env::Api>),
{
    _phantom: PhantomData<Env>,
    pub f: F,
}

impl<Env, F> WithResultNewAddress<Env, F>
where
    Env: TxEnv,
    F: FnOnce(&ManagedAddress<Env::Api>),
{
    pub fn new(f: F) -> Self {
        WithResultNewAddress {
            _phantom: PhantomData,
            f,
        }
    }
}

impl<Env, F, Original> RHListItem<Env, Original> for WithResultNewAddress<Env, F>
where
    Env: TxEnv,
    F: FnOnce(&ManagedAddress<Env::Api>),
{
    type Returns = ();
}

impl<Env, F, Original> RHListItemDeploy<Env, Original> for WithResultNewAddress<Env, F>
where
    Env: TxEnv,
    F: FnOnce(&ManagedAddress<Env::Api>),
{
    fn item_deploy_result(
        self,
        new_address: &ManagedAddress<Env::Api>,
        _raw_results: &ManagedVec<Env::Api, ManagedBuffer<Env::Api>>,
    ) -> Self::Returns {
        (self.f)(new_address);
    }
}
