use super::method_gen::*;
use super::util::*;
use crate::model::PublicRole;
use crate::{
    generate::{snippets, supertrait_gen},
    model::{ArgPaymentMetadata, ContractTrait, Method},
};

pub fn generate_proxy_endpoint_sig(method: &Method) -> proc_macro2::TokenStream {
    let method_name = &method.name;
    let generics = &method.generics;
    let generics_where = &method.generics.where_clause;
    let arg_decl = arg_declarations(&method.method_args);
    let ret_tok = match &method.return_type {
        syn::ReturnType::Default => quote! { () },
        syn::ReturnType::Type(_, ty) => quote! { #ty },
    };
    let result = quote! {
        fn #method_name #generics (
            self,
            #(#arg_decl),*
        ) -> elrond_wasm::types::ContractCall<Self::Api, <#ret_tok as elrond_wasm::io::EndpointResult>::DecodeAs>
        #generics_where
    };
    result
}

pub fn generate_proxy_deploy_sig(method: &Method) -> proc_macro2::TokenStream {
    let method_name = &method.name;
    let generics = &method.generics;
    let generics_where = &method.generics.where_clause;
    let arg_decl = arg_declarations(&method.method_args);
    let result = quote! {
        fn #method_name #generics (
            self,
            #(#arg_decl),*
        ) -> elrond_wasm::types::ContractDeploy<Self::Api>
        #generics_where
    };
    result
}

pub fn generate_proxy_endpoint(m: &Method, endpoint_name: String) -> proc_macro2::TokenStream {
    let msig = generate_proxy_endpoint_sig(m);

    let mut token_count = 0;
    let mut token_expr = quote! { elrond_wasm::types::TokenIdentifier::egld(___api___.clone()) };
    let mut nonce_count = 0;
    let mut nonce_expr = quote! { 0u64 };
    let mut payment_count = 0;
    let mut payment_expr = quote! { elrond_wasm::types::BigUint::zero(___api___.clone()) };

    let arg_push_snippets: Vec<proc_macro2::TokenStream> = m
        .method_args
        .iter()
        .map(|arg| match &arg.metadata.payment {
            ArgPaymentMetadata::NotPayment => {
                let pat = &arg.pat;
                quote! {
                    ___contract_call___.push_endpoint_arg(#pat);
                }
            },
            ArgPaymentMetadata::PaymentToken => {
                token_count += 1;
                let pat = &arg.pat;
                token_expr = quote! { #pat };

                quote! {}
            },
            ArgPaymentMetadata::PaymentNonce => {
                nonce_count += 1;
                let pat = &arg.pat;
                nonce_expr = quote! { #pat };

                quote! {}
            },
            ArgPaymentMetadata::PaymentAmount => {
                payment_count += 1;
                let pat = &arg.pat;
                payment_expr = quote! { #pat };

                quote! {}
            },
        })
        .collect();

    if payment_count > 1 {
        panic!("No more than one payment argument allowed in call proxy");
    }
    if token_count > 1 {
        panic!("No more than one payment token argument allowed in call proxy");
    }
    if nonce_count > 1 {
        panic!("No more than one payment nonce argument allowed in call proxy");
    }

    let single_payment_snippet = if token_count > 0 || nonce_count > 0 || payment_count > 0 {
        quote! {
            ___contract_call___ = ___contract_call___.add_token_transfer(#token_expr, #nonce_expr, #payment_expr);
        }
    } else {
        quote! {}
    };

    let endpoint_name_literal = byte_str_slice_literal(endpoint_name.as_bytes());

    let sig = quote! {
        #[allow(clippy::too_many_arguments)]
        #[allow(clippy::type_complexity)]
        #msig {
            let (___api___, ___address___) = self.into_fields();
            let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                #endpoint_name_literal,
            );
            #single_payment_snippet
            #(#arg_push_snippets)*
            ___contract_call___
        }
    };

    sig
}

pub fn generate_proxy_deploy(init_method: &Method) -> proc_macro2::TokenStream {
    let msig = generate_proxy_deploy_sig(init_method);

    let mut payment_count = 0;
    let mut token_count = 0;
    let mut nonce_count = 0;

    let arg_push_snippets: Vec<proc_macro2::TokenStream> = init_method
        .method_args
        .iter()
        .map(|arg| match &arg.metadata.payment {
            ArgPaymentMetadata::NotPayment => {
                let pat = &arg.pat;
                quote! {
                    ___contract_deploy___.push_endpoint_arg(#pat);
                }
            },
            ArgPaymentMetadata::PaymentToken => {
                token_count += 1;

                quote! {}
            },
            ArgPaymentMetadata::PaymentNonce => {
                nonce_count += 1;

                quote! {}
            },
            ArgPaymentMetadata::PaymentAmount => {
                payment_count += 1;
                let pat = &arg.pat;
                quote! {
                    ___contract_deploy___ = ___contract_deploy___.with_egld_transfer(#pat);
                }
            },
        })
        .collect();

    if payment_count > 1 {
        panic!("No more than one payment argument allowed in call proxy");
    }
    if token_count > 0 {
        panic!("No ESDT payment allowed in #[init]");
    }
    if nonce_count > 0 {
        panic!("No SFT/NFT payment allowed in #[init]");
    }

    let sig = quote! {
        #[allow(clippy::too_many_arguments)]
        #[allow(clippy::type_complexity)]
        #msig {
            let (___api___, ___address___) =
                self.into_fields();
            let mut ___contract_deploy___ = elrond_wasm::types::new_contract_deploy(
                ___api___.clone(),
                ___address___,
            );
            #(#arg_push_snippets)*
            ___contract_deploy___
        }
    };

    sig
}

pub fn generate_method_impl(contract_trait: &ContractTrait) -> Vec<proc_macro2::TokenStream> {
    contract_trait
        .methods
        .iter()
        .filter_map(|m| match &m.public_role {
            PublicRole::Init(_) => Some(generate_proxy_deploy(m)),
            PublicRole::Endpoint(endpoint_metadata) => Some(generate_proxy_endpoint(
                m,
                endpoint_metadata.public_name.to_string(),
            )),
            _ => None,
        })
        .collect()
}

pub fn proxy_trait(contract: &ContractTrait) -> proc_macro2::TokenStream {
    let proxy_supertrait_decl =
        supertrait_gen::proxy_supertrait_decl(contract.supertraits.as_slice());
    let proxy_methods_impl = generate_method_impl(contract);
    quote! {
        pub trait ProxyTrait:
            elrond_wasm::contract_base::ProxyObjBase
            + Sized
            #(#proxy_supertrait_decl)*
        {
            #(#proxy_methods_impl)*
        }
    }
}

pub fn proxy_obj_code(contract: &ContractTrait) -> proc_macro2::TokenStream {
    let proxy_object_def = snippets::proxy_object_def();
    let impl_all_proxy_traits =
        supertrait_gen::impl_all_proxy_traits(contract.supertraits.as_slice());
    quote! {
        #proxy_object_def

        #(#impl_all_proxy_traits)*
    }
}
