use super::arg_def::*;
use super::arg_regular::*;
use super::contract_gen_finish::*;
use super::contract_gen_method::*;
use super::contract_gen_payable::*;
use super::util::*;

pub fn generate_callback_body(methods: &[Method]) -> proc_macro2::TokenStream {
	let raw_decl = find_raw_callback(methods);
	if let Some(raw) = raw_decl {
		raw.generate_call_method_body()
	} else {
		generate_callback_body_regular(methods)
	}
}

fn find_raw_callback(methods: &[Method]) -> Option<Method> {
	methods
		.iter()
		.find(|m| matches!(m.metadata, MethodMetadata::CallbackRaw))
		.cloned()
}

fn generate_callback_body_regular(methods: &[Method]) -> proc_macro2::TokenStream {
	let match_arms: Vec<proc_macro2::TokenStream> = methods
		.iter()
		.filter_map(|m| {
			match m.metadata {
				MethodMetadata::Callback => {
					let payable_snippet = generate_payable_snippet(m);
					let arg_init_snippets: Vec<proc_macro2::TokenStream> = m
						.method_args
						.iter()
						.map(|arg| {
							if arg.is_callback_arg {
								// callback args, loaded from storage via the tx hash
								match &arg.metadata {
									ArgMetadata::Single => generate_load_dyn_arg(
										arg,
										&quote! { &mut ___cb_arg_loader },
									),
									ArgMetadata::Payment => {
										panic!("payment args not allowed in callbacks")
									},
									ArgMetadata::PaymentToken => {
										panic!("payment token args not allowed in callbacks")
									},
									ArgMetadata::VarArgs => {
										panic!("callback var_args not yet supported")
									},
								}
							} else {
								// Should be an AsyncCallResult argument that wraps what comes from the async call.
								// But in principle, one can express it it any way.
								match &arg.metadata {
									ArgMetadata::Single | ArgMetadata::VarArgs => {
										generate_load_dyn_arg(arg, &quote! { &mut ___arg_loader })
									},
									ArgMetadata::Payment | ArgMetadata::PaymentToken => quote! {},
								}
							}
						})
						.collect();

					let fn_ident = &m.name;
					let fn_name_str = &fn_ident.to_string();
					let fn_name_literal = array_literal(fn_name_str.as_bytes());
					let call = m.generate_call_to_method();
					let body_with_result = generate_body_with_result(&m.return_type, &call);

					let match_arm = quote! {
						#fn_name_literal =>
						{
							#payable_snippet
							let mut ___cb_arg_loader = CallDataArgLoader::new(cb_data_deserializer, self.api.clone());
							#(#arg_init_snippets)*
							#body_with_result ;
							___cb_arg_loader.assert_no_more_args();
						},
					};
					Some(match_arm)
				},
				_ => None,
			}
		})
		.collect();
	if match_arms.is_empty() {
		// no callback code needed
		quote! {}
	} else {
		quote! {
			let cb_data_raw = self.api.storage_load_vec_u8(&self.api.get_tx_hash().as_ref());
			let mut cb_data_deserializer = elrond_wasm::hex_call_data::HexCallDataDeserializer::new(cb_data_raw.as_slice());
			let mut ___arg_loader = EndpointDynArgLoader::new(self.api.clone());

			match cb_data_deserializer.get_func_name() {
				[] => { return; }
				#(#match_arms)*
				other => self.api.signal_error(err_msg::CALLBACK_BAD_FUNC)
			}

			___arg_loader.assert_no_more_args();

			// cleanup
			self.api.storage_store_slice_u8(&self.api.get_tx_hash().as_ref(), &[]);
		}
	}
}
