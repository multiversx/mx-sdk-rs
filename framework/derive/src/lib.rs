#![allow(stable_features)]
// ensure we don't run out of macro stack
#![recursion_limit = "1024"]
#![feature(proc_macro_quote)]

#[macro_use]
extern crate syn;

#[macro_use]
extern crate quote;

mod contract_impl;
mod format;
mod generate;
mod macro_contract;
mod macro_module;
mod macro_proxy;
mod managed_vec_item_derive;
mod model;
mod parse;
mod preprocessing;
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

#[proc_macro_derive(ManagedVecItem)]
pub fn managed_vec_item_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    managed_vec_item_derive::managed_vec_item_derive(&ast)
}

#[proc_macro]
pub fn format_receiver_args(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    format::format_receiver_args_macro(input)
}
