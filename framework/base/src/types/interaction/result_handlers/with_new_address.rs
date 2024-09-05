use core::marker::PhantomData;

use crate::types::{DeployRawResult, ManagedAddress, RHListItem, RHListItemExec, TxEnv};

/// Defines a lambda function to be called on the newly deployed address, after a deploy.
pub struct WithNewAddress<Env, F>
where
    Env: TxEnv,
    F: FnOnce(&ManagedAddress<Env::Api>),
{
    _phantom: PhantomData<Env>,
    pub f: F,
}

impl<Env, F> WithNewAddress<Env, F>
where
    Env: TxEnv,
    F: FnOnce(&ManagedAddress<Env::Api>),
{
    pub fn new(f: F) -> Self {
        WithNewAddress {
            _phantom: PhantomData,
            f,
        }
    }
}

impl<Env, F, Original> RHListItem<Env, Original> for WithNewAddress<Env, F>
where
    Env: TxEnv,
    F: FnOnce(&ManagedAddress<Env::Api>),
{
    type Returns = ();
}

impl<Env, F, Original> RHListItemExec<DeployRawResult<Env::Api>, Env, Original>
    for WithNewAddress<Env, F>
where
    Env: TxEnv,
    F: FnOnce(&ManagedAddress<Env::Api>),
{
    fn item_process_result(self, raw_result: &DeployRawResult<Env::Api>) -> Self::Returns {
        (self.f)(&raw_result.new_address);
    }
}
