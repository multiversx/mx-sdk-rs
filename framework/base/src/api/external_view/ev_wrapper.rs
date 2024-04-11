use core::marker::PhantomData;

use crate::api::{
    BlockchainApi, CallTypeApi, CallValueApi, CryptoApi, EndpointArgumentApi, EndpointFinishApi,
    ErrorApi, HandleTypeInfo, LogApi, ManagedTypeApi, PrintApi, SendApi, StaticVarApi,
    StorageMapperApi, StorageWriteApi, VMApi,
};

#[derive(Clone)]
pub struct ExternalViewApi<'a, A: VMApi<'a>> {
    _phantom: PhantomData<&'a A>,
}

impl<'a, A: VMApi<'a>> ExternalViewApi<'a, A> {
    pub(super) fn new() -> Self {
        ExternalViewApi {
            _phantom: PhantomData
        }
    }
}

impl<'a, A> HandleTypeInfo for ExternalViewApi<'a, A>
where
    A: VMApi<'a>,
{
    type ManagedBufferHandle = <A as HandleTypeInfo>::ManagedBufferHandle;

    type BigIntHandle = <A as HandleTypeInfo>::BigIntHandle;

    type BigFloatHandle = <A as HandleTypeInfo>::BigFloatHandle;

    type EllipticCurveHandle = <A as HandleTypeInfo>::EllipticCurveHandle;

    type ManagedMapHandle = <A as HandleTypeInfo>::ManagedMapHandle;
}

impl<'a, A> BlockchainApi<'a> for ExternalViewApi<'a, A>
where
    A: VMApi<'a>,
{
    type BlockchainApiImpl = A::BlockchainApiImpl;

    fn blockchain_api_impl() -> A::BlockchainApiImpl {
        A::blockchain_api_impl()
    }
}

impl<'a, A> CallValueApi<'a> for ExternalViewApi<'a, A>
where
    A: VMApi<'a>,
{
    type CallValueApiImpl = A::CallValueApiImpl;

    fn call_value_api_impl() -> Self::CallValueApiImpl {
        A::call_value_api_impl()
    }
}

impl<'a, A> CryptoApi<'a> for ExternalViewApi<'a, A>
where
    A: VMApi<'a>,
{
    type CryptoApiImpl = A::CryptoApiImpl;

    fn crypto_api_impl() -> Self::CryptoApiImpl {
        A::crypto_api_impl()
    }
}

impl<'a, A> EndpointArgumentApi<'a> for ExternalViewApi<'a, A>
where
    A: VMApi<'a>,
{
    type EndpointArgumentApiImpl = A::EndpointArgumentApiImpl;

    fn argument_api_impl() -> Self::EndpointArgumentApiImpl {
        A::argument_api_impl()
    }
}

impl<'a, A> EndpointFinishApi for ExternalViewApi<'a, A>
where
    A: VMApi<'a>,
{
    type EndpointFinishApiImpl = A::EndpointFinishApiImpl;

    fn finish_api_impl() -> Self::EndpointFinishApiImpl {
        A::finish_api_impl()
    }
}

impl<'a, A> ErrorApi for super::ExternalViewApi<'a, A>
where
    A: VMApi<'a>,
{
    type ErrorApiImpl = A::ErrorApiImpl;

    fn error_api_impl() -> Self::ErrorApiImpl {
        A::error_api_impl()
    }
}

impl<'a, A> LogApi for ExternalViewApi<'a, A>
where
    A: VMApi<'a>,
{
    type LogApiImpl = A::LogApiImpl;

    fn log_api_impl() -> Self::LogApiImpl {
        A::log_api_impl()
    }
}

impl<'a, A> ManagedTypeApi<'a> for ExternalViewApi<'a, A>
where
    A: VMApi<'a>,
{
    type ManagedTypeApiImpl = A::ManagedTypeApiImpl;

    fn managed_type_impl() -> Self::ManagedTypeApiImpl {
        A::managed_type_impl()
    }
}

impl<'a, A> PrintApi<'a> for ExternalViewApi<'a, A>
where
    A: VMApi<'a>,
{
    type PrintApiImpl = A::PrintApiImpl;

    fn print_api_impl() -> Self::PrintApiImpl {
        A::print_api_impl()
    }
}

impl<'a, A> SendApi<'a> for ExternalViewApi<'a, A>
where
    A: VMApi<'a>,
{
    type SendApiImpl = A::SendApiImpl;

    fn send_api_impl() -> Self::SendApiImpl {
        A::send_api_impl()
    }
}

impl<'a, A> StaticVarApi for ExternalViewApi<'a, A>
where
    A: VMApi<'a>,
{
    type StaticVarApiImpl = A::StaticVarApiImpl;

    fn static_var_api_impl() -> Self::StaticVarApiImpl {
        A::static_var_api_impl()
    }
}

impl<'a, A> StorageWriteApi for ExternalViewApi<'a, A>
where
    A: VMApi<'a>,
{
    type StorageWriteApiImpl = A::StorageWriteApiImpl;

    fn storage_write_api_impl() -> Self::StorageWriteApiImpl {
        A::storage_write_api_impl()
    }
}

impl<'a, A> CallTypeApi<'a> for ExternalViewApi<'a, A> where A: VMApi<'a> {}

impl<'a, A> StorageMapperApi<'a> for ExternalViewApi<'a, A> where A: VMApi<'a> {}

impl<'a, A> VMApi<'a> for ExternalViewApi<'a, A>
where
    A: VMApi<'a>,
{
    fn external_view_init_override() -> bool {
        true
    }
}

impl<'a, A> PartialEq for ExternalViewApi<'a, A>
where
    A: VMApi<'a>,
{
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl<'a, A> Eq for ExternalViewApi<'a, A> where A: VMApi<'a> {}
