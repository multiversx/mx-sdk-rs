use super::reserved;
use crate::model::{ArgPaymentMetadata, ContractTrait, Method, PublicRole};

const INIT_ENDPOINT_NAME: &str = "init";

/// TODO: make it work with Result instead of panic
pub fn validate_contract(contract_trait: &ContractTrait) {
    for m in &contract_trait.methods {
        validate_method(m);
    }
}

pub fn validate_method(m: &Method) {
    validate_method_name(m);
    validate_payment_args(m);
    validate_callback_call_result_arg(m);
}

fn validate_method_name(m: &Method) {
    if let PublicRole::Endpoint(endpoint_metadata) = &m.public_role {
        let endpoint_name_str = endpoint_metadata.public_name.to_string();
        assert!(
            endpoint_name_str != INIT_ENDPOINT_NAME,
            "Cannot declare endpoint with name 'init'. Use #[init] instead."
        );
        assert!(!reserved::is_reserved(endpoint_name_str.as_str()), "Cannot declare endpoint with name '{endpoint_name_str}', because that name is reserved by the Arwen API.");
    }
}

fn validate_payment_args(m: &Method) {
    let num_payment_amount = m
        .method_args
        .iter()
        .filter(|&arg| matches!(arg.metadata.payment, ArgPaymentMetadata::PaymentAmount))
        .count();
    let num_payment_token = m
        .method_args
        .iter()
        .filter(|&arg| matches!(arg.metadata.payment, ArgPaymentMetadata::PaymentToken))
        .count();
    let num_payment_nonce = m
        .method_args
        .iter()
        .filter(|&arg| matches!(arg.metadata.payment, ArgPaymentMetadata::PaymentNonce))
        .count();
    let num_payment_multi = m
        .method_args
        .iter()
        .filter(|&arg| matches!(arg.metadata.payment, ArgPaymentMetadata::PaymentMulti))
        .count();
    assert!(
        num_payment_amount <= 1,
        "only one `#[payment]` argument allowed (method: `{}`)",
        m.name
    );
    assert!(
        num_payment_token <= 1,
        "only one `#[payment_token]` argument allowed (method: `{}`)",
        m.name
    );
    assert!(
        num_payment_nonce <= 1,
        "only one `#[payment_nonce]` argument allowed (method: `{}`)",
        m.name
    );
    assert!(
        num_payment_multi <= 1,
        "only one `#[payment_multi]` argument allowed (method: `{}`)",
        m.name
    );
    if !m.is_payable() {
        assert!(num_payment_amount == 0, "`#[payment]` only allowed in payable endpoints, payable init or callbacks (method: `{}`)", m.name);

        assert!(num_payment_token == 0, "`#[payment_token]` only allowed in payable endpoints, payable init or callbacks (method: `{}`)", m.name);
    }
    if let PublicRole::Init(init_metadata) = &m.public_role {
        assert!(
            init_metadata.payable.no_esdt(),
            "only EGLD payments currently allowed in constructors"
        );
    }
    validate_payment_args_not_reference(m);
}

pub fn validate_payment_args_not_reference(m: &Method) {
    if let Some(payment_arg) = m.payment_amount_arg() {
        match &payment_arg.ty {
            syn::Type::Path(_) => {},
            syn::Type::Reference(_) => {
                panic!("The payment argument is expected to be an owned BigUint, references are not allowed.");
            },
            _ => panic!("Unsupported payment argument type"),
        }
    }
}

fn validate_callback_call_result_arg(m: &Method) {
    let num_call_result = m
        .method_args
        .iter()
        .filter(|&arg| arg.metadata.callback_call_result)
        .count();

    if matches!(&m.public_role, PublicRole::Callback(_)) {
        assert!(
            num_call_result <= 1,
            "only one `#[call_result]` argument allowed"
        );
    } else {
        assert!(
            num_call_result <= 1,
            "`#[call_result]` argument only allowed in `#[callback]` methods"
        );
    }
}
