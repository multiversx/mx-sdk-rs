use crate::api::ErrorApi;

use super::{HandleTypeInfo, ManagedTypeApiImpl, StaticVarApi};

pub trait ManagedTypeApi: HandleTypeInfo + StaticVarApi + ErrorApi + Clone + 'static {
    type ManagedTypeApiImpl: ManagedTypeApiImpl
        + HandleTypeInfo<
            ManagedBufferHandle = Self::ManagedBufferHandle,
            BigIntHandle = Self::BigIntHandle,
            BigFloatHandle = Self::BigFloatHandle,
            EllipticCurveHandle = Self::EllipticCurveHandle,
            ManagedMapHandle = Self::ManagedMapHandle,
        >;

    fn managed_type_impl() -> Self::ManagedTypeApiImpl;
}
