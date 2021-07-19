use super::arg_str_serialize::*;
use super::method_gen::*;
use super::util::*;
use crate::model::PublicRole;
use crate::{
	generate::{snippets, supertrait_gen},
	model::{ArgPaymentMetadata, ContractTrait, Method},
};

pub fn generate_proxy_endpoint_sig(method: &Method) -> proc_macro2::TokenStream {
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

pub fn generate_proxy_deploy_sig(method: &Method) -> proc_macro2::TokenStream {
	let method_name = &proc_macro2::Ident::new(&"contract_deploy", proc_macro2::Span::call_site());
	let generics = &method.generics;
	let generics_where = &method.generics.where_clause;
	let arg_decl = arg_declarations(&method.method_args);
	let result = quote! {
		fn #method_name #generics (
			self,
			#(#arg_decl),*
		) -> elrond_wasm::types::ContractDeploy<Self::SendApi>
		#generics_where
	};
	result
}

pub fn generate_proxy_endpoint(m: &Method, endpoint_name: String) -> proc_macro2::TokenStream {
	let msig = generate_proxy_endpoint_sig(m);

	let mut payment_count = 0;
	let mut payment_expr = quote! { ___payment___ };
	let mut token_count = 0;
	let mut token_expr = quote! { ___token___ };
	let mut nonce_count = 0;
	let mut nonce_expr = quote! { ___nonce___ };

	let arg_push_snippets: Vec<proc_macro2::TokenStream> = m
		.method_args
		.iter()
		.map(|arg| {
			let arg_accumulator = quote! { ___contract_call___.get_mut_arg_buffer() };

			match &arg.metadata.payment {
				ArgPaymentMetadata::NotPayment => {
					arg_serialize_push(arg, &arg_accumulator, &quote! { ___api___.clone() })
				},
				ArgPaymentMetadata::PaymentAmount => {
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
				ArgPaymentMetadata::PaymentNonce => {
					nonce_count += 1;
					let pat = &arg.pat;
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
		#[allow(clippy::too_many_arguments)]
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

	sig
}

pub fn generate_proxy_deploy(init_method: &Method) -> proc_macro2::TokenStream {
	let msig = generate_proxy_deploy_sig(init_method);

	let mut payment_count = 0;
	let mut payment_expr = quote! { ___payment___ };
	let mut token_count = 0;
	let mut nonce_count = 0;

	let arg_push_snippets: Vec<proc_macro2::TokenStream> = init_method
		.method_args
		.iter()
		.map(|arg| {
			let arg_accumulator = quote! { ___contract_deploy___.get_mut_arg_buffer() };

			match &arg.metadata.payment {
				ArgPaymentMetadata::NotPayment => {
					arg_serialize_push(arg, &arg_accumulator, &quote! { ___api___.clone() })
				},
				ArgPaymentMetadata::PaymentAmount => {
					payment_count += 1;
					let pat = &arg.pat;
					payment_expr = quote! { #pat };

					quote! {}
				},
				ArgPaymentMetadata::PaymentToken => {
					token_count += 1;

					quote! {}
				},
				ArgPaymentMetadata::PaymentNonce => {
					nonce_count += 1;

					quote! {}
				},
			}
		})
		.collect();

	if payment_count > 1 {
		panic!("No more than one payment argument allowed in call proxy");
	}
	if token_count > 0 {
		panic!("No ESDT payment allowed in #init");
	}
	if nonce_count > 1 {
		panic!("No SFT/NFT payment allowed in #init");
	}

	let sig = quote! {
		#[allow(clippy::too_many_arguments)]
		#msig {
			let (___api___, ___code___, ___code_metadata___, ___payment___) =
				self.into_fields();
			let mut ___contract_deploy___ = elrond_wasm::types::new_contract_deploy(
				___api___.clone(),
				___code___,
				___code_metadata___,
				#payment_expr,
			);
			#(#arg_push_snippets)*
			___contract_deploy___
		}
	};

	sig
}

pub fn generate_method_impl(contract_trait: &ContractTrait) -> Vec<proc_macro2::TokenStream> {
	contract_trait
		.methods
		.iter()
		.filter_map(|m| match &m.public_role {
			PublicRole::Endpoint(endpoint_metadata) => Some(generate_proxy_endpoint(
				m,
				endpoint_metadata.public_name.to_string(),
			)),
			_ => None,
		})
		.collect()
}

pub fn generate_deploy_impl(contract_trait: &ContractTrait) -> Option<proc_macro2::TokenStream> {
	let opt_init = contract_trait.methods.iter().find(|&m| {
		if let PublicRole::Init(_) = m.public_role {
			true
		} else {
			false
		}
	});

	opt_init.map(|m| generate_proxy_deploy(m))
}

pub fn proxy_trait(contract: &ContractTrait) -> proc_macro2::TokenStream {
	let proxy_supertrait_decl =
		supertrait_gen::proxy_supertrait_decl(contract.supertraits.as_slice(), &"ProxyTrait");
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
		supertrait_gen::impl_all_proxy_traits(contract.supertraits.as_slice(), &"ProxyTrait", &"Proxy");
	quote! {
		#proxy_object_def

		#(#impl_all_proxy_traits)*
	}
}

pub fn deploy_proxy_trait(contract: &ContractTrait) -> proc_macro2::TokenStream {
	let proxy_supertrait_decl =
		supertrait_gen::proxy_supertrait_decl(contract.supertraits.as_slice(), &"DeployProxyTrait");
	let init_method_impl = generate_deploy_impl(contract).unwrap();
	quote! {
		pub trait DeployProxyTrait:
			elrond_wasm::api::DeployProxyObjApi
			+ Sized
			#(#proxy_supertrait_decl)*
		{
			#init_method_impl
		}
	}
}

pub fn deploy_proxy_obj_code(contract: &ContractTrait) -> proc_macro2::TokenStream {
	let proxy_object_def = snippets::deploy_proxy_object_def();
	let impl_all_proxy_traits =
		supertrait_gen::impl_all_proxy_traits(contract.supertraits.as_slice(), &"DeployProxyTrait", &"DeployProxy");
	quote! {
		#proxy_object_def

		#(#impl_all_proxy_traits)*
	}
}
