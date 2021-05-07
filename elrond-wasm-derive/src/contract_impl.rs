use super::generate::{abi_gen, snippets};
use crate::generate::auto_impl::generate_auto_impls;
use crate::generate::callback_gen::*;
use crate::generate::callback_proxies_gen::*;
use crate::generate::contract_gen::*;
use crate::generate::function_selector::generate_function_selector_body;
use crate::generate::proxy_gen;
use crate::generate::supertrait_gen;
use crate::model::ContractTrait;

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
	let function_selector_body = generate_function_selector_body(&contract);
	let callback_body = generate_callback_body(&contract.methods);
	let where_self_big_int = snippets::where_self_big_int();

	let (callbacks_def, callbacks_impl, callback_proxies_obj) =
		generate_callback_proxies(&contract);

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

			#callbacks_def
		}
	};

	let auto_impl_trait = quote! {
		pub trait AutoImpl: elrond_wasm::api::ContractBase {}

		impl<C> #trait_name_ident for C
		#where_self_big_int
		C: AutoImpl #(#supertraits_main)*
		{
			#(#auto_impls)*

			#callbacks_impl
		}
	};

	let endpoint_wrapper_supertrait_decl =
		supertrait_gen::endpoint_wrapper_supertrait_decl(contract.supertraits.as_slice());
	let endpoint_wrappers = quote! {
		pub trait EndpointWrappers:
			elrond_wasm::api::ContractPrivateApi
			+ #trait_name_ident
			#(#endpoint_wrapper_supertrait_decl)*
		#where_self_big_int
		{
			#(#call_methods)*

			fn call(&self, fn_name: &[u8]) -> bool {
				#function_selector_body
			}

			fn callback(&self) {
				#callback_body
			}
		}
	};

	let abi_body = abi_gen::generate_abi_method_body(&contract, is_contract_main);
	let abi = quote! {
		pub struct AbiProvider {}

		impl elrond_wasm::api::ContractAbiProvider for AbiProvider {
			type Storage = elrond_wasm::api::uncallable::StorageApiUncallable;
			type BigUint = elrond_wasm::api::uncallable::BigUintUncallable;
			type BigInt = elrond_wasm::api::uncallable::BigIntUncallable;

			fn abi() -> elrond_wasm::abi::ContractAbi {
				#abi_body
			}
		}
	};

	let callback_proxy = quote! {
		// #callback_proxies
	};

	let module_traits_code = quote! {
		#main_definition

		#auto_impl_trait

		#endpoint_wrappers

		#callback_proxy

		#abi
	};

	let contract_object_def = snippets::contract_object_def();
	let impl_contract_base = snippets::impl_contract_base();
	let impl_all_auto_impl = supertrait_gen::impl_all_auto_impl(contract.supertraits.as_slice());
	let impl_private_api = snippets::impl_private_api();
	let impl_all_endpoint_wrappers =
		supertrait_gen::impl_all_endpoint_wrappers(contract.supertraits.as_slice());
	let impl_callable_contract = snippets::impl_callable_contract();
	let new_contract_object_fn = snippets::new_contract_object_fn();

	let contract_obj_code = quote! {

		#contract_object_def

		#impl_contract_base

		#(#impl_all_auto_impl)*

		#impl_private_api

		#(#impl_all_endpoint_wrappers)*

		#impl_callable_contract

		#new_contract_object_fn
	};

	let wasm_callback_fn = if is_contract_main {
		quote! {
			#[no_mangle]
			pub fn callBack () {
				let inst = super::endpoints::new_arwen_instance();
				super::EndpointWrappers::callback(&inst);
			}
		}
	} else {
		quote! {}
	};

	let wasm_endpoints = quote! {
		#[cfg(feature = "wasm-output-mode")]
		#[allow(non_snake_case)]
		mod endpoints {
			use super::*;

			fn new_arwen_instance() -> super::ContractObj<elrond_wasm_node::ArwenApiImpl> {
				let api = elrond_wasm_node::ArwenApiImpl{};
				super::contract_obj(api)
			}

			#(#endpoints)*

			#wasm_callback_fn
		}
	};

	let proxy_supertrait_decl =
		supertrait_gen::proxy_supertrait_decl(contract.supertraits.as_slice());
	let proxy_methods_impl = proxy_gen::generate_method_impl(&contract);
	let proxy_trait = quote! {
		pub trait Proxy:
			elrond_wasm::api::ProxyObjApi
			+ Sized
			#(#proxy_supertrait_decl)*
		{
			#(#proxy_methods_impl)*
		}
	};

	let proxy_obj_code = if is_contract_main {
		let proxy_object_def = snippets::proxy_object_def();
		let impl_all_proxy_traits =
			supertrait_gen::impl_all_proxy_traits(contract.supertraits.as_slice());
		quote! {
			#proxy_object_def

			#(#impl_all_proxy_traits)*
		}
	} else {
		quote! {}
	};

	quote! {
		#module_traits_code

		#contract_obj_code

		#wasm_endpoints

		#proxy_trait

		#proxy_obj_code

		#callback_proxies_obj
	}

	// if is_contract_main {
	// 	quote! {
	// 		#module_code

	// 		#contract_only_code
	// 	}
	// } else {
	// 	module_code
	// }
}
