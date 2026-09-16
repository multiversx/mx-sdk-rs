use multiversx_sc::{
    api::ManagedTypeApi,
    types::{
        EgldOrEsdtTokenIdentifier, EgldOrEsdtTokenPayment, EsdtTokenData, EsdtTokenIdentifier,
        EsdtTokenPayment, FungiblePayment, Payment, TokenId,
    },
};

use crate::{api::StaticApi, facade::result_handlers::HasUnmanaged};

macro_rules! has_unmanaged_static {
    ($ty:ident) => {
        impl<M: ManagedTypeApi> HasUnmanaged for $ty<M> {
            type Unmanaged = $ty<StaticApi>;
        }
    };
}

has_unmanaged_static!(EsdtTokenIdentifier);
has_unmanaged_static!(EgldOrEsdtTokenIdentifier);
has_unmanaged_static!(TokenId);
has_unmanaged_static!(Payment);
has_unmanaged_static!(FungiblePayment);
has_unmanaged_static!(EsdtTokenData);
has_unmanaged_static!(EsdtTokenPayment);
has_unmanaged_static!(EgldOrEsdtTokenPayment);
