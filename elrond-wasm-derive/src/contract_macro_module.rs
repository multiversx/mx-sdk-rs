use super::contract_gen::*;
use super::contract_impl::*;
use super::*;

pub fn process_module(
	args: proc_macro::TokenStream,
	input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	let args_input = parse_macro_input!(args as syn::AttributeArgs);
	let proc_input = &parse_macro_input!(input as syn::ItemTrait);

	let contract = Contract::new(args_input, proc_input);

	let contract_impl = contract_implementation(&contract, false);

	proc_macro::TokenStream::from(quote! {
		#contract_impl
	})
}
