use super::util::*;
use crate::model::{ContractTrait, Method, PublicRole};

fn function_selector_match_arm(m: &Method, endpoint_name: &str) -> proc_macro2::TokenStream {
	let fn_ident = &m.name;
	let call_method_ident = generate_call_method_name(fn_ident);
	let endpoint_name_str = array_literal(endpoint_name.to_string().as_bytes());
	quote! {
		#endpoint_name_str =>
		{
			self.#call_method_ident();
			true
		},
	}
}

pub fn generate_function_selector_body(
	contract: &ContractTrait,
	include_submodules: bool,
) -> proc_macro2::TokenStream {
	let match_arms: Vec<proc_macro2::TokenStream> = contract
		.methods
		.iter()
		.filter_map(|m| match &m.public_role {
			PublicRole::Init(_) => Some(function_selector_match_arm(&m, "init")),
			PublicRole::Endpoint(endpoint_metadata) => Some(function_selector_match_arm(
				&m,
				endpoint_metadata.public_name.to_string().as_str(),
			)),
			_ => None,
		})
		.collect();

	let module_arms: Vec<proc_macro2::TokenStream> = if include_submodules {
		contract
			.methods
			.iter()
			.filter_map(|m| {
				if m.is_module() {
					let method_name = &m.name;
					Some(quote! {
						if self.#method_name().call(fn_name) {
							return true;
						}
					})
				} else {
					None
				}
			})
			.collect()
	} else {
		Vec::new()
	};
	quote! {
		if match fn_name {
			b"callBack" => { self.callback(); return true; }
			#(#match_arms)*
			other => false
		} {
			return true;
		}
		#(#module_arms)*
		false
	}
}
