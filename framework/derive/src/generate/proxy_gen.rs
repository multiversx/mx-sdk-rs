use quote::ToTokens;

use super::util::*;
use crate::{
    generate::{convert_to_owned_type::convert_to_owned_type, snippets, supertrait_gen},
    model::{ArgPaymentMetadata, ContractTrait, Method, MethodArgument, PublicRole},
};

pub fn proxy_arg_gen(
    method_args: &[MethodArgument],
    generics: &mut syn::Generics,
) -> Vec<proc_macro2::TokenStream> {
    let mut args_decl = Vec::new();
    for (arg_index, arg) in method_args.iter().enumerate() {
        let unprocessed_attributes = &arg.unprocessed_attributes;
        let pat = &arg.pat;
        if arg.is_endpoint_arg() {
            // let ty = &arg.ty;
            let mut bounds = syn::punctuated::Punctuated::new();
            bounds.push(syn::TypeParamBound::Trait(syn::TraitBound {
                paren_token: None,
                modifier: syn::TraitBoundModifier::None,
                lifetimes: None,
                path: equivalent_encode_path_gen(&arg.ty),
            }));
            let arg_type_generated_ident = generate_proxy_type_generic(arg_index);
            generics
                .params
                .push(syn::GenericParam::Type(syn::TypeParam {
                    attrs: Vec::new(),
                    ident: arg_type_generated_ident.clone(),
                    colon_token: Some(syn::token::Colon(proc_macro2::Span::call_site())),
                    bounds,
                    eq_token: None,
                    default: None,
                }));
            args_decl.push(quote! { #(#unprocessed_attributes)* #pat : #arg_type_generated_ident });
        } else {
            let ty = &arg.ty;
            args_decl.push(quote! { #(#unprocessed_attributes)* #pat : #ty });
        }
    }

    args_decl
}

pub fn generate_proxy_method_sig(
    method: &Method,
    proxy_return_struct_path: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let method_name = &method.name;
    let mut generics = method.generics.clone();
    let generics_where = &method.generics.where_clause;
    let arg_decl = proxy_arg_gen(&method.method_args, &mut generics);
    let ret_tok = match &method.return_type {
        syn::ReturnType::Default => quote! { () },
        syn::ReturnType::Type(_, ty) => quote! { #ty },
    };
    let result = quote! {
        fn #method_name #generics (
            &mut self,
            #(#arg_decl),*
        ) -> #proxy_return_struct_path<Self::Api, #ret_tok>
        #generics_where
    };
    result
}

pub fn generate_proxy_endpoint(m: &Method, endpoint_name: String) -> proc_macro2::TokenStream {
    let mut token_count = 0;
    let mut token_expr =
        quote! { multiversx_sc::types::EgldOrEsdtTokenIdentifier::<Self::Api>::egld() };
    let mut nonce_count = 0;
    let mut nonce_expr = quote! { 0u64 };
    let mut payment_count = 0;
    let mut payment_expr = quote! { multiversx_sc::types::BigUint::<Self::Api>::zero() };
    let mut multi_count = 0;
    let mut multi_expr_opt = None;

    let mut arg_push_snippets = Vec::<proc_macro2::TokenStream>::new();

    for arg in &m.method_args {
        match &arg.metadata.payment {
            ArgPaymentMetadata::NotPayment => {
                let pat = &arg.pat;
                arg_push_snippets.push(quote! {
                    multiversx_sc::types::ContractCall::proxy_arg(&mut ___contract_call___, &#pat);
                });
            },
            ArgPaymentMetadata::PaymentToken => {
                token_count += 1;
                let pat = &arg.pat;
                token_expr = quote! { #pat };
            },
            ArgPaymentMetadata::PaymentNonce => {
                nonce_count += 1;
                let pat = &arg.pat;
                nonce_expr = quote! { #pat };
            },
            ArgPaymentMetadata::PaymentAmount => {
                payment_count += 1;
                let pat = &arg.pat;
                payment_expr = quote! { #pat };
            },
            ArgPaymentMetadata::PaymentMulti => {
                multi_count += 1;
                let pat = &arg.pat;
                multi_expr_opt = Some(quote! { #pat });
            },
        }
    }

    assert!(
        payment_count <= 1,
        "No more than one payment argument allowed in call proxy"
    );
    assert!(
        token_count <= 1,
        "No more than one payment token argument allowed in call proxy"
    );
    assert!(
        nonce_count <= 1,
        "No more than one payment nonce argument allowed in call proxy"
    );
    assert!(
        multi_count <= 1,
        "No more than one payment multi argument allowed in call proxy"
    );

    let contract_call_type;
    let contract_call_init;
    if token_count > 0 || nonce_count > 0 || payment_count > 0 {
        assert!(multi_count == 0, "#[payment_multi] cannot coexist with any other payment annotation in the same endpoint");

        if token_count == 0 && nonce_count == 0 {
            contract_call_type = quote! { multiversx_sc::types::ContractCallWithEgld };
            contract_call_init = quote! {
                let mut ___contract_call___ = multiversx_sc::types::ContractCallWithEgld::new(
                    ___address___,
                    #endpoint_name,
                    #payment_expr,
                );
            };
        } else {
            contract_call_type = quote! { multiversx_sc::types::ContractCallWithEgldOrSingleEsdt };
            contract_call_init = quote! {
                let mut ___contract_call___ = multiversx_sc::types::ContractCallWithEgldOrSingleEsdt::new(
                    ___address___,
                    #endpoint_name,
                    #token_expr,
                    #nonce_expr,
                    #payment_expr,
                );
            };
        }
    } else if multi_count > 0 {
        let multi_expr = multi_expr_opt.unwrap();
        contract_call_type = quote! { multiversx_sc::types::ContractCallWithMultiEsdt };
        contract_call_init = quote! {
            let mut ___contract_call___ = multiversx_sc::types::ContractCallWithMultiEsdt::new(
                ___address___,
                #endpoint_name,
                #multi_expr.clone_value(),
            );
        };
    } else {
        contract_call_type = quote! { multiversx_sc::types::ContractCallNoPayment };
        contract_call_init = quote! {
            let mut ___contract_call___ = multiversx_sc::types::ContractCallNoPayment::new(
                ___address___,
                #endpoint_name,
            );
        };
    }

    let msig = generate_proxy_method_sig(m, contract_call_type);

    let sig = quote! {
        #[allow(clippy::too_many_arguments)]
        #[allow(clippy::type_complexity)]
        #msig {
            let ___address___ = self.extract_address();
            #contract_call_init
            #(#arg_push_snippets)*
            ___contract_call___
        }
    };

    sig
}

pub fn generate_proxy_deploy(init_method: &Method) -> proc_macro2::TokenStream {
    let msig =
        generate_proxy_method_sig(init_method, quote! { multiversx_sc::types::ContractDeploy });

    let mut payment_count = 0;
    let mut multi_count = 0;
    let mut token_count = 0;
    let mut nonce_count = 0;

    let arg_push_snippets: Vec<proc_macro2::TokenStream> = init_method
        .method_args
        .iter()
        .map(|arg| match &arg.metadata.payment {
            ArgPaymentMetadata::NotPayment => {
                let pat = &arg.pat;
                quote! {
                    ___contract_deploy___.push_endpoint_arg(&#pat);
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
            ArgPaymentMetadata::PaymentMulti => {
                multi_count += 1;
                let pat = &arg.pat;
                quote! {
                    ___contract_deploy___ = ___contract_deploy___.with_multi_token_transfer(#pat);
                }
            },
        })
        .collect();

    assert!(
        payment_count <= 1,
        "No more than one payment argument allowed in call proxy"
    );
    assert!(token_count == 0, "No ESDT payment allowed in #[init]");
    assert!(nonce_count == 0, "No SFT/NFT payment allowed in #[init]");

    let sig = quote! {
        #[allow(clippy::too_many_arguments)]
        #[allow(clippy::type_complexity)]
        #msig {
            let ___opt_address___ = self.extract_opt_address();
            let mut ___contract_deploy___ = multiversx_sc::types::new_contract_deploy(
                ___opt_address___,
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
            multiversx_sc::contract_base::ProxyObjBase
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

fn equivalent_encode_path_gen(ty: &syn::Type) -> syn::Path {
    let owned_type = convert_to_owned_type(ty);
    syn::parse_str(
        format!(
            "multiversx_sc::codec::CodecInto<{}>",
            owned_type.to_token_stream()
        )
        .as_str(),
    )
    .unwrap()
}
