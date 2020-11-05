use super::contract_gen::*;
use super::contract_gen_method::*;
use super::util::*;

pub fn generate_function_selector_body(
	contract: &Contract,
	include_submodules: bool,
) -> proc_macro2::TokenStream {
	let match_arms: Vec<proc_macro2::TokenStream> = contract
		.methods
		.iter()
		.filter_map(|m| {
			if let Some(endpoint_name) = m.metadata.endpoint_name() {
				let fn_ident = &m.name;
				let call_method_ident = generate_call_method_name(fn_ident);
				let endpoint_name_str = array_literal(endpoint_name.to_string().as_bytes());
				let match_arm = quote! {
					#endpoint_name_str =>
					{
						self.#call_method_ident();
						true
					},
				};
				Some(match_arm)
			} else {
				None
			}
		})
		.collect();

	let module_arms: Vec<proc_macro2::TokenStream> = if include_submodules {
		contract
			.methods
			.iter()
			.filter_map(|m| {
				if let MethodMetadata::Module { .. } = &m.metadata {
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
