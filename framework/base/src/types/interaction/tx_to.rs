use crate::{api::ManagedTypeApi, types::ManagedAddress};

pub trait TxTo<Api>
where
    Api: ManagedTypeApi,
{
}

impl<Api> TxTo<Api> for () where Api: ManagedTypeApi {}

pub trait TxToSpecified<Api>: TxTo<Api>
where
    Api: ManagedTypeApi,
{
    fn to_address_ref(&self) -> &ManagedAddress<Api>;

    fn into_address(self) -> ManagedAddress<Api>;
}

impl<Api> TxTo<Api> for ManagedAddress<Api> where Api: ManagedTypeApi {}
impl<Api> TxToSpecified<Api> for ManagedAddress<Api>
where
    Api: ManagedTypeApi,
{
    fn to_address_ref(&self) -> &ManagedAddress<Api> {
        self
    }

    fn into_address(self) -> ManagedAddress<Api> {
        self
    }
}

impl<Api> TxTo<Api> for &ManagedAddress<Api> where Api: ManagedTypeApi {}
impl<Api> TxToSpecified<Api> for &ManagedAddress<Api>
where
    Api: ManagedTypeApi,
{
    fn to_address_ref(&self) -> &ManagedAddress<Api> {
        self
    }

    fn into_address(self) -> ManagedAddress<Api> {
        self.clone()
    }
}
