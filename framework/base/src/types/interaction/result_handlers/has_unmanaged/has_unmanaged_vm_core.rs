use crate::types::{
    Address, BLSKey, BLSSignature, BoxedBytes, CodeMetadata, DurationMillis, DurationSeconds,
    EsdtLocalRole, EsdtTokenType, H256, MessageHashType, OperationCompletionStatus,
    TimestampMillis, TimestampSeconds,
};

use crate::types::HasUnmanaged;

macro_rules! has_unmanaged_self {
    ($ty:ty) => {
        impl HasUnmanaged for $ty {
            type Unmanaged = Self;
        }
    };
}

has_unmanaged_self!(H256);
has_unmanaged_self!(Address);
has_unmanaged_self!(BoxedBytes);
has_unmanaged_self!(CodeMetadata);
has_unmanaged_self!(BLSKey);
has_unmanaged_self!(BLSSignature);
has_unmanaged_self!(EsdtTokenType);
has_unmanaged_self!(EsdtLocalRole);
has_unmanaged_self!(DurationMillis);
has_unmanaged_self!(DurationSeconds);
has_unmanaged_self!(TimestampMillis);
has_unmanaged_self!(TimestampSeconds);
has_unmanaged_self!(MessageHashType);
has_unmanaged_self!(OperationCompletionStatus);
