use core::marker::PhantomData;

use crate::{
    api::{
        const_handles, use_raw_handle, EndpointArgumentApi, EndpointArgumentApiImpl, ErrorApi,
        HandleTypeInfo, ManagedBufferApiImpl, ManagedTypeApi, StaticVarApi, VMApi,
    },
    types::{ManagedArgBuffer, ManagedBuffer, ManagedType},
};

/// Replaces the EndpointArgumentApi inside a promises callback,
/// and causes it to read arguments from the callback data instead of the regular tx input.
#[derive(Clone, Default)]
pub struct CallbackArgApiWrapper<A: VMApi> {
    _phantom: PhantomData<A>,
}

impl<A: VMApi> CallbackArgApiWrapper<A> {
    pub fn new() -> Self {
        CallbackArgApiWrapper {
            _phantom: PhantomData,
        }
    }
}

impl<A> HandleTypeInfo for CallbackArgApiWrapper<A>
where
    A: VMApi,
{
    type ManagedBufferHandle = <A as HandleTypeInfo>::ManagedBufferHandle;

    type BigIntHandle = <A as HandleTypeInfo>::BigIntHandle;

    type BigFloatHandle = <A as HandleTypeInfo>::BigFloatHandle;

    type EllipticCurveHandle = <A as HandleTypeInfo>::EllipticCurveHandle;

    type ManagedMapHandle = <A as HandleTypeInfo>::ManagedMapHandle;
}

impl<A: VMApi> ErrorApi for CallbackArgApiWrapper<A> {
    type ErrorApiImpl = A::ErrorApiImpl;

    fn error_api_impl() -> Self::ErrorApiImpl {
        A::error_api_impl()
    }
}

impl<A: VMApi> StaticVarApi for CallbackArgApiWrapper<A> {
    type StaticVarApiImpl = A::StaticVarApiImpl;

    fn static_var_api_impl() -> Self::StaticVarApiImpl {
        A::static_var_api_impl()
    }
}

impl<A: VMApi> ManagedTypeApi for CallbackArgApiWrapper<A> {
    type ManagedTypeApiImpl = A::ManagedTypeApiImpl;

    fn managed_type_impl() -> Self::ManagedTypeApiImpl {
        A::managed_type_impl()
    }
}

impl<A: VMApi> EndpointArgumentApi for CallbackArgApiWrapper<A> {
    type EndpointArgumentApiImpl = Self;

    fn argument_api_impl() -> Self::EndpointArgumentApiImpl {
        Self::new()
    }
}

impl<A: VMApi> EndpointArgumentApiImpl for CallbackArgApiWrapper<A> {
    fn endpoint_init(&self) {
        A::argument_api_impl()
            .load_callback_closure_buffer(use_raw_handle(const_handles::MBUF_TEMPORARY_1));
        let cb_closure_args_serialized =
            ManagedBuffer::<A>::from_raw_handle(const_handles::MBUF_TEMPORARY_1);
        let mut cb_closure_args_buffer =
            ManagedArgBuffer::<A>::from_raw_handle(const_handles::CALLBACK_CLOSURE_ARGS_BUFFER);
        cb_closure_args_buffer.deserialize_overwrite(cb_closure_args_serialized);
    }

    fn get_num_arguments(&self) -> i32 {
        ManagedArgBuffer::<Self>::from_raw_handle(const_handles::CALLBACK_CLOSURE_ARGS_BUFFER).len()
            as i32
    }

    fn load_argument_managed_buffer(&self, arg_index: i32, dest: Self::ManagedBufferHandle) {
        let cb_closure_args_buffer =
            ManagedArgBuffer::<Self>::from_raw_handle(const_handles::CALLBACK_CLOSURE_ARGS_BUFFER);
        let item_buffer = cb_closure_args_buffer.get(arg_index as usize);
        A::managed_type_impl().mb_overwrite(dest.clone(), &[]);
        A::managed_type_impl().mb_append(dest, item_buffer.get_handle());
    }

    fn load_callback_closure_buffer(&self, dest: Self::ManagedBufferHandle) {
        A::argument_api_impl().load_callback_closure_buffer(dest);
    }
}
