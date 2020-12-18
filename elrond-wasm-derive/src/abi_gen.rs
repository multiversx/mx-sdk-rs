use super::arg_def::*;
use super::contract_gen::*;
use super::contract_gen_method::*;
use super::util::*;

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

				let input_snippets: Vec<proc_macro2::TokenStream> = m
					.method_args
					.iter()
					.filter_map(|arg| {
						if let ArgMetadata::Payment = arg.metadata {
							None
						} else {
							let mut arg_type = arg.ty.clone();
							clear_all_type_lifetimes(&mut arg_type);
							let arg_name = &arg.pat;
							let arg_name_str = quote! { #arg_name }.to_string();
							Some(quote! {
								endpoint_abi.add_input::<#arg_type>(#arg_name_str);
								contract_abi.add_type_descriptions::<#arg_type>();
							})
						}
					})
					.collect();

				let output_snippet = match &m.return_type {
					syn::ReturnType::Default => quote! {},
					syn::ReturnType::Type(_, ty) => {
						let mut res_type = ty.clone();
						clear_all_type_lifetimes(&mut res_type);
						quote! {
							endpoint_abi.add_output::<#res_type>();
							contract_abi.add_type_descriptions::<#res_type>();
						}
					},
				};

				Some(quote! {
					let mut endpoint_abi = elrond_wasm::abi::EndpointAbi{
						docs: &[ #(#endpoint_docs),* ],
						name: #endpoint_name_str,
						payable: #payable,
						inputs: Vec::new(),
						outputs: Vec::new(),
					};
					#(#input_snippets)*
					#output_snippet
					contract_abi.endpoints.push(endpoint_abi);
				})
			} else if let MethodMetadata::Module { .. } = &m.metadata {
				let method_name = &m.name;
				Some(quote! {
					if include_modules {
						contract_abi.coalesce(self.#method_name().abi(false));
					}
				})
			} else {
				None
			}
		})
		.collect();

	let contract_docs = &contract.docs;
	let contract_name = &contract.trait_name.to_string();
	quote! {
		let mut contract_abi = elrond_wasm::abi::ContractAbi{
			docs: &[ #(#contract_docs),* ],
			name: #contract_name,
			endpoints: Vec::new(),
			type_descriptions: <elrond_wasm::abi::TypeDescriptionContainerImpl as elrond_wasm::abi::TypeDescriptionContainer>::new(),
		};
		#(#endpoint_snippets)*
		contract_abi
	}
}
