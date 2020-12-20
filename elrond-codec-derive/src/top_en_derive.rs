use proc_macro::TokenStream;
use quote::quote;
use syn;

use crate::util::*;

pub fn impl_top_encode_macro(ast: &syn::DeriveInput) -> TokenStream {
	let name = &ast.ident;
	let gen = match &ast.data {
		syn::Data::Struct(_) => {
			let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();

			quote! {
				impl #impl_generics elrond_codec::TopEncode for #name #ty_generics #where_clause {
					#[inline]
					fn top_encode<O: elrond_codec::TopEncodeOutput>(&self, output: O) -> Result<(), elrond_codec::EncodeError> {
						elrond_codec::top_encode_from_nested(self, output)
					}

					#[inline]
					fn top_encode_or_exit<O: elrond_codec::TopEncodeOutput, ExitCtx: Clone>(
						&self,
						output: O,
						c: ExitCtx,
						exit: fn(ExitCtx, elrond_codec::EncodeError) -> !,
					) {
						elrond_codec::top_encode_from_nested_or_exit(self, output, c, exit);
					}
				}
			}
		},
		syn::Data::Enum(_) => {
			if is_simple_enum(&ast.data) {
				let idents = extract_field_names(&ast.data);
				let value: Vec<u8> = (0..idents.len() as u8).collect();
				let name_repeated = std::iter::repeat(name);
				let name_repeated_again = name_repeated.clone();

				quote! {
					impl TopEncode for #name {
						fn top_encode<O: elrond_codec::TopEncodeOutput>(&self, output: O) -> Result<(), elrond_codec::EncodeError> {
							//self.to_u8().top_encode(output)
							match self {
								#(#name_repeated::#idents => #value.top_encode(output),)*
							}
						}

						fn top_encode_or_exit<O: elrond_codec::TopEncodeOutput, ExitCtx: Clone>(
							&self,
							output: O,
							c: ExitCtx,
							exit: fn(ExitCtx, elrond_codec::EncodeError) -> !,
						) {
							match self {
								#(#name_repeated_again::#idents => #value.top_encode_or_exit(output, c, exit),)*
							}
						}
					}
				}
			} else {
				panic!("Only simple enums can have top encode!")
			}
		},
		syn::Data::Union(_) => panic!("Union not supported"),
	};

	gen.into()
}
