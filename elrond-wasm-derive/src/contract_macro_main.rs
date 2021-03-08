use super::contract_impl::contract_implementation;
use crate::parse::parse_contract_trait;
use crate::validate::validate_contract;

pub fn process_contract(
	args: proc_macro::TokenStream,
	input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	let args_input = parse_macro_input!(args as syn::AttributeArgs);
	let proc_input = &parse_macro_input!(input as syn::ItemTrait);

	let contract = parse_contract_trait(args_input, proc_input);
	validate_contract(&contract);

	let contract_impl = contract_implementation(&contract, true);
	let contract_impl_ident = contract.contract_impl_name;

	let wasm_callback_endpoint = quote! {
		#[cfg(feature = "wasm-output-mode")]
		pub mod callback_endpoint {
			use super::*;

			fn new_arwen_instance() -> #contract_impl_ident<elrond_wasm_node::ArwenApiImpl, elrond_wasm_node::api::ArwenBigInt, elrond_wasm_node::api::ArwenBigUint> {
				let api = elrond_wasm_node::ArwenApiImpl{};
				#contract_impl_ident::new(api)
			}

			#[no_mangle]
			#[allow(non_snake_case)]
			pub fn callBack () {
				let inst = new_arwen_instance();
				inst.callback();
			}
		}
	};

	proc_macro::TokenStream::from(quote! {
		#[macro_use]
		extern crate elrond_wasm;

		#contract_impl

		#wasm_callback_endpoint
	})
}
