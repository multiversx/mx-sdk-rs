use super::attributes::*;
use crate::model::{ArgMetadata, ArgPaymentMetadata, MethodArgument};

pub fn extract_method_args(m: &syn::TraitItemMethod) -> Vec<MethodArgument> {
    if m.sig.inputs.is_empty() {
        missing_self_panic(m);
    }

    let mut receiver_processed = false;
    m.sig
        .inputs
        .iter()
        .filter_map(|arg| match arg {
            syn::FnArg::Receiver(ref selfref) => {
                if selfref.mutability.is_some() || receiver_processed {
                    missing_self_panic(m);
                }
                receiver_processed = true;
                None
            },
            syn::FnArg::Typed(pat_typed) => {
                if !receiver_processed {
                    missing_self_panic(m);
                }

                Some(extract_method_arg(pat_typed))
            },
        })
        .collect()
}

fn missing_self_panic(m: &syn::TraitItemMethod) -> ! {
    panic!(
        "Trait method `{}` must have `&self` as its first argument.",
        m.sig.ident
    )
}

fn extract_method_arg(pat_typed: &syn::PatType) -> MethodArgument {
    let pat = &*pat_typed.pat;
    let ty = &*pat_typed.ty;
    let mut arg_metadata = ArgMetadata::default();
    let mut unprocessed_attributes = Vec::new();

    process_arg_attributes(
        &pat_typed.attrs,
        &mut arg_metadata,
        &mut unprocessed_attributes,
    );

    let original_pat = pat.clone();
    let mut cleaned_pat = original_pat.clone();
    if let syn::Pat::Ident(ident) = &mut cleaned_pat {
        ident.mutability = None;
    }

    MethodArgument {
        original_pat,
        pat: cleaned_pat,
        ty: ty.clone(),
        unprocessed_attributes,
        metadata: arg_metadata,
    }
}

fn process_arg_attributes(
    attrs: &[syn::Attribute],
    arg_metadata: &mut ArgMetadata,
    unprocessed_attributes: &mut Vec<syn::Attribute>,
) {
    for attr in attrs {
        let processed = process_arg_attribute(attr, arg_metadata);
        if !processed {
            unprocessed_attributes.push(attr.clone());
        }
    }
}

fn process_arg_attribute(attr: &syn::Attribute, arg_metadata: &mut ArgMetadata) -> bool {
    process_payment_token_attribute(attr, arg_metadata)
        || process_payment_nonce_attribute(attr, arg_metadata)
        || process_payment_amount_attribute(attr, arg_metadata)
        || process_payment_multi_attribute(attr, arg_metadata)
        || process_callback_result_attribute(attr, arg_metadata)
        || process_event_topic_attribute(attr, arg_metadata)
}

fn check_no_other_payment_attr(arg_metadata: &ArgMetadata) {
    assert!(!arg_metadata.payment.is_payment_arg(), "Can only annotate argument with one of the following attributes: `#[payment_token]`, `#[payment_nonce]` or `#[payment_amount]`/`#[payment]`.");
}

fn process_payment_token_attribute(attr: &syn::Attribute, arg_metadata: &mut ArgMetadata) -> bool {
    let has_attr = is_payment_token(attr);
    if has_attr {
        check_no_other_payment_attr(&*arg_metadata);
        arg_metadata.payment = ArgPaymentMetadata::PaymentToken;
    }
    has_attr
}

fn process_payment_nonce_attribute(attr: &syn::Attribute, arg_metadata: &mut ArgMetadata) -> bool {
    let has_attr = is_payment_nonce(attr);
    if has_attr {
        check_no_other_payment_attr(&*arg_metadata);
        arg_metadata.payment = ArgPaymentMetadata::PaymentNonce;
    }
    has_attr
}

fn process_payment_amount_attribute(attr: &syn::Attribute, arg_metadata: &mut ArgMetadata) -> bool {
    let has_attr = is_payment_amount(attr);
    if has_attr {
        check_no_other_payment_attr(&*arg_metadata);
        arg_metadata.payment = ArgPaymentMetadata::PaymentAmount;
    }
    has_attr
}

fn process_payment_multi_attribute(attr: &syn::Attribute, arg_metadata: &mut ArgMetadata) -> bool {
    let has_attr = is_payment_multi(attr);
    if has_attr {
        check_no_other_payment_attr(&*arg_metadata);
        arg_metadata.payment = ArgPaymentMetadata::PaymentMulti;
    }
    has_attr
}

fn process_callback_result_attribute(
    attr: &syn::Attribute,
    arg_metadata: &mut ArgMetadata,
) -> bool {
    let has_attr = is_callback_result_arg(attr);
    if has_attr {
        arg_metadata.callback_call_result = true;
    }
    has_attr
}

fn process_event_topic_attribute(attr: &syn::Attribute, arg_metadata: &mut ArgMetadata) -> bool {
    let has_attr = is_event_topic(attr);
    if has_attr {
        arg_metadata.event_topic = true;
    }
    has_attr
}
