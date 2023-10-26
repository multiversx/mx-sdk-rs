use crate::{api::ManagedTypeApi, types::BigUint};

/// Simple newtype wrapper around a BigUint value.
///
/// Its purpose is to indicate
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EgldPayment<Api>
where
    Api: ManagedTypeApi + 'static,
{
    pub value: BigUint<Api>,
}

impl<Api> From<BigUint<Api>> for EgldPayment<Api>
where
    Api: ManagedTypeApi + 'static,
{
    fn from(value: BigUint<Api>) -> Self {
        EgldPayment { value }
    }
}

impl<Api> EgldPayment<Api>
where
    Api: ManagedTypeApi + 'static,
{
    pub fn no_payment() -> Self {
        EgldPayment::from(BigUint::zero())
    }
}
