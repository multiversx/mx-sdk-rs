use crate::model::ContractTrait;

use super::generate::{abi_gen, snippets};
use crate::generate::auto_impl::generate_auto_impls;
use crate::generate::callback_gen::*;
use crate::generate::contract_gen::*;
use crate::generate::function_selector::generate_function_selector_body;

pub fn contract_implementation(
	contract: &ContractTrait,
	is_contract_main: bool,
) -> proc_macro2::TokenStream {
	let trait_name_ident = contract.trait_name.clone();
	let method_impls = extract_method_impls(&contract);

	if !contract.supertrait_paths.is_empty() {
		panic!("contract inheritance currently not supported");
	}

	let call_methods = generate_call_methods(&contract);
	let auto_impl_defs = generate_auto_impl_defs(&contract);
	let auto_impls = generate_auto_impls(&contract);
	let endpoints = generate_wasm_endpoints(&contract);
	let function_selector_body = generate_function_selector_body(&contract, is_contract_main);
	let abi_body = abi_gen::generate_abi_method_body(&contract);
	let callback_body = generate_callback_body(&contract.methods);
	let callback_proxies = generate_callback_proxies(&contract.methods);
	let api_where = snippets::api_where();

	let supertrait_impls = generate_supertrait_impls(&contract);
	let contract_trait_api_impl = snippets::contract_trait_api_impl(&trait_name_ident);

	// this definition is common to release and debug mode
	let main_definition = quote! {
		pub trait #trait_name_ident<T, BigInt, BigUint>:
		ContractSelfApi<BigInt, BigUint>
		// #( + #supertrait_paths <T, BigInt, BigUint>)* // currently not supported
		+ Sized
		#api_where
		{
			#(#method_impls)*

			#(#auto_impl_defs)*

			fn callback(&self);

			fn callbacks(&self) -> callback_proxy::CallbackProxies<T, BigInt, BigUint>;
		}
	};

	let implementation_mod = quote! {
		pub mod implementation {
			use super::*;
			use super::#trait_name_ident as ContractDef;

			pub struct #trait_name_ident<T, BigInt, BigUint>
			#api_where
			{
				api: T,
				_phantom1: core::marker::PhantomData<BigInt>,
				_phantom2: core::marker::PhantomData<BigUint>,
			}

			impl <T, BigInt, BigUint> elrond_wasm::api::ContractImpl<T> for #trait_name_ident<T, BigInt, BigUint>
			#api_where
			{
				fn new_contract_impl(api: T) -> Self {
					#trait_name_ident {
						api,
						_phantom1: core::marker::PhantomData,
						_phantom2: core::marker::PhantomData,
					}
				}
			}

			impl <T, BigInt, BigUint> elrond_wasm::api::CallableContract<T> for #trait_name_ident<T, BigInt, BigUint>
			#api_where
			{
				fn call(&self, fn_name: &[u8]) -> bool {
					#function_selector_body
				}

				fn clone_contract(&self) -> Box<dyn elrond_wasm::api::CallableContract<T>> {
					Box::new(elrond_wasm::api::new_contract_impl::<T, #trait_name_ident<T, BigInt, BigUint>>(self.api.clone()))
				}

				fn into_api(self: Box<Self>) -> T {
					self.api
				}
			}

			#contract_trait_api_impl

			#(#supertrait_impls)*

			impl <T, BigInt, BigUint> super::#trait_name_ident<T, BigInt, BigUint> for #trait_name_ident<T, BigInt, BigUint>
			#api_where
			{
				#(#auto_impls)*

				fn callback(&self) {
					#callback_body
				}

				fn callbacks(&self) -> super::callback_proxy::CallbackProxies<T, BigInt, BigUint> {
					super::callback_proxy::CallbackProxies::new(self.api.clone())
				}
			}

			impl <T, BigInt, BigUint> #trait_name_ident<T, BigInt, BigUint>
			#api_where
			{
				#(#call_methods)*
			}

			impl <T, BigInt, BigUint> elrond_wasm::api::ContractWithAbi for #trait_name_ident<T, BigInt, BigUint>
			#api_where
			{
				type Storage = T::Storage;

				fn abi(&self, include_modules: bool) -> elrond_wasm::abi::ContractAbi{
					#abi_body
				}
			}
		}

		mod callback_proxy {
			use super::*;
			#callback_proxies
		}
	};

	let helper = quote! {
		pub fn contract_obj<T, BigInt, BigUint>(api: T) -> implementation::#trait_name_ident<T, BigInt, BigUint>
		#api_where
		{
			elrond_wasm::api::new_contract_impl(api)
		}
	};

	let wasm_endpoints = quote! {
		#[cfg(feature = "wasm-output-mode")]
		#[allow(non_snake_case)]
		pub mod endpoints {
			use super::*;

			fn new_arwen_instance() -> super::implementation::#trait_name_ident<elrond_wasm_node::ArwenApiImpl, elrond_wasm_node::api::ArwenBigInt, elrond_wasm_node::api::ArwenBigUint> {
				let api = elrond_wasm_node::ArwenApiImpl{};
				elrond_wasm::api::new_contract_impl(api)
			}

			#(#endpoints)*
		}
	};

	quote! {
		#main_definition

		#implementation_mod

		#helper

		#wasm_endpoints
	}
}
