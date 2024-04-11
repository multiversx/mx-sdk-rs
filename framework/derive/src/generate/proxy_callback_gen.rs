use super::snippets;
use crate::model::{
    ArgPaymentMetadata, CallbackMetadata, ContractTrait, Method, MethodArgument, PublicRole,
};

/// Excludes the `#[call_result]` and the payment args.
pub fn cb_proxy_arg_declarations(method_args: &[MethodArgument]) -> Vec<proc_macro2::TokenStream> {
    method_args
        .iter()
        .filter_map(|arg| {
            if arg.metadata.payment.is_payment_arg() || arg.metadata.callback_call_result {
                None
            } else {
                let pat = &arg.pat;
                let ty = &arg.ty;
                Some(quote! {#pat : #ty })
            }
        })
        .collect()
}

fn generate_callback_proxy_method(
    m: &Method,
    callback: &CallbackMetadata,
) -> proc_macro2::TokenStream {
    let arg_decl = cb_proxy_arg_declarations(&m.method_args);
    let cb_name_literal = callback.callback_name.to_string();

    let cb_arg_push_snippets: Vec<proc_macro2::TokenStream> = m
        .method_args
        .iter()
        .map(|arg| {
            if let ArgPaymentMetadata::NotPayment = arg.metadata.payment {
                if arg.metadata.callback_call_result {
                    quote! {}
                } else {
                    let pat = &arg.pat;
                    quote! {
                        ___callback_call___.push_endpoint_arg(&#pat);
                    }
                }
            } else {
                quote! {}
            }
        })
        .collect();
    let method_name = &m.name;
    quote! {
        #[allow(clippy::too_many_arguments)]
        #[allow(clippy::type_complexity)]
        fn #method_name(
            self,
            #(#arg_decl),*
        ) -> multiversx_sc::types::CallbackClosure<Self::Api> {
            let mut ___callback_call___ =
                multiversx_sc::types::new_callback_call::<Self::Api>(#cb_name_literal);
            #(#cb_arg_push_snippets)*
            ___callback_call___
        }
    }
}

pub fn generate_callback_proxies_object(methods: &[Method]) -> proc_macro2::TokenStream {
    let proxy_methods: Vec<proc_macro2::TokenStream> = methods
        .iter()
        .filter_map(|m| {
            if let PublicRole::Callback(callback) | PublicRole::CallbackPromise(callback) =
                &m.public_role
            {
                Some(generate_callback_proxy_method(m, callback))
            } else {
                None
            }
        })
        .collect();

    let callback_proxy_object_def = snippets::callback_proxy_object_def();

    quote! {
        #callback_proxy_object_def

        pub trait CallbackProxy: multiversx_sc::contract_base::CallbackProxyObjBase + Sized {
            #(#proxy_methods)*
        }

        impl<A> self::CallbackProxy for CallbackProxyObj<A> where A: multiversx_sc::api::VMApi + 'static {}
    }
}

pub fn generate_callback_proxies(
    contract: &ContractTrait,
) -> (
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
) {
    if contract.callback_count() == 0 {
        (quote! {}, quote! {}, quote! {})
    } else {
        (
            quote! {
                fn callbacks(&self) -> self::CallbackProxyObj<Self::Api>;
            },
            quote! {
                fn callbacks(&self) -> self::CallbackProxyObj<Self::Api> {
                    <self::CallbackProxyObj::<Self::Api> as multiversx_sc::contract_base::CallbackProxyObjBase>::new_cb_proxy_obj()
                }
            },
            generate_callback_proxies_object(contract.methods.as_slice()),
        )
    }
}
