use crate::{
    api::{BlockchainApi, CallTypeApi},
    contract_base::BlockchainWrapper,
    types::{AnnotatedValue, ManagedAddress, ManagedBuffer, TxScEnv},
};

use super::{TxTo, TxToSpecified};

/// Indicates that transaction should be sent to the caller (the sender of the current transaction).
pub struct ToCaller;

fn get_caller<Api>() -> ManagedAddress<Api>
where
    Api: CallTypeApi + BlockchainApi,
{
    BlockchainWrapper::<Api>::new().get_caller()
}

impl<Api> AnnotatedValue<TxScEnv<Api>, ManagedAddress<Api>> for ToCaller
where
    Api: CallTypeApi + BlockchainApi,
{
    fn annotation(&self, env: &TxScEnv<Api>) -> ManagedBuffer<Api> {
        get_caller::<Api>().hex_expr()
    }

    fn into_value(self, _env: &TxScEnv<Api>) -> ManagedAddress<Api> {
        get_caller::<Api>()
    }
}

impl<Api> TxTo<TxScEnv<Api>> for ToCaller where Api: CallTypeApi + BlockchainApi {}
impl<Api> TxToSpecified<TxScEnv<Api>> for ToCaller
where
    Api: CallTypeApi + BlockchainApi,
{
    fn with_address_ref<F, R>(&self, env: &TxScEnv<Api>, f: F) -> R
    where
        F: FnOnce(&ManagedAddress<Api>) -> R,
    {
        f(&get_caller::<Api>())
    }
}
