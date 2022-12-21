use crate::types::LockableStaticBuffer;

use super::{HandleConstraints, HandleTypeInfo};

pub trait StaticVarApi: HandleTypeInfo {
    type StaticVarApiImpl: StaticVarApiImpl
        + HandleTypeInfo<
            ManagedBufferHandle = Self::ManagedBufferHandle,
            BigIntHandle = Self::BigIntHandle,
            BigFloatHandle = Self::BigFloatHandle,
            EllipticCurveHandle = Self::EllipticCurveHandle,
        >;

    fn static_var_api_impl() -> Self::StaticVarApiImpl;
}

/// A raw bytes buffer stored statically:
/// - in wasm as a static variable
/// - in debug mode on the thread local context
pub trait StaticVarApiImpl: HandleTypeInfo {
    fn with_lockable_static_buffer<R, F: FnOnce(&mut LockableStaticBuffer) -> R>(&self, f: F) -> R;

    fn set_external_view_target_address_handle(&self, handle: Self::ManagedBufferHandle);

    fn get_external_view_target_address_handle(&self) -> Self::ManagedBufferHandle;

    fn next_handle<H: HandleConstraints>(&self) -> H;

    fn set_num_arguments(&self, num_arguments: i32);

    fn get_num_arguments(&self) -> i32;

    fn set_call_value_egld_handle(&self, handle: Self::BigIntHandle);

    fn get_call_value_egld_handle(&self) -> Self::BigIntHandle;

    fn set_call_value_multi_esdt_handle(&self, handle: Self::ManagedBufferHandle);

    fn get_call_value_multi_esdt_handle(&self) -> Self::ManagedBufferHandle;
}
