use super::contract_gen::*;
use super::contract_gen_method::*;

pub fn generate_abi_method_body(contract: &Contract) -> proc_macro2::TokenStream {
	let endpoint_snippets: Vec<proc_macro2::TokenStream> = contract
		.methods
		.iter()
		.filter_map(|m| {
			if let Some(endpoint_name) = m.metadata.endpoint_name() {
				let endpoint_docs = &m.docs;
				let endpoint_name_str = endpoint_name.to_string();
				let payable = if let MethodMetadata::Regular { payable, .. } = m.metadata {
					payable
				} else {
					false
				};

				Some(quote! {
					abi.endpoints.push(elrond_wasm::abi::EndpointAbi{
						docs: &[ #(#endpoint_docs),* ],
						name: #endpoint_name_str,
						payable: #payable,
					});
				})
			} else if let MethodMetadata::Module { .. } = &m.metadata {
				let method_name = &m.name;
				Some(quote! {
					if include_modules {
						abi.coalesce(self.#method_name().abi(false));
					}
				})
			} else {
				None
			}
		})
		.collect();

	let contract_docs = &contract.docs;
	quote! {
		let mut abi = elrond_wasm::abi::ContractAbi{
			docs: &[ #(#contract_docs),* ],
			endpoints: Vec::new(),
		};
		#(#endpoint_snippets)*
		abi
	}
}
