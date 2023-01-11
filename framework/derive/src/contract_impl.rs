use super::generate::{abi_gen, snippets};
use crate::{
    generate::{
        auto_impl::generate_auto_impls, auto_impl_proxy::generate_all_proxy_trait_imports,
        callback_gen::*, contract_gen::*, endpoints_mod_gen::generate_endpoints_mod,
        function_selector::generate_function_selector_body, proxy_callback_gen::*, proxy_gen,
        supertrait_gen,
    },
    model::ContractTrait,
};

/// Provides the implementation for both modules and contracts.
/// TODO: not a great pattern to have the `is_contract_main` flag, reorganize the code and get rid of it.
pub fn contract_implementation(
    contract: &ContractTrait,
    is_contract_main: bool,
) -> proc_macro2::TokenStream {
    let proxy_trait_imports = generate_all_proxy_trait_imports(contract);
    let module_original_attributes = &contract.original_attributes;
    let trait_name_ident = contract.trait_name.clone();
    let method_impls = extract_method_impls(contract);
    let call_methods = generate_call_methods(contract);
    let auto_impl_defs = generate_auto_impl_defs(contract);
    let auto_impls = generate_auto_impls(contract);
    let endpoints_mod = generate_endpoints_mod(contract, is_contract_main);
    let function_selector_body = generate_function_selector_body(contract);
    let (callback_selector_body, callback_body) = generate_callback_selector_and_main(contract);
    let (callbacks_def, callbacks_impl, callback_proxies_obj) = generate_callback_proxies(contract);

    // this definition is common to release and debug mode
    let supertraits_main = supertrait_gen::main_supertrait_decl(contract.supertraits.as_slice());
    let main_definition = quote! {
        #(#proxy_trait_imports)*

        #(#module_original_attributes)*
        pub trait #trait_name_ident:
        multiversx_sc::contract_base::ContractBase
        + Sized
        #(#supertraits_main)*
        where
        {
            #(#method_impls)*

            #(#auto_impl_defs)*

            #callbacks_def
        }
    };

    let auto_impl_trait = quote! {
        pub trait AutoImpl: multiversx_sc::contract_base::ContractBase {}

        impl<C> #trait_name_ident for C
        where
        C: AutoImpl #(#supertraits_main)*
        {
            #(#auto_impls)*

            #callbacks_impl
        }

        impl<A> AutoImpl for multiversx_sc::contract_base::UniversalContractObj<A> where
            A: multiversx_sc::api::VMApi
        {
        }
    };

    let endpoint_wrapper_supertrait_decl =
        supertrait_gen::endpoint_wrapper_supertrait_decl(contract.supertraits.as_slice());
    let endpoint_wrappers = quote! {
        pub trait EndpointWrappers:
            multiversx_sc::contract_base::ContractBase
            + #trait_name_ident
            #(#endpoint_wrapper_supertrait_decl)*
        {
            #(#call_methods)*

            fn call(&self, fn_name: &str) -> bool {
                #function_selector_body
            }

            fn callback_selector(&self, mut ___cb_closure___: multiversx_sc::types::CallbackClosureForDeser<Self::Api>) -> multiversx_sc::types::CallbackSelectorResult<Self::Api> {
                #callback_selector_body
            }

            fn callback(&self) {
                #callback_body
            }
        }

        impl<A> EndpointWrappers for multiversx_sc::contract_base::UniversalContractObj<A> where
            A: multiversx_sc::api::VMApi
        {
        }
    };

    let abi_provider = abi_gen::generate_abi_provider(contract, is_contract_main);

    let module_traits_code = quote! {
        #main_definition

        #auto_impl_trait

        #endpoint_wrappers

        #abi_provider
    };

    let contract_object_def = snippets::contract_object_def();
    let impl_contract_base = snippets::impl_contract_base();
    let impl_all_auto_impl = supertrait_gen::impl_all_auto_impl(contract.supertraits.as_slice());
    let impl_all_endpoint_wrappers =
        supertrait_gen::impl_all_endpoint_wrappers(contract.supertraits.as_slice());
    let impl_callable_contract = snippets::impl_callable_contract();
    let new_contract_object_fn = snippets::new_contract_object_fn();

    let contract_obj_code = quote! {

        #contract_object_def

        #impl_contract_base

        #(#impl_all_auto_impl)*

        #(#impl_all_endpoint_wrappers)*

        #impl_callable_contract

        #new_contract_object_fn
    };

    let proxy_trait = proxy_gen::proxy_trait(contract);
    let proxy_obj_code = if is_contract_main {
        proxy_gen::proxy_obj_code(contract)
    } else {
        quote! {}
    };

    quote! {
        #module_traits_code

        #contract_obj_code

        #endpoints_mod

        #proxy_trait

        #proxy_obj_code

        #callback_proxies_obj
    }
}
