use crate::{
    api::ManagedTypeApi,
    types::{
        Address, BLSKey, BLSSignature, BoxedBytes, CodeMetadata, DurationMillis, DurationSeconds,
        EsdtLocalRole, EsdtTokenType, H256, ManagedAddress, ManagedByteArray, MessageHashType,
        OperationCompletionStatus, TimestampMillis, TimestampSeconds,
    },
};

use crate::types::HasManaged;

impl<M: ManagedTypeApi> HasManaged<M> for Address {
    type Managed = ManagedAddress<M>;
}

impl<M: ManagedTypeApi> HasManaged<M> for H256 {
    type Managed = ManagedByteArray<M, 32>;
}

macro_rules! has_managed_self {
    ($ty:ty) => {
        impl<M: ManagedTypeApi> HasManaged<M> for $ty {
            type Managed = Self;
        }
    };
}

has_managed_self!(BoxedBytes);
has_managed_self!(CodeMetadata);
has_managed_self!(BLSKey);
has_managed_self!(BLSSignature);
has_managed_self!(EsdtTokenType);
has_managed_self!(EsdtLocalRole);
has_managed_self!(DurationMillis);
has_managed_self!(DurationSeconds);
has_managed_self!(TimestampMillis);
has_managed_self!(TimestampSeconds);
has_managed_self!(MessageHashType);
has_managed_self!(OperationCompletionStatus);
