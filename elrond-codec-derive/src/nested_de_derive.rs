use proc_macro::TokenStream;
use quote::quote;
use syn;

use crate::util::*;

pub fn impl_nested_decode_macro(ast: &syn::DeriveInput) -> TokenStream {
	if let syn::Data::Union(_) = &ast.data {
		panic!("Union not supported!");
	}

	let name = &ast.ident;
	let idents = extract_field_names(&ast.data);
	let gen = match &ast.data {
		syn::Data::Struct(_) => {
			let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
			let types = extract_struct_field_types(&ast.data);

			if idents.len() > 0 {
				quote! {
					impl #impl_generics NestedDecode for #name #ty_generics #where_clause {
						fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
							Ok(#name {
								#(#idents: <#types>::dep_decode(input)?,)*
							})
						}

						fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
							input: &mut I,
							c: ExitCtx,
							exit: fn(ExitCtx, DecodeError) -> !,
						) -> Self {
							#name {
								#(#idents: <#types>::dep_decode_or_exit(input, c.clone(), exit),)*
							}
						}
					}
				}
			} else {
				quote! {
					impl #impl_generics NestedDecode for #name #ty_generics #where_clause {
						fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
							Ok(#name (
								#(<#types>::dep_decode(input)?),*
							))
						}

						fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
							input: &mut I,
							c: ExitCtx,
							exit: fn(ExitCtx, DecodeError) -> !,
						) -> Self {
							#name (
								#(<#types>::dep_decode_or_exit(input, c.clone(), exit)),*
							)
						}
					}
				}
			}
		},
		syn::Data::Enum(_) => {
			let types = extract_enum_field_types(&ast.data);
			let value: Vec<u8> = (0..idents.len() as u8).collect();
			let mut enum_decode_snippets = Vec::new();
			let mut enum_decode_or_exit_snippets = Vec::new();

			for i in 0..types.len() {
				let type_list = &types[i];
				let ident = &idents[i];
				let val = &value[i];

				if type_list.is_empty() {
					enum_decode_snippets.push(quote! {
						#val => Some(#name::#ident),
					});

					enum_decode_or_exit_snippets.push(quote! {
						#val => Some(#name::#ident),
					});
				} else if type_list.len() == 1 {
					let var_type = &type_list[0];

					enum_decode_snippets.push(quote! {
						#val => Some(#name::#ident(<#var_type>::dep_decode(input)?)),
					});

					enum_decode_or_exit_snippets.push(quote! {
						#val => Some(#name::#ident(<#var_type>::dep_decode_or_exit(input, c.clone(), exit))),
					});
				} else {
					panic!("Only enums with one or less fields supported at the moment!");
				}
			}

			quote! {
				impl NestedDecode for #name {
					fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
						let return_value = match u8::dep_decode(input)? {
							#(#enum_decode_snippets)*
							_ => None
						};

						match return_value {
							Some(r) => Ok(r),
							None => Err(DecodeError::INVALID_VALUE)
						}
					}

					fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
						input: &mut I,
						c: ExitCtx,
						exit: fn(ExitCtx, DecodeError) -> !,
					) -> Self {
						let return_value = match u8::dep_decode_or_exit(input, c.clone(), exit) {
							#(#enum_decode_or_exit_snippets)*
							_ => None
						};

						match return_value {
							Some(r) => r,
							None => exit(c, DecodeError::INVALID_VALUE)
						}
					}
				}
			}
		},
		syn::Data::Union(_) => panic!("Union not supported!"),
	};

	gen.into()
}
