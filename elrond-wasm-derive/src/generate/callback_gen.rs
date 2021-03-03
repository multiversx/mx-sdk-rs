use crate::model::{ArgPaymentMetadata, Method, MethodArgument, PublicRole};

// use super::arg_def::*;
use super::{
	arg_regular::*,
	method_call_gen::{
		generate_body_with_result, generate_call_method_body, generate_call_to_method_expr,
	},
};
// use super::contract_gen_finish::*;
// use super::contract_gen_method::*;
use super::arg_str_serialize::arg_serialize_push;
use super::payable_gen::*;
use super::util::*;

pub fn generate_callback_body(methods: &[Method]) -> proc_macro2::TokenStream {
	let raw_decl = find_raw_callback(methods);
	if let Some(raw) = raw_decl {
		generate_call_method_body(&raw)
	} else {
		generate_callback_body_regular(methods)
	}
}

fn find_raw_callback(methods: &[Method]) -> Option<Method> {
	methods
		.iter()
		.find(|m| matches!(m.public_role, PublicRole::CallbackRaw))
		.cloned()
}

fn generate_callback_body_regular(methods: &[Method]) -> proc_macro2::TokenStream {
	let match_arms: Vec<proc_macro2::TokenStream> = methods
		.iter()
		.filter_map(|m| {
			if matches!(m.public_role, PublicRole::Callback) {
				let payable_snippet = generate_payable_snippet(m);
				let arg_init_snippets: Vec<proc_macro2::TokenStream> = m
					.method_args
					.iter()
					.map(|arg| {
						if matches!(
							arg.metadata.payment,
							ArgPaymentMetadata::Payment | ArgPaymentMetadata::PaymentToken
						) {
							quote! {}
						} else if arg.metadata.callback_call_result {
							// Should be an AsyncCallResult argument that wraps what comes from the async call.
							// But in principle, one can express it it any way.
							generate_load_dyn_arg(arg, &quote! { &mut ___arg_loader })
						} else {
							// callback args, loaded from storage via the tx hash
							generate_load_dyn_arg(arg, &quote! { &mut ___cb_arg_loader___ })
						}
					})
					.collect();

				let fn_ident = &m.name;
				let fn_name_str = &fn_ident.to_string();
				let fn_name_literal = array_literal(fn_name_str.as_bytes());
				let call = generate_call_to_method_expr(&m);
				let body_with_result = generate_body_with_result(&m.return_type, &call);

				let match_arm = quote! {
					#fn_name_literal =>
					{
						#payable_snippet
						let mut ___cb_arg_loader___ = CallDataArgLoader::new(cb_data_deserializer, self.api.clone());
						#(#arg_init_snippets)*
						#body_with_result ;
						___cb_arg_loader___.assert_no_more_args();
					},
				};
				Some(match_arm)
			} else {
				None
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
pub fn cb_proxy_arg_declarations(method_args: &[MethodArgument]) -> Vec<proc_macro2::TokenStream> {
	method_args
		.iter()
		.filter_map(|arg| {
			if matches!(
				arg.metadata.payment,
				ArgPaymentMetadata::Payment | ArgPaymentMetadata::PaymentToken
			) {
				None
			} else if arg.metadata.callback_call_result {
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
			if matches!(m.public_role, PublicRole::Callback) {
				let method_name = &m.name;
				let arg_decl = cb_proxy_arg_declarations(&m.method_args);
				let cb_name_literal = ident_str_literal(&method_name);

				let arg_push_snippets: Vec<proc_macro2::TokenStream> = m
					.method_args
					.iter()
					.map(|arg| {
						let arg_accumulator = quote! { &mut ___closure_arg_buffer___ };

						if let ArgPaymentMetadata::NotPayment = arg.metadata.payment {
							if arg.metadata.callback_call_result {
								quote! {}
							} else {
								arg_serialize_push(arg, &arg_accumulator)
							}
						} else {
							quote! {}
						}
					})
					.collect();
				let proxy_decl = quote! {
					pub fn #method_name ( &self , #(#arg_decl),* ) -> elrond_wasm::types::CallbackCall{
						let mut ___closure_arg_buffer___ = elrond_wasm::types::ArgBuffer::new();
						#(#arg_push_snippets)*
						elrond_wasm::types::CallbackCall::from_arg_buffer(#cb_name_literal, &___closure_arg_buffer___)
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
			A: elrond_wasm::api::ErrorApi + Clone + 'static,
		{
			pub api: A,
			_phantom1: core::marker::PhantomData<BigInt>,
			_phantom2: core::marker::PhantomData<BigUint>,
		}

		impl<A, BigInt, BigUint> CallbackProxies<A, BigInt, BigUint>
		where
			BigUint: elrond_wasm::api::BigUintApi + 'static,
			BigInt: elrond_wasm::api::BigIntApi<BigUint> + 'static,
			A: elrond_wasm::api::ErrorApi + Clone + 'static,
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
