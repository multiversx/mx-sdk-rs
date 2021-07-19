use super::generate::{abi_gen, snippets};
use crate::generate::callback_gen::*;
use crate::generate::callback_proxies_gen::*;
use crate::generate::contract_gen::*;
use crate::generate::function_selector::generate_function_selector_body;
use crate::generate::proxy_gen;
use crate::generate::supertrait_gen;
use crate::generate::{
	auto_impl::generate_auto_impls, auto_impl_proxy::generate_all_proxy_trait_imports,
};
use crate::model::ContractTrait;

/// Provides the implementation for both modules and contracts.
/// TODO: not a great pattern to have the `is_contract_main` flag, reorganize the code and get rid of it.
pub fn contract_implementation(
	contract: &ContractTrait,
	is_contract_main: bool,
) -> proc_macro2::TokenStream {
	let proxy_trait_imports = generate_all_proxy_trait_imports(contract);
	let module_original_attributes = &contract.original_attributes;
	let trait_name_ident = contract.trait_name.clone();
	let method_impls = extract_method_impls(contract);
	let call_methods = generate_call_methods(contract);
	let auto_impl_defs = generate_auto_impl_defs(contract);
	let auto_impls = generate_auto_impls(contract);
	let endpoints = generate_wasm_endpoints(contract);
	let function_selector_body = generate_function_selector_body(contract);
	let (callback_selector_body, callback_body) = generate_callback_selector_and_main(contract);
	let where_self_big_int = snippets::where_self_big_int();

	let (callbacks_def, callbacks_impl, callback_proxies_obj) = generate_callback_proxies(contract);

	// this definition is common to release and debug mode
	let supertraits_main = supertrait_gen::main_supertrait_decl(contract.supertraits.as_slice());
	let main_definition = quote! {
		#(#proxy_trait_imports)*

		#(#module_original_attributes)*
		pub trait #trait_name_ident:
		elrond_wasm::api::ContractBase
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

			fn callback_selector<'a>(&self, mut ___cb_data_deserializer___: elrond_wasm::hex_call_data::HexCallDataDeserializer<'a>) -> elrond_wasm::types::CallbackSelectorResult<'a> {
				#callback_selector_body
			}

			fn callback(&self) {
				#callback_body
			}
		}
	};

	let abi_provider = abi_gen::generate_abi_provider(contract, is_contract_main);

	let module_traits_code = quote! {
		#main_definition

		#auto_impl_trait

		#endpoint_wrappers

		#abi_provider
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

	let proxy_trait = proxy_gen::proxy_trait(contract);
	let proxy_obj_code = if is_contract_main {
		proxy_gen::proxy_obj_code(contract)
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
}
