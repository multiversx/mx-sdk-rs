use super::generate::snippets;
use crate::generate::contract_gen::*;
use crate::generate::proxy_gen;
use crate::generate::supertrait_gen;
use crate::model::ContractTrait;
use crate::parse::parse_contract_trait;
use crate::validate::validate_contract;

pub fn process_proxy(
	args: proc_macro::TokenStream,
	input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	let args_input = parse_macro_input!(args as syn::AttributeArgs);
	let proc_input = parse_macro_input!(input as syn::ItemTrait);

	let contract = parse_contract_trait(args_input, &proc_input);
	validate_contract(&contract);

	let proxy_impl = proxy_implementation(&contract, true);

	proc_macro::TokenStream::from(quote! {
	  #proxy_impl
	})
}

pub fn proxy_implementation(
	contract: &ContractTrait,
	is_contract_main: bool,
) -> proc_macro2::TokenStream {
	let trait_name_ident = contract.trait_name.clone();
	let method_impls = extract_method_impls(&contract);
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
		}
	};

	let proxy_trait = proxy_gen::proxy_trait(&contract);
	let proxy_obj_code = if is_contract_main {
		proxy_gen::proxy_obj_code(&contract)
	} else {
		quote! {}
	};

	quote! {
		#main_definition

		#proxy_trait

		#proxy_obj_code
	}
}
