use core::marker::PhantomData;

use crate::{
    abi::EndpointLocationAbi,
    api::{
        BlockchainApi, CallTypeApi, CallValueApi, CryptoApi, EndpointArgumentApi,
        EndpointFinishApi, ErrorApi, LogApi, ManagedTypeApi, PrintApi, SendApi, StaticVarApi,
        StorageMapperApi, StorageWriteApi, VMApi,
    },
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

impl<A> BlockchainApi for ExternalViewApi<A>
where
    A: VMApi,
{
    type BlockchainApiImpl = A::BlockchainApiImpl;

    fn blockchain_api_impl() -> A::BlockchainApiImpl {
        A::blockchain_api_impl()
    }
}

impl<A: VMApi> CallValueApi for ExternalViewApi<A> {
    type CallValueApiImpl = A::CallValueApiImpl;

    fn call_value_api_impl() -> Self::CallValueApiImpl {
        A::call_value_api_impl()
    }
}

impl<A: VMApi> CryptoApi for ExternalViewApi<A> {
    type CryptoApiImpl = A::CryptoApiImpl;

    fn crypto_api_impl() -> Self::CryptoApiImpl {
        A::crypto_api_impl()
    }
}

impl<A: VMApi> EndpointArgumentApi for ExternalViewApi<A> {
    type EndpointArgumentApiImpl = A::EndpointArgumentApiImpl;

    fn argument_api_impl() -> Self::EndpointArgumentApiImpl {
        A::argument_api_impl()
    }
}

impl<A: VMApi> EndpointFinishApi for ExternalViewApi<A> {
    type EndpointFinishApiImpl = A::EndpointFinishApiImpl;

    fn finish_api_impl() -> Self::EndpointFinishApiImpl {
        A::finish_api_impl()
    }
}

impl<A: VMApi> ErrorApi for super::ExternalViewApi<A> {
    type ErrorApiImpl = A::ErrorApiImpl;

    fn error_api_impl() -> Self::ErrorApiImpl {
        A::error_api_impl()
    }
}

impl<A: VMApi> LogApi for ExternalViewApi<A> {
    type LogApiImpl = A::LogApiImpl;

    fn log_api_impl() -> Self::LogApiImpl {
        A::log_api_impl()
    }
}

impl<A: VMApi> ManagedTypeApi for ExternalViewApi<A> {
    type ManagedTypeApiImpl = A::ManagedTypeApiImpl;

    fn managed_type_impl() -> Self::ManagedTypeApiImpl {
        A::managed_type_impl()
    }
}

impl<A: VMApi> PrintApi for ExternalViewApi<A> {
    type PrintApiImpl = A::PrintApiImpl;

    fn print_api_impl() -> Self::PrintApiImpl {
        A::print_api_impl()
    }
}

impl<A: VMApi> SendApi for ExternalViewApi<A> {
    type SendApiImpl = A::SendApiImpl;

    fn send_api_impl() -> Self::SendApiImpl {
        A::send_api_impl()
    }
}

impl<A: VMApi> StaticVarApi for ExternalViewApi<A> {
    type StaticVarApiImpl = A::StaticVarApiImpl;

    fn static_var_api_impl() -> Self::StaticVarApiImpl {
        A::static_var_api_impl()
    }
}

impl<A: VMApi> StorageWriteApi for ExternalViewApi<A> {
    type StorageWriteApiImpl = A::StorageWriteApiImpl;

    fn storage_write_api_impl() -> Self::StorageWriteApiImpl {
        A::storage_write_api_impl()
    }
}

impl<A: VMApi> CallTypeApi for ExternalViewApi<A> {}

impl<A: VMApi> StorageMapperApi for ExternalViewApi<A> {}

impl<A: VMApi> VMApi for ExternalViewApi<A> {
    fn has_location(location: EndpointLocationAbi) -> bool {
        location == EndpointLocationAbi::ViewContract
    }
}

impl<A: VMApi> PartialEq for ExternalViewApi<A> {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl<A: VMApi> Eq for ExternalViewApi<A> {}
