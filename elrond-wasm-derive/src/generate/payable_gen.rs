use super::util::*;
use crate::model::{Method, MethodArgument, MethodPayableMetadata};

pub fn generate_payable_snippet(m: &Method) -> proc_macro2::TokenStream {
	payable_snippet_for_metadata(
		m.payable_metadata(),
		&m.payment_token_arg(),
		&m.payment_amount_arg(),
		&m.payment_nonce_arg(),
	)
}

fn payable_snippet_for_metadata(
	mpm: MethodPayableMetadata,
	payment_token_arg: &Option<MethodArgument>,
	payment_amount_arg: &Option<MethodArgument>,
	payment_nonce_arg: &Option<MethodArgument>,
) -> proc_macro2::TokenStream {
	match mpm {
		MethodPayableMetadata::NotPayable => {
			let amount_init = zero_amount_init(payment_amount_arg);
			let token_init = egld_token_init(payment_token_arg);
			let nonce_init = zero_nonce_init(payment_nonce_arg);
			quote! {
				self.call_value().check_not_payable();
				#amount_init
				#token_init
				#nonce_init
			}
		},
		MethodPayableMetadata::Egld => {
			let payment_var_name = var_name_or_underscore(payment_amount_arg);
			let token_init = egld_token_init(payment_token_arg);
			let nonce_init = zero_nonce_init(payment_nonce_arg);
			quote! {
				let #payment_var_name = self.call_value().require_egld();
				#token_init
				#nonce_init
			}
		},
		MethodPayableMetadata::SingleEsdtToken(token_name) => {
			let token_literal = byte_str_slice_literal(token_name.as_bytes());
			let payment_var_name = var_name_or_underscore(payment_amount_arg);
			let token_init = if let Some(arg) = payment_token_arg {
				let pat = &arg.pat;
				quote! {
					let #pat = TokenIdentifier::from(#token_literal);
				}
			} else {
				quote! {}
			};
			let nonce_init = nonce_getter_init(payment_nonce_arg);
			quote! {
				let #payment_var_name = self.call_value().require_esdt(#token_literal);
				#token_init
				#nonce_init
			}
		},
		MethodPayableMetadata::AnyToken => {
			let nonce_init = nonce_getter_init(payment_nonce_arg);
			if payment_amount_arg.is_none() && payment_token_arg.is_none() {
				nonce_init
			} else {
				let payment_var_name = var_name_or_underscore(payment_amount_arg);
				let token_var_name = var_name_or_underscore(payment_token_arg);
				quote! {
					let (#payment_var_name, #token_var_name) = self.call_value().payment_token_pair();
					#nonce_init
				}
			}
		},
	}
}

fn zero_amount_init(opt_arg: &Option<MethodArgument>) -> proc_macro2::TokenStream {
	if let Some(arg) = opt_arg {
		let pat = &arg.pat;
		quote! {
			let #pat = BigUint::zero();
		}
	} else {
		quote! {}
	}
}

fn egld_token_init(opt_arg: &Option<MethodArgument>) -> proc_macro2::TokenStream {
	if let Some(arg) = opt_arg {
		let pat = &arg.pat;
		quote! {
			let #pat = TokenIdentifier::egld();
		}
	} else {
		quote! {}
	}
}

fn zero_nonce_init(opt_arg: &Option<MethodArgument>) -> proc_macro2::TokenStream {
	if let Some(arg) = opt_arg {
		let pat = &arg.pat;
		quote! {
			let #pat = 0u64;
		}
	} else {
		quote! {}
	}
}

fn nonce_getter_init(opt_arg: &Option<MethodArgument>) -> proc_macro2::TokenStream {
	if let Some(arg) = opt_arg {
		let pat = &arg.pat;
		quote! {
			let #pat = self.call_value().esdt_token_nonce();
		}
	} else {
		quote! {}
	}
}

fn var_name_or_underscore(opt_arg: &Option<MethodArgument>) -> proc_macro2::TokenStream {
	if let Some(arg) = opt_arg {
		let pat = &arg.pat;
		quote! { #pat }
	} else {
		quote! { _ }
	}
}
