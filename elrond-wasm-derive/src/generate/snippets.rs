pub fn contract_object_def() -> proc_macro2::TokenStream {
    quote! {
        pub struct ContractObj<A>
        where
            A: elrond_wasm::api::VMApi,
        {
            _phantom: core::marker::PhantomData<A>,
        }
    }
}

pub fn impl_contract_base() -> proc_macro2::TokenStream {
    quote! {
        impl<A> elrond_wasm::contract_base::ContractBase for ContractObj<A>
        where
            A: elrond_wasm::api::VMApi,
        {
            type Api = A;
        }
    }
}

pub fn new_contract_object_fn() -> proc_macro2::TokenStream {
    quote! {
        pub fn contract_obj<A>() -> ContractObj<A>
        where
            A: elrond_wasm::api::VMApi,
        {
            ContractObj {
                _phantom: core::marker::PhantomData,
            }
        }

        pub struct ContractBuilder;

        impl elrond_wasm::contract_base::CallableContractBuilder for self::ContractBuilder {
            fn new_contract_obj<A: elrond_wasm::api::VMApi>(
                &self,
            ) -> elrond_wasm::types::heap::Box<dyn elrond_wasm::contract_base::CallableContract> {
                elrond_wasm::types::heap::Box::new(ContractObj::<A> {
                    _phantom: core::marker::PhantomData,
                })
            }
        }
    }
}

// TODO: explore auto-implementations of supertraits
#[allow(dead_code)]
pub fn impl_auto_impl() -> proc_macro2::TokenStream {
    quote! {
        impl<A> AutoImpl for ContractObj<A> where
            A: elrond_wasm::contract_base::ContractBase
                + elrond_wasm::api::ErrorApi
                + elrond_wasm::api::EndpointArgumentApi
                + elrond_wasm::api::EndpointFinishApi
                + elrond_wasm::api::ManagedTypeApi
        {
        }
    }
}

pub fn impl_callable_contract() -> proc_macro2::TokenStream {
    quote! {
        impl<A> elrond_wasm::contract_base::CallableContract for ContractObj<A>
        where
            A: elrond_wasm::api::VMApi,
        {
            fn call(&self, fn_name: &[u8]) -> bool {
                EndpointWrappers::call(self, fn_name)
            }

            fn clone_obj(&self) -> elrond_wasm::types::heap::Box<dyn elrond_wasm::contract_base::CallableContract> {
                elrond_wasm::types::heap::Box::new(ContractObj::<A> {
                    _phantom: core::marker::PhantomData,
                })
            }
        }
    }
}

pub fn proxy_object_def() -> proc_macro2::TokenStream {
    quote! {
        pub struct Proxy<A>
        where
            A: elrond_wasm::api::VMApi + 'static,
        {
            pub address: elrond_wasm::types::ManagedOption<A, elrond_wasm::types::ManagedAddress<A>>,
        }

        impl<A> elrond_wasm::contract_base::ProxyObjBase for Proxy<A>
        where
            A: elrond_wasm::api::VMApi + 'static,
        {
            type Api = A;

            fn new_proxy_obj() -> Self {
                Proxy {
                    address: elrond_wasm::types::ManagedOption::none(),
                }
            }

            fn contract(mut self, address: elrond_wasm::types::ManagedAddress<Self::Api>) -> Self {
                self.address = elrond_wasm::types::ManagedOption::some(address);
                self
            }

            fn extract_opt_address(
                &mut self,
            ) -> elrond_wasm::types::ManagedOption<
                Self::Api,
                elrond_wasm::types::ManagedAddress<Self::Api>,
            > {
                core::mem::replace(&mut self.address, elrond_wasm::types::ManagedOption::none())
            }

            fn extract_address(&mut self) -> elrond_wasm::types::ManagedAddress<Self::Api> {
                self.extract_opt_address().unwrap_or_sc_panic(elrond_wasm::err_msg::RECIPIENT_ADDRESS_NOT_SET)
            }
        }
    }
}

pub fn callback_proxy_object_def() -> proc_macro2::TokenStream {
    quote! {
        pub struct CallbackProxyObj<A>
        where
            A: elrond_wasm::api::VMApi + 'static,
        {
            _phantom: core::marker::PhantomData<A>,
        }

        impl<A> elrond_wasm::contract_base::CallbackProxyObjBase for CallbackProxyObj<A>
        where
            A: elrond_wasm::api::VMApi + 'static,
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
        <Self::Api as elrond_wasm::api::VMApi>::init_static();
    }
}
