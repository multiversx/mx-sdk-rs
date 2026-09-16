use crate::{
    api::UnmanagedApi,
    types::{
        EgldOrEsdtTokenIdentifier, EgldOrEsdtTokenPayment, EsdtTokenData, EsdtTokenIdentifier,
        EsdtTokenPayment, FungiblePayment, Payment, TokenId,
    },
};

use crate::types::HasUnmanaged;

macro_rules! has_unmanaged_self {
    ($ty:ident) => {
        impl<M: UnmanagedApi> HasUnmanaged for $ty<M> {
            type Unmanaged = Self;
        }
    };
}

has_unmanaged_self!(EsdtTokenIdentifier);
has_unmanaged_self!(EgldOrEsdtTokenIdentifier);
has_unmanaged_self!(TokenId);
has_unmanaged_self!(Payment);
has_unmanaged_self!(FungiblePayment);
has_unmanaged_self!(EsdtTokenData);
has_unmanaged_self!(EsdtTokenPayment);
has_unmanaged_self!(EgldOrEsdtTokenPayment);
