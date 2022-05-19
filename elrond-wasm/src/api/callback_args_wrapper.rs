use core::marker::PhantomData;

use crate::api::{EndpointArgumentApi, ErrorApi, ManagedTypeApi, StaticVarApi, VMApi};

#[derive(Clone)]
pub struct CallbackArgApiWrapper<A: VMApi> {
    _phantom: PhantomData<A>,
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
    type EndpointArgumentApiImpl = A::EndpointArgumentApiImpl;

    fn argument_api_impl() -> Self::EndpointArgumentApiImpl {
        A::argument_api_impl()
    }
}
