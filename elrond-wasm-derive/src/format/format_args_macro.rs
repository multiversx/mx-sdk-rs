use proc_macro::quote;

use crate::{format::format_tokenize, generate::util::byte_str_literal};

use super::{count_args, parse_format_string, FormatPartType};

pub fn format_receiver_args_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let tokens = format_tokenize::tokenize(input);
    assert!(
        tokens.len() > 2,
        "format_receiver_args macro requires at least 2 arguments"
    );
    let num_arguments = tokens.len() - 2;
    let mut tokens_iter = tokens.into_iter();

    let accumulator_expr = tokens_iter.next().unwrap();
    let format_string = if let proc_macro::TokenTree::Literal(lit) = tokens_iter.next().unwrap() {
        lit.to_string()
    } else {
        panic!("First argument should be the format string");
    };

    let format_str_parts = parse_format_string(&format_string);
    let num_placeholders = count_args(&format_str_parts);
    assert!(
        num_placeholders == num_arguments,
        "Number of placeholders ({}) does not match number of arguments ({}).",
        num_placeholders,
        num_arguments
    );

    format_str_parts.into_iter().map(|part| {
        match part {
            FormatPartType::StaticAscii(ascii_string) => {
                let str_as_bytes = byte_str_literal(ascii_string.as_bytes());
                quote! (
                    elrond_wasm::formatter::FormatReceiver::push_static_ascii(&mut $accumulator_expr, $str_as_bytes);
                )
            },
            FormatPartType::Ascii => {
                let arg_expr = tokens_iter.next().unwrap();
                quote! (
                    elrond_wasm::formatter::FormatReceiver::push_top_encode_bytes(&mut $accumulator_expr, &$arg_expr);
                )
            },
            FormatPartType::Hex => {
                let arg_expr = tokens_iter.next().unwrap();
                quote! (
                    elrond_wasm::formatter::FormatReceiver::push_top_encode_hex(&mut $accumulator_expr, &$arg_expr);
                )
            },
        }
    }).collect()
}
