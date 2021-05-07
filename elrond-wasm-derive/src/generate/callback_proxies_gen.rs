use super::{
	arg_str_serialize::arg_serialize_push,
	util::*,
};
use crate::model::{ArgPaymentMetadata, ContractTrait, Method, MethodArgument, PublicRole};

/// Excludes the `#[call_result]`.
pub fn cb_proxy_arg_declarations(method_args: &[MethodArgument]) -> Vec<proc_macro2::TokenStream> {
	method_args
		.iter()
		.filter_map(|arg| {
			if matches!(
				arg.metadata.payment,
				ArgPaymentMetadata::Payment | ArgPaymentMetadata::PaymentToken
			) || arg.metadata.callback_call_result
			{
				None
			} else {
				let pat = &arg.pat;
				let ty = &arg.ty;
				Some(quote! {#pat : #ty })
			}
		})
		.collect()
}

pub fn generate_callback_proxies_object(methods: &[Method]) -> proc_macro2::TokenStream {
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
								arg_serialize_push(
									arg,
									&arg_accumulator,
									&quote! { self.api.clone() },
								)
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
		pub struct CallbackProxies<A>
		where
			A: elrond_wasm::api::ErrorApi + Clone + 'static,
		{
			pub api: A,
		}

		impl<A> CallbackProxies<A>
		where
			A: elrond_wasm::api::ErrorApi + Clone + 'static,
		{
			pub fn new(api: A) -> Self {
				CallbackProxies { api }
			}

			#(#proxy_methods)*
		}
	}
}

pub fn generate_callback_proxies(
	contract: &ContractTrait,
) -> (
	proc_macro2::TokenStream,
	proc_macro2::TokenStream,
	proc_macro2::TokenStream,
) {
	if contract.callback_count() == 0 {
		(quote! {}, quote! {}, quote! {})
	} else {
		(
			quote! {
				fn callbacks(&self) -> self::CallbackProxies<Self::ErrorApi>;
			},
			quote! {
				fn callbacks(&self) -> self::CallbackProxies<Self::ErrorApi> {
					self::CallbackProxies::new(self.error_api())
				}
			},
			generate_callback_proxies_object(contract.methods.as_slice()),
		)
	}
}
