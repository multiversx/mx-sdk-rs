extern crate proc_macro;

mod nested_de_derive;
mod nested_en_derive;
mod top_de_derive;
mod top_en_derive;
mod util;

use nested_de_derive::nested_decode_impl;
use nested_en_derive::nested_encode_impl;
use top_de_derive::{top_decode_impl, top_decode_or_default_impl};
use top_en_derive::{top_encode_impl, top_encode_or_default_impl};

use proc_macro::TokenStream;

#[proc_macro_derive(NestedEncode)]
pub fn nested_encode_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    nested_encode_impl(&ast)
}

#[proc_macro_derive(TopEncode)]
pub fn top_encode_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    top_encode_impl(&ast)
}

#[proc_macro_derive(TopEncodeOrDefault)]
pub fn top_encode_or_default_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    top_encode_or_default_impl(&ast)
}

#[proc_macro_derive(NestedDecode)]
pub fn nested_decode_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    nested_decode_impl(&ast)
}

#[proc_macro_derive(TopDecode)]
pub fn top_decode_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    top_decode_impl(&ast)
}

#[proc_macro_derive(TopDecodeOrDefault)]
pub fn top_decode_or_default_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    top_decode_or_default_impl(&ast)
}
