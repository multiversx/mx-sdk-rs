#![allow(dead_code)]
#![allow(stable_features)]
// ensure we don't run out of macro stack
#![recursion_limit = "1024"]

#[macro_use]
extern crate syn;

#[macro_use]
extern crate quote;

mod contract_impl;
mod generate;
mod macro_contract;
mod macro_module;
mod macro_proxy;
mod model;
mod parse;
mod type_abi_derive;
mod validate;

#[proc_macro_attribute]
pub fn contract(
	args: proc_macro::TokenStream,
	input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	macro_contract::process_contract(args, input)
}

#[proc_macro_attribute]
pub fn module(
	args: proc_macro::TokenStream,
	input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	macro_module::process_module(args, input)
}

#[proc_macro_attribute]
pub fn proxy(
	args: proc_macro::TokenStream,
	input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	macro_proxy::process_proxy(args, input)
}

#[proc_macro_derive(TypeAbi)]
pub fn type_abi_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let ast = syn::parse(input).unwrap();

	type_abi_derive::type_abi_derive(&ast)
}
