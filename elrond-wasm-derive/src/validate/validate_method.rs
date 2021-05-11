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
		if endpoint_name_str == INIT_ENDPOINT_NAME {
			panic!("Cannot declare endpoint with name 'init'. Use #[init] instead.")
		}
		if reserved::is_reserved(endpoint_name_str.as_str()) {
			panic!("Cannot declare endpoint with name '{}', because that name is reserved by the Arwen API.", endpoint_name_str);
		}
	}
}

fn validate_payment_args(m: &Method) {
	let num_payment = m
		.method_args
		.iter()
		.filter(|&arg| matches!(arg.metadata.payment, ArgPaymentMetadata::Payment))
		.count();
	let num_payment_token = m
		.method_args
		.iter()
		.filter(|&arg| matches!(arg.metadata.payment, ArgPaymentMetadata::PaymentToken))
		.count();
	if num_payment > 1 {
		panic!(
			"only one `#[payment]` argument allowed (method: `{}`)",
			m.name.to_string()
		);
	}
	if num_payment_token > 1 {
		panic!(
			"only one `#[payment_token]` argument allowed (method: `{}`)",
			m.name.to_string()
		);
	}
	if !m.is_payable() {
		if num_payment > 0 {
			panic!("`#[payment]` only allowed in payable endpoints, payable init or callbacks (method: `{}`)", m.name.to_string());
		}
		if num_payment_token > 0 {
			panic!(
				"`#[payment_token]` only allowed in payable endpoints, payable init or callbacks (method: `{}`)", m.name.to_string());
		}
	}
	if let PublicRole::Init(init_metadata) = &m.public_role {
		if !init_metadata.payable.no_esdt() {
			panic!("only EGLD payments currently allowed in constructors");
		}
	}
	validate_payment_args_not_reference(m);
}

pub fn validate_payment_args_not_reference(m: &Method) {
	if let Some(payment_arg) = m.payment_arg() {
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
		if num_call_result > 1 {
			panic!("only one `#[call_result]` argument allowed");
		}
	} else if num_call_result > 1 {
		panic!("`#[call_result]` argument only allowed in `#[callback]` methods");
	}
}
