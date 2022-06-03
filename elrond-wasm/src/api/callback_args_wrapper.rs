use core::marker::PhantomData;

use crate::{
    api::{EndpointArgumentApi, ErrorApi, ManagedTypeApi, StaticVarApi, VMApi},
    types::{ManagedArgBuffer, ManagedBuffer, ManagedType},
};

use super::{const_handles, EndpointArgumentApiImpl, Handle, ManagedBufferApi};

#[derive(Clone)]
pub struct CallbackArgApiWrapper<A: VMApi> {
    _phantom: PhantomData<A>,
}

impl<A: VMApi> CallbackArgApiWrapper<A> {
    pub(super) fn new() -> Self {
        CallbackArgApiWrapper {
            _phantom: PhantomData,
        }
    }
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
        A::argument_api_impl().load_callback_closure_buffer(const_handles::MBUF_TEMPORARY_1);
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

    fn load_argument_managed_buffer(&self, arg_index: i32, dest: Handle) {
        let cb_closure_args_buffer =
            ManagedArgBuffer::<Self>::from_raw_handle(const_handles::CALLBACK_CLOSURE_ARGS_BUFFER);
        let item_buffer = cb_closure_args_buffer.get(arg_index as usize);
        A::managed_type_impl().mb_overwrite(dest, &[]);
        A::managed_type_impl().mb_append(dest, item_buffer.get_raw_handle());
    }

    fn load_callback_closure_buffer(&self, dest: Handle) {
        A::argument_api_impl().load_callback_closure_buffer(dest);
    }
}
