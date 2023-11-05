use crate::{api::CallTypeApi, contract_base::BlockchainWrapper, types::ManagedAddress};

use super::AnnotatedValue;

pub trait TxFrom<Api>
where
    Api: CallTypeApi,
{
    fn resolve_address(&self) -> ManagedAddress<Api>;
}

pub trait TxFromSpecified<Api>: TxFrom<Api> + AnnotatedValue<Api, ManagedAddress<Api>>
where
    Api: CallTypeApi,
{
}

impl<Api> TxFrom<Api> for ()
where
    Api: CallTypeApi,
{
    fn resolve_address(&self) -> ManagedAddress<Api> {
        BlockchainWrapper::<Api>::new().get_sc_address()
    }
}

impl<Api> TxFrom<Api> for ManagedAddress<Api>
where
    Api: CallTypeApi,
{
    fn resolve_address(&self) -> ManagedAddress<Api> {
        self.clone()
    }
}
impl<Api> TxFromSpecified<Api> for ManagedAddress<Api> where Api: CallTypeApi {}

impl<Api> TxFrom<Api> for &ManagedAddress<Api>
where
    Api: CallTypeApi,
{
    fn resolve_address(&self) -> ManagedAddress<Api> {
        (*self).clone()
    }
}
impl<Api> TxFromSpecified<Api> for &ManagedAddress<Api> where Api: CallTypeApi {}
