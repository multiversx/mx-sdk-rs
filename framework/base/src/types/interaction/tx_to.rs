use crate::{api::ManagedTypeApi, types::ManagedAddress};

use super::AnnotatedValue;

pub trait TxTo<Api>
where
    Api: ManagedTypeApi,
{
}

impl<Api> TxTo<Api> for () where Api: ManagedTypeApi {}

pub trait TxToSpecified<Api>: TxTo<Api> + AnnotatedValue<Api, ManagedAddress<Api>>
where
    Api: ManagedTypeApi,
{
}

impl<Api> TxTo<Api> for ManagedAddress<Api> where Api: ManagedTypeApi {}
impl<Api> TxToSpecified<Api> for ManagedAddress<Api> where Api: ManagedTypeApi {}

impl<Api> TxTo<Api> for &ManagedAddress<Api> where Api: ManagedTypeApi {}
impl<Api> TxToSpecified<Api> for &ManagedAddress<Api> where Api: ManagedTypeApi {}
