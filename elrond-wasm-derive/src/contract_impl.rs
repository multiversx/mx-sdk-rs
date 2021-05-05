use crate::model::ContractTrait;

use super::generate::{abi_gen, snippets};
use crate::generate::auto_impl::generate_auto_impls;
use crate::generate::callback_gen::*;
use crate::generate::contract_gen::*;
use crate::generate::function_selector::generate_function_selector_body;
use crate::generate::supertrait_gen;

pub fn contract_implementation(
	contract: &ContractTrait,
	is_contract_main: bool,
) -> proc_macro2::TokenStream {
	let trait_name_ident = contract.trait_name.clone();
	let method_impls = extract_method_impls(&contract);
	let call_methods = generate_call_methods(&contract);
	let auto_impl_defs = generate_auto_impl_defs(&contract);
	let auto_impls = generate_auto_impls(&contract);
	let endpoints = generate_wasm_endpoints(&contract);
	let function_selector_body = generate_function_selector_body(&contract, is_contract_main);
	let abi_body = abi_gen::generate_abi_method_body(&contract);
	let callback_body = generate_callback_body(&contract.methods);
	let callback_proxies = generate_callback_proxies(&contract.methods);
	let where_self_big_int = snippets::where_self_big_int();

	// this definition is common to release and debug mode
	let supertraits_main = supertrait_gen::main_supertrait_decl(contract.supertraits.as_slice());
	let main_definition = quote! {
		pub trait #trait_name_ident:
		ContractBase
		+ Sized
		#(#supertraits_main)*
		#where_self_big_int
		{
			#(#method_impls)*

			#(#auto_impl_defs)*

			fn callback(&self);

			// fn callbacks(&self) -> callback_proxy::CallbackProxies<T, BigInt, BigUint>;
		}
	};

	let auto_impl_trait = quote! {
		pub trait AutoImpl: elrond_wasm::api::ContractBase {}

		impl<C> #trait_name_ident for C
		#where_self_big_int
		C: AutoImpl #(#supertraits_main)*
		{
			#(#auto_impls)*

			fn callback(&self) {
				#callback_body
			}

			// fn callbacks(&self) -> super::callback_proxy::CallbackProxies<T, BigInt, BigUint> {
			// 	super::callback_proxy::CallbackProxies::new(self.api.clone())
			// }
		}
	};

	let endpoint_wrapper_supertrait_decl =
		supertrait_gen::endpoint_wrapper_supertrait_decl(contract.supertraits.as_slice());
	let endpoint_wrappers = quote! {
		pub trait EndpointWrappers:
			elrond_wasm::api::ContractPrivateApi 
			+ #trait_name_ident #(#endpoint_wrapper_supertrait_decl)*
		#where_self_big_int
		{
			#(#call_methods)*

			fn call(&self, fn_name: &[u8]) -> bool {
				#function_selector_body
			}
		}
	};

	let abi = quote! {
		// impl <T, BigInt, BigUint> elrond_wasm::api::ContractWithAbi for #trait_name_ident<T, BigInt, BigUint>
		// #api_where
		// {
		// 	type Storage = T::Storage;

		// 	fn abi(&self, include_modules: bool) -> elrond_wasm::abi::ContractAbi{
		// 		#abi_body
		// 	}
		// }
	};

	let callback_proxy = quote! {
		// #callback_proxies
	};

	let new_contract_object_fn = snippets::new_contract_object_fn();

	let wasm_endpoints = quote! {
		#[cfg(feature = "wasm-output-mode")]
		#[allow(non_snake_case)]
		pub mod endpoints {
			use super::*;

			fn new_arwen_instance() -> super::ContractObj {
				let api = elrond_wasm_node::ArwenApiImpl{};
				elrond_wasm::api::new_contract_impl(api)
			}

			#(#endpoints)*
		}
	};

	let module_code = quote! {
		#main_definition

		#auto_impl_trait

		#endpoint_wrappers

		#callback_proxy

		#wasm_endpoints

		#abi
	};

	let contract_object_def = snippets::contract_object_def();
	let impl_contract_base = snippets::impl_contract_base();
	let impl_all_auto_impl = supertrait_gen::impl_all_auto_impl(contract.supertraits.as_slice());
	let impl_private_api = snippets::impl_private_api();
	let impl_all_endpoint_wrappers =
		supertrait_gen::impl_all_endpoint_wrappers(contract.supertraits.as_slice());
	let impl_callable_contract = snippets::impl_callable_contract();

	let contract_only_code = quote! {
		#contract_object_def

		#impl_contract_base

		#(#impl_all_auto_impl)*

		#impl_private_api

		#(#impl_all_endpoint_wrappers)*

		#impl_callable_contract

		#new_contract_object_fn
	};

	if is_contract_main {
		quote! {
			#module_code

			#contract_only_code
		}
	} else {
		module_code
	}
}
