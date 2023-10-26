use crate::{api::CallTypeApi, contract_base::BlockchainWrapper, types::ManagedAddress};

pub trait TxFrom<Api>
where
    Api: CallTypeApi,
{
    fn to_address(&self) -> ManagedAddress<Api>;
}

impl<Api> TxFrom<Api> for ()
where
    Api: CallTypeApi,
{
    fn to_address(&self) -> ManagedAddress<Api> {
        BlockchainWrapper::<Api>::new().get_sc_address()
    }
}

impl<Api> TxFrom<Api> for ManagedAddress<Api>
where
    Api: CallTypeApi,
{
    fn to_address(&self) -> ManagedAddress<Api> {
        self.clone()
    }
}
