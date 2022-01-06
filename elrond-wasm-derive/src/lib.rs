#![allow(stable_features)]
// ensure we don't run out of macro stack
#![recursion_limit = "1024"]
#![feature(extend_one)]
use formatted_error_message::{count_args, split_msg_into_format_parts, FormatPartType};

use crate::generate::util::byte_str_literal;

#[macro_use]
extern crate syn;

#[macro_use]
extern crate quote;

mod contract_impl;
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

mod formatted_error_message;

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
pub fn sc_error_format(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut token_stream = proc_macro::TokenStream::new();
    let mut arg_index = 0usize;

    // println!("{:#?}", input);

    let mut iter = input.into_iter();
    let format_string = if let proc_macro::TokenTree::Literal(lit) = iter.next().unwrap() {
        lit.to_string()
    } else {
        panic!("First argument should be the format string");
    };

    let mut arg_groups = Vec::new();
    for tt in iter {
        match tt {
            proc_macro::TokenTree::Group(g) => arg_groups.push(g),
            _ => {},
        }
    }

    let parts = split_msg_into_format_parts(&format_string);
    let nr_args_provided = count_args(&parts);
    if nr_args_provided != arg_groups.len() {
        panic!(
            "Number of braces ({}) does not match number of arguments ({}).",
            nr_args_provided,
            arg_groups.len()
        )
    }

    for part in parts {
        match part {
            FormatPartType::StaticAscii(ascii_string) => {
                let str_as_bytes = byte_str_literal(ascii_string.as_bytes());
                let add_static_part = quote! {
                    ___buffer___.append_bytes(#str_as_bytes);
                };
                token_stream.extend_one(proc_macro::TokenStream::from(add_static_part));
            },
            FormatPartType::Ascii => {
                let encode_arg = quote! {
                    encoded_arg_by_index = ___encoded_args___.get(#arg_index).unwrap();
                    ___buffer___.append_managed_buffer(&encoded_arg_by_index);
                };
                token_stream.extend_one(proc_macro::TokenStream::from(encode_arg));

                arg_index += 1;
            },
            FormatPartType::Hex => {
                let encode_arg = quote! {
                    encoded_arg_by_index = ___encoded_args___.get(#arg_index).unwrap();
                    elrond_wasm::hex_util::add_arg_as_hex_to_buffer(&mut ___buffer___, &encoded_arg_by_index);
                };
                token_stream.extend_one(proc_macro::TokenStream::from(encode_arg));

                arg_index += 1;
            },
        }
    }

    token_stream
}
