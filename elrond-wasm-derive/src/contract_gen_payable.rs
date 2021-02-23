use super::util::*;
// use super::parse_attr::*;
use super::arg_def::*;
use super::contract_gen_method::*;

pub fn generate_payable_snippet(m: &Method) -> proc_macro2::TokenStream {
	payable_snippet_for_metadata(m.metadata.payable_metadata(), &m.payment_arg, &m.token_arg)
}

fn payable_snippet_for_metadata(
	mpm: MethodPayableMetadata,
	payment_arg: &Option<MethodArg>,
	token_arg: &Option<MethodArg>,
) -> proc_macro2::TokenStream {
	match mpm {
		MethodPayableMetadata::NoMetadata => quote! {},
		MethodPayableMetadata::NotPayable => {
			let payment_init = if let Some(arg) = payment_arg {
				let pat = &arg.pat;
				quote! {
					let #pat = BigUint::zero();
				}
			} else {
				quote! {}
			};
			let token_init = if let Some(arg) = token_arg {
				let pat = &arg.pat;
				quote! {
					let #pat = TokenIdentifier::egld();
				}
			} else {
				quote! {}
			};
			quote! {
				self.call_value().check_not_payable();
				#payment_init
				#token_init
			}
		},
		MethodPayableMetadata::Egld => {
			let payment_var_name = var_name_or_underscore(payment_arg);
			let token_init = if let Some(arg) = token_arg {
				let pat = &arg.pat;
				quote! {
					let #pat = TokenIdentifier::egld();
				}
			} else {
				quote! {}
			};
			quote! {
				let #payment_var_name = self.call_value().require_egld();
				#token_init
			}
		},
		MethodPayableMetadata::SingleEsdtToken(token_name) => {
			let token_literal = byte_str_slice_literal(token_name.as_bytes());
			let payment_var_name = var_name_or_underscore(payment_arg);
			let token_init = if let Some(arg) = token_arg {
				let pat = &arg.pat;
				quote! {
					let #pat = TokenIdentifier::from(#token_literal);
				}
			} else {
				quote! {}
			};
			quote! {
				let #payment_var_name = self.call_value().require_esdt(#token_literal);
				#token_init
			}
		},
		MethodPayableMetadata::AnyToken => {
			if payment_arg.is_none() && token_arg.is_none() {
				quote! {}
			} else {
				let payment_var_name = var_name_or_underscore(payment_arg);
				let token_var_name = var_name_or_underscore(token_arg);
				quote! {
					let (#payment_var_name, #token_var_name) = self.call_value().payment_token_pair();
				}
			}
		},
	}
}

fn var_name_or_underscore(opt_arg: &Option<MethodArg>) -> proc_macro2::TokenStream {
	if let Some(arg) = opt_arg {
		let pat = &arg.pat;
		quote! { #pat }
	} else {
		quote! { _ }
	}
}

pub fn generate_payment_snippet(arg: &MethodArg) -> proc_macro2::TokenStream {
	match &arg.ty {
		syn::Type::Path(type_path) => {
			let type_path_segment = type_path.path.segments.last().unwrap().clone();
			generate_payment_snippet_for_arg_type(&type_path_segment, &arg.pat)
		},
		syn::Type::Reference(type_reference) => {
			if type_reference.mutability.is_some() {
				panic!("Mutable references not supported as contract method arguments");
			}
			match &*type_reference.elem {
				syn::Type::Path(type_path) => {
					let type_path_segment = type_path.path.segments.last().unwrap().clone();
					generate_payment_snippet_for_arg_type(&type_path_segment, &arg.pat)
				},
				_ => panic!("Unsupported reference argument type, reference does not contain type path: {:?}", type_reference),
			}
		},
		other_arg => panic!(
			"Unsupported argument type: {:?}, neither path nor reference",
			other_arg
		),
	}
}

fn generate_payment_snippet_for_arg_type(
	type_path_segment: &syn::PathSegment,
	pat: &syn::Pat,
) -> proc_macro2::TokenStream {
	let type_str = type_path_segment.ident.to_string();
	match type_str.as_str() {
		"BigUint" => quote! {
			let #pat = self.api.egld_value();
		},
		other_stype_str => panic!(
			"Arguments annotated with #[payment] must be of type BigUint. Found: {}",
			other_stype_str
		),
	}
}
