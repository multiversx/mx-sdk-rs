use super::arg_def::*;
use super::arg_regular::*;
use super::contract_gen_finish::*;
use super::contract_gen_method::*;
use super::contract_gen_payable::*;
use super::util::*;
use crate::arg_str_serialize::arg_serialize_push;

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
							match &arg.metadata {
								ArgMetadata::Single | ArgMetadata::VarArgs => {
									// callback args, loaded from storage via the tx hash
									generate_load_dyn_arg(arg, &quote! { &mut ___cb_arg_loader })
								},
								ArgMetadata::Payment => {
									panic!("payment args not allowed in callbacks")
								},
								ArgMetadata::PaymentToken => {
									panic!("payment token args not allowed in callbacks")
								},
								ArgMetadata::AsyncCallResultArg => {
									// Should be an AsyncCallResult argument that wraps what comes from the async call.
									// But in principle, one can express it it any way.
									generate_load_dyn_arg(arg, &quote! { &mut ___arg_loader })
								},
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

/// Excludes the `#[call_result]`.
pub fn cb_proxy_arg_declarations(method_args: &[MethodArg]) -> Vec<proc_macro2::TokenStream> {
	method_args
		.iter()
		.filter_map(|arg| {
			if let ArgMetadata::AsyncCallResultArg = arg.metadata {
				None
			} else {
				let pat = &arg.pat;
				let ty = &arg.ty;
				Some(quote! {#pat : #ty })
			}
		})
		.collect()
}

pub fn generate_callback_proxies(methods: &[Method]) -> proc_macro2::TokenStream {
	let proxy_methods: Vec<proc_macro2::TokenStream> = methods
		.iter()
		.filter_map(|m| {
			if matches!(m.metadata, MethodMetadata::Callback) {
				let method_name = &m.name;
				let arg_decl = cb_proxy_arg_declarations(&m.method_args);
				let cb_name_literal = ident_str_literal(&method_name);

				let arg_push_snippets: Vec<proc_macro2::TokenStream> = m
					.method_args
					.iter()
					.map(|arg| {
						let arg_accumulator = quote! { closure_data };

						match &arg.metadata {
							ArgMetadata::Single | ArgMetadata::VarArgs => {
								arg_serialize_push(arg, &arg_accumulator)
							},
							ArgMetadata::Payment
							| ArgMetadata::PaymentToken
							| ArgMetadata::AsyncCallResultArg => quote! {},
						}
					})
					.collect();
				let proxy_decl = quote! {
					pub fn #method_name ( &self , #(#arg_decl),* ) -> elrond_wasm::types::CallbackCall{
						let mut closure_data = elrond_wasm::hex_call_data::HexCallDataSerializer::new(#cb_name_literal);
						#(#arg_push_snippets)*
						elrond_wasm::types::CallbackCall::from_raw(closure_data)
					}

				};

				Some(proxy_decl)
			} else {
				None
			}
		})
		.collect();

	quote! {
		pub struct CallbackProxies<A, BigInt, BigUint>
		where
			BigUint: elrond_wasm::api::BigUintApi + 'static,
			BigInt: elrond_wasm::api::BigIntApi<BigUint> + 'static,
			A: elrond_wasm::api::ErrorApi + 'static,
		{
			pub api: A,
			_phantom1: core::marker::PhantomData<BigInt>,
			_phantom2: core::marker::PhantomData<BigUint>,
		}

		impl<A, BigInt, BigUint> CallbackProxies<A, BigInt, BigUint>
		where
			BigUint: elrond_wasm::api::BigUintApi + 'static,
			BigInt: elrond_wasm::api::BigIntApi<BigUint> + 'static,
			A: elrond_wasm::api::ErrorApi + 'static,
		{
			pub fn new(api: A) -> Self {
				CallbackProxies {
					api,
					_phantom1: core::marker::PhantomData,
					_phantom2: core::marker::PhantomData,
				}
			}

			#(#proxy_methods)*
		}
	}
}
