use super::{attr_names::*, util::*};

pub fn is_payment_amount(attr: &syn::Attribute) -> bool {
    is_attribute_with_no_args(attr, ATTR_PAYMENT_AMOUNT)
        || is_attribute_with_no_args(attr, ATTR_PAYMENT)
}

pub fn is_payment_token(attr: &syn::Attribute) -> bool {
    is_attribute_with_no_args(attr, ATTR_PAYMENT_TOKEN)
}

pub fn is_payment_nonce(attr: &syn::Attribute) -> bool {
    is_attribute_with_no_args(attr, ATTR_PAYMENT_NONCE)
}

pub fn is_payment_multi(attr: &syn::Attribute) -> bool {
    is_attribute_with_no_args(attr, ATTR_PAYMENT_MULTI)
}

pub fn is_var_args(attr: &syn::Attribute) -> bool {
    is_attribute_with_no_args(attr, ATTR_VAR_ARGS)
}

pub fn is_callback_result_arg(attr: &syn::Attribute) -> bool {
    is_attribute_with_no_args(attr, ATTR_CALLBACK_CALL_RESULT)
}

pub fn is_event_topic(attr: &syn::Attribute) -> bool {
    is_attribute_with_no_args(attr, ATTR_EVENT_INDEXED)
}
