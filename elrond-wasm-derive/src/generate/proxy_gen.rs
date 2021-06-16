use super::arg_str_serialize::*;
use super::method_gen::*;
use super::util::*;
use crate::{
	generate::{snippets, supertrait_gen},
	model::{ArgPaymentMetadata, ContractTrait, Method},
};

pub fn generate_proxy_sig(method: &Method) -> proc_macro2::TokenStream {
	let method_name = &method.name;
	let generics = &method.generics;
	let generics_where = &method.generics.where_clause;
	let arg_decl = arg_declarations(&method.method_args);
	let ret_tok = match &method.return_type {
		syn::ReturnType::Default => quote! { () },
		syn::ReturnType::Type(_, ty) => quote! { #ty },
	};
	let result = quote! {
		fn #method_name #generics (
			self,
			#(#arg_decl),*
		) -> elrond_wasm::types::ContractCall<Self::SendApi, <#ret_tok as elrond_wasm::io::EndpointResult>::DecodeAs>
		#generics_where
	};
	result
}

pub fn generate_method_impl(contract_trait: &ContractTrait) -> Vec<proc_macro2::TokenStream> {
	contract_trait
		.methods
		.iter()
		.filter_map(|m| {
			if let Some(endpoint_name) = m.endpoint_name() {
				let msig = generate_proxy_sig(m);

				let mut payment_count = 0;
				let mut payment_local_decl = quote! { ___payment___ };
				let mut payment_expr = quote! { ___payment___ };
				let mut token_count = 0;
				let mut token_local_decl = quote! { ___token___ };
				let mut token_expr = quote! { ___token___ };
				let mut nonce_count = 0;
				let mut nonce_local_decl = quote! { ___nonce___ };
				let mut nonce_expr = quote! { ___nonce___ };

				let arg_push_snippets: Vec<proc_macro2::TokenStream> = m
					.method_args
					.iter()
					.map(|arg| {
						let arg_accumulator = quote! { ___contract_call___.get_mut_arg_buffer() };

						match &arg.metadata.payment {
							ArgPaymentMetadata::NotPayment => arg_serialize_push(
								arg,
								&arg_accumulator,
								&quote! { ___api___.clone() },
							),
							ArgPaymentMetadata::PaymentAmount => {
								payment_count += 1;
								let pat = &arg.pat;
								payment_local_decl = quote! { _ };
								payment_expr = quote! { #pat };

								quote! {}
							},
							ArgPaymentMetadata::PaymentToken => {
								token_count += 1;
								let pat = &arg.pat;
								token_local_decl = quote! { _ };
								token_expr = quote! { #pat };

								quote! {}
							},
							ArgPaymentMetadata::PaymentNonce => {
								nonce_count += 1;
								let pat = &arg.pat;
								nonce_local_decl = quote! { _ };
								nonce_expr = quote! { #pat };

								quote! {}
							},
						}
					})
					.collect();

				if payment_count > 1 {
					panic!("No more than one payment argument allowed in call proxy");
				}
				if token_count > 1 {
					panic!("No more than one payment token argument allowed in call proxy");
				}
				if nonce_count > 1 {
					panic!("No more than one payment nonce argument allowed in call proxy");
				}

				let endpoint_name_literal = byte_str_slice_literal(endpoint_name.as_bytes());
				let sig = quote! {
					#msig {
						let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
							self.into_fields();
						let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
							___api___.clone(),
							___address___,
							#token_expr,
							#payment_expr,
							#nonce_expr,
							elrond_wasm::types::BoxedBytes::from(#endpoint_name_literal),
						);
						#(#arg_push_snippets)*
						___contract_call___
					}
				};
				Some(sig)
			} else {
				None
			}
		})
		.collect()
}

pub fn proxy_trait(contract: &ContractTrait) -> proc_macro2::TokenStream {
	let proxy_supertrait_decl =
		supertrait_gen::proxy_supertrait_decl(contract.supertraits.as_slice());
	let proxy_methods_impl = generate_method_impl(contract);
	quote! {
		pub trait ProxyTrait:
			elrond_wasm::api::ProxyObjApi
			+ Sized
			#(#proxy_supertrait_decl)*
		{
			#(#proxy_methods_impl)*
		}
	}
}

pub fn proxy_obj_code(contract: &ContractTrait) -> proc_macro2::TokenStream {
	let proxy_object_def = snippets::proxy_object_def();
	let impl_all_proxy_traits =
		supertrait_gen::impl_all_proxy_traits(contract.supertraits.as_slice());
	quote! {
		#proxy_object_def

		#(#impl_all_proxy_traits)*
	}
}
