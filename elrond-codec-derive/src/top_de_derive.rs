use proc_macro::TokenStream;
use quote::quote;
use syn;

use crate::util::*;

pub fn impl_top_decode_macro(ast: &syn::DeriveInput) -> TokenStream {
	let name = &ast.ident;
	let gen = match &ast.data {
		syn::Data::Struct(_) => {
			let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();

			quote! {
				impl #impl_generics elrond_codec::TopDecode for #name #ty_generics #where_clause {
					fn top_decode<I: elrond_codec::TopDecodeInput>(input: I) -> Result<Self, elrond_codec::DecodeError> {
						elrond_codec::top_decode_from_nested(input)
					}

					fn top_decode_or_exit<I: elrond_codec::TopDecodeInput, ExitCtx: Clone>(
						input: I,
						c: ExitCtx,
						exit: fn(ExitCtx, elrond_codec::DecodeError) -> !,
					) -> Self {
						elrond_codec::top_decode_from_nested_or_exit(input, c, exit)
					}
				}
			}
		},
		syn::Data::Enum(_) => {
			if is_simple_enum(&ast.data) {
				let idents = extract_field_names(&ast.data);
				let value = 0..idents.len() as u8;
				let value_again = value.clone();
				let name_repeated = std::iter::repeat(name);
				let name_repeated_again = name_repeated.clone();

				quote! {
					impl TopDecode for #name {
						fn top_decode<I: elrond_codec::TopDecodeInput>(input: I) -> Result<Self, elrond_codec::DecodeError> {
							match u8::top_decode(input)? {
								#(#value => core::result::Result::Ok(#name_repeated::#idents),)*
								_ => core::result::Result::Err(elrond_codec::DecodeError::INVALID_VALUE),
							}
						}

						fn top_decode_or_exit<I: elrond_codec::TopDecodeInput, ExitCtx: Clone>(
							input: I,
							c: ExitCtx,
							exit: fn(ExitCtx, elrond_codec::DecodeError) -> !,
						) -> Self {
							match u8::top_decode_or_exit(input, c.clone(), exit) {
								#(#value_again => #name_repeated_again::#idents,)*
								_ => exit(c, elrond_codec::DecodeError::INVALID_VALUE),
							}
						}
					}
				}
			} else {
				panic!("Only simple enums can have top decode!")
			}
		},
		syn::Data::Union(_) => panic!("Union not supported"),
	};

	gen.into()
}
