pub fn contract_object_def() -> proc_macro2::TokenStream {
    quote! {
        pub struct ContractObj<A: elrond_wasm::api::ContractBase> {
            api: A,
        }
    }
}

pub fn impl_contract_base() -> proc_macro2::TokenStream {
    quote! {
        impl<A>elrond_wasm::api::ContractBase for ContractObj<A>
        where
            A:elrond_wasm::api::ContractBase
                + elrond_wasm::api::ErrorApi
                + elrond_wasm::api::EndpointArgumentApi
                + elrond_wasm::api::EndpointFinishApi
                + elrond_wasm::api::ManagedTypeApi
                + Clone
                + 'static,
        {
            type TypeManager = A::TypeManager;
            type Storage = A::Storage;
            type CallValue = A::CallValue;
            type SendApi = A::SendApi;
            type BlockchainApi = A::BlockchainApi;
            type CryptoApi = A::CryptoApi;
            type LogApi = A::LogApi;
            type ErrorApi = A::ErrorApi;

            #[inline]
            fn get_storage_raw(&self) -> Self::Storage {
                self.api.get_storage_raw()
            }
            #[inline]
            fn call_value(&self) -> Self::CallValue {
                self.api.call_value()
            }
            #[inline]
            fn send(&self) -> Self::SendApi {
                self.api.send()
            }
            #[inline]
            fn type_manager(&self) -> Self::TypeManager {
                self.api.type_manager()
            }
            #[inline]
            fn blockchain(&self) -> Self::BlockchainApi {
                self.api.blockchain()
            }
            #[inline]
            fn crypto(&self) -> Self::CryptoApi {
                self.api.crypto()
            }
            #[inline]
            fn log_api_raw(&self) -> Self::LogApi {
                self.api.log_api_raw()
            }
            #[inline]
            fn error_api(&self) -> Self::ErrorApi {
                self.api.error_api()
            }
        }
    }
}

pub fn new_contract_object_fn() -> proc_macro2::TokenStream {
    quote! {
        pub fn contract_obj<A>(api: A) -> ContractObj<A>
            where A: elrond_wasm::api::ContractBase
                + elrond_wasm::api::ErrorApi
                + elrond_wasm::api::EndpointArgumentApi
                + elrond_wasm::api::EndpointFinishApi
                + elrond_wasm::api::ManagedTypeApi
                + Clone
                + 'static,
        {
            ContractObj { api }
        }
    }
}

// TODO: explore auto-implementations of supertraits
#[allow(dead_code)]
pub fn impl_auto_impl() -> proc_macro2::TokenStream {
    quote! {
        impl<A> AutoImpl for ContractObj<A> where
            A: elrond_wasm::api::ContractBase
                + elrond_wasm::api::ErrorApi
                + elrond_wasm::api::EndpointArgumentApi
                + elrond_wasm::api::EndpointFinishApi
                + elrond_wasm::api::ManagedTypeApi
                + Clone
                + 'static
        {
        }
    }
}
pub fn impl_private_api() -> proc_macro2::TokenStream {
    quote! {
        impl<A> elrond_wasm::api::ContractPrivateApi for ContractObj<A>
        where
            A: elrond_wasm::api::ContractBase
                + elrond_wasm::api::ErrorApi
                + elrond_wasm::api::EndpointArgumentApi
                + elrond_wasm::api::EndpointFinishApi
                + elrond_wasm::api::ManagedTypeApi
                + Clone
                + 'static,
        {
            type ArgumentApi = A;
            type CallbackClosureArgumentApi = A;
            type FinishApi = A;

            #[inline]
            fn argument_api(&self) -> Self::ArgumentApi {
                self.api.clone()
            }

            #[inline]
            fn callback_closure_arg_api(&self) -> Self::CallbackClosureArgumentApi {
                self.api.clone()
            }

            #[inline]
            fn finish_api(&self) -> Self::FinishApi {
                self.api.clone()
            }
        }
    }
}

pub fn impl_callable_contract() -> proc_macro2::TokenStream {
    quote! {
        impl<A> elrond_wasm::api::CallableContract<A> for ContractObj<A>
            where A: elrond_wasm::api::ContractBase
                + elrond_wasm::api::ErrorApi
                + elrond_wasm::api::EndpointArgumentApi
                + elrond_wasm::api::EndpointFinishApi
                + elrond_wasm::api::ManagedTypeApi
                + Clone
                + 'static,
        {
            fn call(&self, fn_name: &[u8]) -> bool {
                EndpointWrappers::call(self, fn_name)
            }
            fn into_api(self: Box<Self>) -> A {
                self.api
            }
        }
    }
}

pub fn proxy_object_def() -> proc_macro2::TokenStream {
    quote! {
        pub struct Proxy<SA>
        where
            SA: elrond_wasm::api::SendApi + 'static,
        {
            pub api: SA,
            pub address: elrond_wasm::types::ManagedAddress<SA::ProxyTypeManager>,
        }

        impl<SA> elrond_wasm::api::ProxyObjApi for Proxy<SA>
        where
            SA: elrond_wasm::api::SendApi + 'static,
        {
            type TypeManager = SA::ProxyTypeManager;
            type Storage = SA::ProxyStorage;
            type SendApi = SA;

            fn new_proxy_obj(api: SA) -> Self {
                let zero_address = ManagedAddress::zero_address(api.type_manager());
                Proxy {
                    api,
                    address: zero_address,
                }
            }

            #[inline]
            fn contract(mut self, address: ManagedAddress<Self::TypeManager>) -> Self {
                self.address = address;
                self
            }

            #[inline]
            fn into_fields(self) -> (Self::SendApi, ManagedAddress<Self::TypeManager>) {
                (self.api, self.address)
            }
        }
    }
}

pub fn callback_proxy_object_def() -> proc_macro2::TokenStream {
    quote! {
        pub struct CallbackProxyObj<SA>
        where
            SA: elrond_wasm::api::SendApi + 'static,
        {
            pub api: SA,
        }

        impl<SA> elrond_wasm::api::CallbackProxyObjApi for CallbackProxyObj<SA>
        where
            SA: elrond_wasm::api::SendApi + 'static,
        {
            type TypeManager = SA::ProxyTypeManager;
            type Storage = SA::ProxyStorage;
            type SendApi = SA;

            fn new_cb_proxy_obj(api: SA) -> Self {
                CallbackProxyObj {
                    api,
                }
            }

            fn cb_call_api(self) -> Self::TypeManager {
                self.api.type_manager()
            }
        }
    }
}
