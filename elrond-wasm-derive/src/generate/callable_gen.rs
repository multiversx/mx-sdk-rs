use super::arg_str_serialize::*;
use super::method_gen::*;
use super::util::*;
use crate::model::{ArgPaymentMetadata, CallableMethod, CallableTrait};

pub fn generate_proxy_sig(callable_method: &CallableMethod) -> proc_macro2::TokenStream {
	let method_name = &callable_method.name;
	let generics = &callable_method.generics;
	let generics_where = &callable_method.generics.where_clause;
	let arg_decl = arg_declarations(&callable_method.method_args);
	let ret_tok = match &callable_method.return_type {
		syn::ReturnType::Default => quote! {},
		syn::ReturnType::Type(_, ty) => quote! { -> #ty },
	};
	let result =
		quote! { fn #method_name #generics ( self , #(#arg_decl),* ) #ret_tok #generics_where };
	result
}

pub fn extract_pub_method_sigs(callable_trait: &CallableTrait) -> Vec<proc_macro2::TokenStream> {
	callable_trait
		.methods
		.iter()
		.map(|m| {
			let sig = generate_proxy_sig(&m);
			quote! { #sig ; }
		})
		.collect()
}

pub fn generate_method_impl(callable_trait: &CallableTrait) -> Vec<proc_macro2::TokenStream> {
	callable_trait
		.methods
		.iter()
		.map(|m| {
			let msig = generate_proxy_sig(&m);

			let mut payment_count = 0;
			let mut payment_expr = quote! { self.payment };
			let mut token_count = 0;
			let mut token_expr = quote! { self.token };
			let arg_push_snippets: Vec<proc_macro2::TokenStream> = m
				.method_args
				.iter()
				.map(|arg| {
					let arg_accumulator = quote! { ___contract_call___.get_mut_arg_buffer() };

					match &arg.metadata.payment {
						ArgPaymentMetadata::NotPayment => arg_serialize_push(arg, &arg_accumulator, &quote!{ self.api.clone() }),
						ArgPaymentMetadata::Payment => {
							payment_count += 1;
							let pat = &arg.pat;
							payment_expr = quote! { #pat };

							quote! {}
						},
						ArgPaymentMetadata::PaymentToken => {
							token_count += 1;
							let pat = &arg.pat;
							token_expr = quote! { #pat };

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

			let m_name_literal = ident_str_literal(&m.name);
			let sig = quote! {
				#msig {
					let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
						self.address,
						#token_expr,
						#payment_expr,
						elrond_wasm::types::BoxedBytes::from(#m_name_literal));
					#(#arg_push_snippets)*
					___contract_call___
				}
			};
			sig
		})
		.collect()
}
