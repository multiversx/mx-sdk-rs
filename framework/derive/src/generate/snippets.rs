pub fn contract_object_def() -> proc_macro2::TokenStream {
    quote! {
    pub struct ContractObj<A>(multiversx_sc::contract_base::UniversalContractObj<A>)
    where
        A: multiversx_sc::api::VMApi;
    }
}

pub fn impl_contract_base() -> proc_macro2::TokenStream {
    quote! {
        impl<A> multiversx_sc::contract_base::ContractBase for ContractObj<A>
        where
            A: multiversx_sc::api::VMApi,
        {
            type Api = A;
        }
    }
}

pub fn new_contract_object_fn() -> proc_macro2::TokenStream {
    quote! {
        pub fn contract_obj<A>() -> ContractObj<A>
        where
            A: multiversx_sc::api::VMApi,
        {
            ContractObj::<A>(multiversx_sc::contract_base::UniversalContractObj::<A>::new())
        }

        pub struct ContractBuilder;

        impl multiversx_sc::contract_base::CallableContractBuilder for self::ContractBuilder {
            fn new_contract_obj<A: multiversx_sc::api::VMApi + Send + Sync>(
                &self,
            ) -> multiversx_sc::types::heap::Box<dyn multiversx_sc::contract_base::CallableContract>
            {
                multiversx_sc::types::heap::Box::new(self::contract_obj::<A>())
            }
        }
    }
}

// TODO: explore auto-implementations of supertraits
#[allow(dead_code)]
pub fn impl_auto_impl() -> proc_macro2::TokenStream {
    quote! {
        impl<A> AutoImpl for ContractObj<A> where
            A: multiversx_sc::contract_base::ContractBase
                + multiversx_sc::api::ErrorApi
                + multiversx_sc::api::EndpointArgumentApi
                + multiversx_sc::api::EndpointFinishApi
                + multiversx_sc::api::ManagedTypeApi
        {
        }
    }
}

pub fn impl_callable_contract() -> proc_macro2::TokenStream {
    quote! {
        impl<A> multiversx_sc::contract_base::CallableContract for ContractObj<A>
        where
            A: multiversx_sc::api::VMApi + Send + Sync,
        {
            fn call(&self, fn_name: &str) -> bool {
                let mut obj = multiversx_sc::contract_base::UniversalContractObj::<A>::new();
                EndpointWrappers::call(&mut obj, fn_name)
            }
        }
    }
}

pub fn proxy_object_def() -> proc_macro2::TokenStream {
    quote! {
        pub struct Proxy<A>
        where
            A: multiversx_sc::api::VMApi + 'static,
        {
            _phantom: core::marker::PhantomData<A>,
        }

        impl<A> multiversx_sc::contract_base::ProxyObjBase for Proxy<A>
        where
            A: multiversx_sc::api::VMApi + 'static,
        {
            type Api = A;
            type To = ();

            fn extract_opt_address(
                &mut self,
            ) -> multiversx_sc::types::ManagedOption<
                Self::Api,
                multiversx_sc::types::ManagedAddress<Self::Api>,
            > {
                multiversx_sc::types::ManagedOption::none()
            }

            fn extract_address(&mut self) -> multiversx_sc::types::ManagedAddress<Self::Api> {
                multiversx_sc::api::ErrorApiImpl::signal_error(
                    &<A as multiversx_sc::api::ErrorApi>::error_api_impl(),
                    multiversx_sc::err_msg::RECIPIENT_ADDRESS_NOT_SET.as_bytes(),
                )
            }

            fn extract_proxy_to(&mut self) -> Self::To {}
        }

        impl<A> multiversx_sc::contract_base::ProxyObjNew for Proxy<A>
        where
            A: multiversx_sc::api::VMApi + 'static,
        {
            type ProxyTo = ProxyTo<A>;

            fn new_proxy_obj() -> Self {
                Proxy {
                    _phantom: core::marker::PhantomData,
                }
            }

            fn contract(mut self, address: multiversx_sc::types::ManagedAddress<Self::Api>) -> Self::ProxyTo {
                ProxyTo {
                    address: multiversx_sc::types::ManagedOption::some(address)
                }
            }
        }

        pub struct ProxyTo<A>
        where
            A: multiversx_sc::api::VMApi + 'static,
        {
            pub address:
                multiversx_sc::types::ManagedOption<A, multiversx_sc::types::ManagedAddress<A>>,
        }

        impl<A> multiversx_sc::contract_base::ProxyObjBase for ProxyTo<A>
        where
            A: multiversx_sc::api::VMApi + 'static,
        {
            type Api = A;
            type To = multiversx_sc::types::ManagedAddress<A>;

            fn extract_opt_address(
                &mut self,
            ) -> multiversx_sc::types::ManagedOption<
                Self::Api,
                multiversx_sc::types::ManagedAddress<Self::Api>,
            > {
                core::mem::replace(
                    &mut self.address,
                    multiversx_sc::types::ManagedOption::none(),
                )
            }

            fn extract_address(&mut self) -> multiversx_sc::types::ManagedAddress<Self::Api> {
                let address = core::mem::replace(
                    &mut self.address,
                    multiversx_sc::types::ManagedOption::none(),
                );
                address.unwrap_or_sc_panic(multiversx_sc::err_msg::RECIPIENT_ADDRESS_NOT_SET)
            }

            fn extract_proxy_to(&mut self) -> Self::To {
                self.extract_address()
            }
        }
    }
}

pub fn callback_proxy_object_def() -> proc_macro2::TokenStream {
    quote! {
        pub struct CallbackProxyObj<A>
        where
            A: multiversx_sc::api::VMApi + 'static,
        {
            _phantom: core::marker::PhantomData<A>,
        }

        impl<A> multiversx_sc::contract_base::CallbackProxyObjBase for CallbackProxyObj<A>
        where
            A: multiversx_sc::api::VMApi + 'static,
        {
            type Api = A;

            fn new_cb_proxy_obj() -> Self {
                CallbackProxyObj {
                    _phantom: core::marker::PhantomData,
                }
            }
        }
    }
}

pub fn call_method_api_static_init() -> proc_macro2::TokenStream {
    quote! {
        <Self::Api as multiversx_sc::api::VMApi>::init_static();
    }
}
