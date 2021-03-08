#![allow(dead_code)]
#![allow(stable_features)]
// ensure we don't run out of macro stack
#![recursion_limit = "1024"]

#[macro_use]
extern crate syn;

#[macro_use]
extern crate quote;

mod callable;
mod contract_impl;
mod contract_macro_main;
mod contract_macro_module;
mod generate;
mod model;
mod parse;
mod type_abi_derive;
mod validate;

#[proc_macro_attribute]
pub fn contract(
	args: proc_macro::TokenStream,
	input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	contract_macro_main::process_contract(args, input)
}

#[proc_macro_attribute]
pub fn module(
	args: proc_macro::TokenStream,
	input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	contract_macro_module::process_module(args, input)
}

#[proc_macro_attribute]
pub fn callable(
	args: proc_macro::TokenStream,
	input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	callable::process_callable(args, input)
}

#[proc_macro_derive(TypeAbi)]
pub fn type_abi_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let ast = syn::parse(input).unwrap();

	type_abi_derive::type_abi_derive(&ast)
}
