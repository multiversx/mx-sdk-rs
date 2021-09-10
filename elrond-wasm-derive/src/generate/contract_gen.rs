use super::{method_call_gen::generate_call_method, method_gen, util::*};
use crate::model::{ContractTrait, Method, MethodImpl, PublicRole};

pub fn extract_method_impls(contract_trait: &ContractTrait) -> Vec<proc_macro2::TokenStream> {
    contract_trait
        .methods
        .iter()
        .filter_map(|m| {
            if let MethodImpl::Explicit(body) = &m.implementation {
                let msig = method_gen::generate_sig_with_attributes(m);
                Some(quote! {
                    #msig
                    #body
                })
            } else {
                None
            }
        })
        .collect()
}

pub fn generate_call_methods(contract_trait: &ContractTrait) -> Vec<proc_macro2::TokenStream> {
    contract_trait
        .methods
        .iter()
        .filter_map(|m| match &m.public_role {
            PublicRole::Init(_init_metadata) => Some(generate_call_method(m)),
            PublicRole::Endpoint(_endpoint_metadata) => Some(generate_call_method(m)),
            _ => None,
        })
        .collect()
}

/// Definitions for methods that get auto-generated implementations: events, getters, setters
pub fn generate_auto_impl_defs(contract_trait: &ContractTrait) -> Vec<proc_macro2::TokenStream> {
    contract_trait
        .methods
        .iter()
        .filter_map(|m| {
            if let MethodImpl::Generated(_) = &m.implementation {
                let sig = method_gen::generate_sig_with_attributes(m);
                Some(quote! { #sig ; })
            } else {
                None
            }
        })
        .collect()
}

pub fn generate_wasm_endpoints(contract_trait: &ContractTrait) -> Vec<proc_macro2::TokenStream> {
    contract_trait
        .methods
        .iter()
        .filter_map(|m| match &m.public_role {
            PublicRole::Init(_) => Some(generate_wasm_endpoint(m, &quote! { init })),
            PublicRole::Endpoint(endpoint_metadata) => {
                let endpoint_ident = &endpoint_metadata.public_name;
                Some(generate_wasm_endpoint(m, &quote! { #endpoint_ident }))
            },
            _ => None,
        })
        .collect()
}

fn generate_wasm_endpoint(
    m: &Method,
    endpoint_ident: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let fn_ident = &m.name;
    let call_method_ident = generate_call_method_name(fn_ident);
    quote! {
        #[no_mangle]
        pub fn #endpoint_ident ()
        {
            let inst = new_arwen_instance();
            inst.#call_method_ident();
        }
    }
}
