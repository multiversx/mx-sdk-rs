extern crate proc_macro;

mod nested_de_derive;
mod nested_en_derive;
mod top_de_derive;
mod top_en_derive;
mod util;

use nested_de_derive::impl_nested_decode_macro;
use nested_en_derive::impl_nested_encode_macro;
use top_de_derive::impl_top_decode_macro;
use top_en_derive::impl_top_encode_macro;

use proc_macro::TokenStream;
use syn;

#[proc_macro_derive(NestedEncode)]
pub fn nested_encode_derive(input: TokenStream) -> TokenStream {
	let ast = syn::parse(input).unwrap();

	impl_nested_encode_macro(&ast)
}

#[proc_macro_derive(TopEncode)]
pub fn top_encode_derive(input: TokenStream) -> TokenStream {
	let ast = syn::parse(input).unwrap();

	impl_top_encode_macro(&ast)
}

#[proc_macro_derive(NestedDecode)]
pub fn nested_decode_derive(input: TokenStream) -> TokenStream {
	let ast = syn::parse(input).unwrap();

	impl_nested_decode_macro(&ast)
}

#[proc_macro_derive(TopDecode)]
pub fn top_decode_derive(input: TokenStream) -> TokenStream {
	let ast = syn::parse(input).unwrap();

	impl_top_decode_macro(&ast)
}
