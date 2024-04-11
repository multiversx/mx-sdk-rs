use core::marker::PhantomData;

use crate::api::{
    BlockchainApi, CallTypeApi, CallValueApi, CryptoApi, EndpointArgumentApi, EndpointFinishApi,
    ErrorApi, HandleTypeInfo, LogApi, ManagedTypeApi, PrintApi, SendApi, StaticVarApi,
    StorageMapperApi, StorageWriteApi, VMApi,
};

#[derive(Clone)]
pub struct ExternalViewApi<A: VMApi> {
    _phantom: PhantomData<A>,
}

impl<A: VMApi> ExternalViewApi<A> {
    pub(super) fn new() -> Self {
        ExternalViewApi {
            _phantom: PhantomData,
        }
    }
}

impl<A> HandleTypeInfo for ExternalViewApi<A>
where
    A: VMApi,
{
    type ManagedBufferHandle = <A as HandleTypeInfo>::ManagedBufferHandle;

    type BigIntHandle = <A as HandleTypeInfo>::BigIntHandle;

    type BigFloatHandle = <A as HandleTypeInfo>::BigFloatHandle;

    type EllipticCurveHandle = <A as HandleTypeInfo>::EllipticCurveHandle;

    type ManagedMapHandle = <A as HandleTypeInfo>::ManagedMapHandle;
}

impl<A> BlockchainApi for ExternalViewApi<A>
where
    A: VMApi,
{
    type BlockchainApiImpl = A::BlockchainApiImpl;

    fn blockchain_api_impl() -> A::BlockchainApiImpl {
        A::blockchain_api_impl()
    }
}

impl<A> CallValueApi for ExternalViewApi<A>
where
    A: VMApi,
{
    type CallValueApiImpl = A::CallValueApiImpl;

    fn call_value_api_impl() -> Self::CallValueApiImpl {
        A::call_value_api_impl()
    }
}

impl<A> CryptoApi for ExternalViewApi<A>
where
    A: VMApi,
{
    type CryptoApiImpl = A::CryptoApiImpl;

    fn crypto_api_impl() -> Self::CryptoApiImpl {
        A::crypto_api_impl()
    }
}

impl<A> EndpointArgumentApi for ExternalViewApi<A>
where
    A: VMApi,
{
    type EndpointArgumentApiImpl = A::EndpointArgumentApiImpl;

    fn argument_api_impl() -> Self::EndpointArgumentApiImpl {
        A::argument_api_impl()
    }
}

impl<A> EndpointFinishApi for ExternalViewApi<A>
where
    A: VMApi,
{
    type EndpointFinishApiImpl = A::EndpointFinishApiImpl;

    fn finish_api_impl() -> Self::EndpointFinishApiImpl {
        A::finish_api_impl()
    }
}

impl<A> ErrorApi for super::ExternalViewApi<A>
where
    A: VMApi,
{
    type ErrorApiImpl = A::ErrorApiImpl;

    fn error_api_impl() -> Self::ErrorApiImpl {
        A::error_api_impl()
    }
}

impl<A> LogApi for ExternalViewApi<A>
where
    A: VMApi,
{
    type LogApiImpl = A::LogApiImpl;

    fn log_api_impl() -> Self::LogApiImpl {
        A::log_api_impl()
    }
}

impl<A> ManagedTypeApi for ExternalViewApi<A>
where
    A: VMApi,
{
    type ManagedTypeApiImpl = A::ManagedTypeApiImpl;

    fn managed_type_impl() -> Self::ManagedTypeApiImpl {
        A::managed_type_impl()
    }
}

impl<A> PrintApi for ExternalViewApi<A>
where
    A: VMApi,
{
    type PrintApiImpl = A::PrintApiImpl;

    fn print_api_impl() -> Self::PrintApiImpl {
        A::print_api_impl()
    }
}

impl<A> SendApi for ExternalViewApi<A>
where
    A: VMApi,
{
    type SendApiImpl = A::SendApiImpl;

    fn send_api_impl() -> Self::SendApiImpl {
        A::send_api_impl()
    }
}

impl<A> StaticVarApi for ExternalViewApi<A>
where
    A: VMApi,
{
    type StaticVarApiImpl = A::StaticVarApiImpl;

    fn static_var_api_impl() -> Self::StaticVarApiImpl {
        A::static_var_api_impl()
    }
}

impl<A> StorageWriteApi for ExternalViewApi<A>
where
    A: VMApi,
{
    type StorageWriteApiImpl = A::StorageWriteApiImpl;

    fn storage_write_api_impl() -> Self::StorageWriteApiImpl {
        A::storage_write_api_impl()
    }
}

impl<A> CallTypeApi for ExternalViewApi<A> where A: VMApi {}

impl<A> StorageMapperApi for ExternalViewApi<A> where A: VMApi {}

impl<A> VMApi for ExternalViewApi<A>
where
    A: VMApi,
{
    fn external_view_init_override() -> bool {
        true
    }
}

impl<A> PartialEq for ExternalViewApi<A>
where
    A: VMApi,
{
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl<A> Eq for ExternalViewApi<A> where A: VMApi {}
