use crate::model::{Method, MethodArgument, MethodPayableMetadata};

pub fn generate_payable_snippet(m: &Method) -> proc_macro2::TokenStream {
    let call_value_init = call_value_init_snippet(m.payable_metadata());

    let token_init = opt_payment_arg_snippet(&m.payment_token_arg(), quote! {arg_payment_token});
    let nonce_init = opt_payment_arg_snippet(&m.payment_nonce_arg(), quote! {arg_payment_nonce});
    let amount_init = opt_payment_arg_snippet(&m.payment_amount_arg(), quote! {arg_payment_amount});
    let multi_init = opt_payment_arg_snippet(&m.payment_multi_arg(), quote! {arg_payment_multi});

    quote! {
        #call_value_init
        #token_init
        #nonce_init
        #amount_init
        #multi_init
    }
}

fn call_value_init_snippet(mpm: MethodPayableMetadata) -> proc_macro2::TokenStream {
    match &mpm {
        MethodPayableMetadata::NotPayable => {
            quote! {
                multiversx_sc::io::call_value_init::not_payable::<Self::Api>();
            }
        },
        MethodPayableMetadata::Egld => {
            quote! {
                multiversx_sc::io::call_value_init::payable_egld::<Self::Api>();
            }
        },
        MethodPayableMetadata::SingleEsdtToken(token_identifier) => {
            quote! {
                multiversx_sc::io::call_value_init::payable_single_specific_token::<Self::Api>(#token_identifier);
            }
        },
        MethodPayableMetadata::AnyToken => {
            quote! {
                multiversx_sc::io::call_value_init::payable_any::<Self::Api>();
            }
        },
    }
}

fn opt_payment_arg_snippet(
    opt_arg: &Option<MethodArgument>,
    init_fn_name: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    opt_arg
        .as_ref()
        .map(|arg| {
            let pat = &arg.pat;
            quote! {
                let #pat = multiversx_sc::io::call_value_init::#init_fn_name::<Self::Api>();
            }
        })
        .unwrap_or_default()
}
