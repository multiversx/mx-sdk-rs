use crate::{
    api::ManagedTypeApi,
    types::{
        EgldOrEsdtTokenIdentifier, EgldOrEsdtTokenPayment, EsdtTokenData, EsdtTokenIdentifier,
        EsdtTokenPayment, FungiblePayment, Payment, TokenId,
    },
};

use crate::types::HasManaged;

macro_rules! has_managed_self {
    ($ty:ident) => {
        impl<M: ManagedTypeApi> HasManaged<M> for $ty<M> {
            type Managed = Self;
        }
    };
}

has_managed_self!(EsdtTokenIdentifier);
has_managed_self!(EgldOrEsdtTokenIdentifier);
has_managed_self!(TokenId);
has_managed_self!(Payment);
has_managed_self!(FungiblePayment);
has_managed_self!(EsdtTokenData);
has_managed_self!(EsdtTokenPayment);
has_managed_self!(EgldOrEsdtTokenPayment);
