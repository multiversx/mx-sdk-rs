use crate::util::*;
use proc_macro::TokenStream;
use quote::quote;
use syn;

pub fn impl_nested_encode_macro(ast: &syn::DeriveInput) -> TokenStream {
	let name = &ast.ident;
	let idents = extract_field_names(&ast.data);
	let gen = match &ast.data {
		syn::Data::Struct(_) => {
			let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();

			if idents.len() > 0 {
				quote! {
					impl #impl_generics NestedEncode for #name #ty_generics #where_clause {
						fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
							#(self.#idents.dep_encode(dest)?;)*

							Ok(())
						}

						fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
							&self,
							dest: &mut O,
							c: ExitCtx,
							exit: fn(ExitCtx, EncodeError) -> !,
						) {
							#(self.#idents.dep_encode_or_exit(dest, c.clone(), exit);)*
						}
					}
				}
			} else {
				let total_fields = match &ast.data {
					syn::Data::Struct(s) => match &s.fields {
						syn::Fields::Unnamed(u) => u.unnamed.len(),
						_ => panic!("only structs with unnamed fields should reach here!"),
					},
					_ => panic!("only structs should reach here!"),
				};
				let nameless_field_ident = (0..total_fields).map(syn::Index::from);
				let nameless_field_ident_again = nameless_field_ident.clone();

				quote! {
					impl #impl_generics NestedEncode for #name #ty_generics #where_clause {
						fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
							#(self.#nameless_field_ident.dep_encode(dest)?;)*

							Ok(())
						}

						fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
							&self,
							dest: &mut O,
							c: ExitCtx,
							exit: fn(ExitCtx, EncodeError) -> !,
						) {
							#(self.#nameless_field_ident_again.dep_encode_or_exit(dest, c.clone(), exit);)*
						}
					}
				}
			}
		},
		syn::Data::Enum(_) => {
			let types = extract_enum_field_types(&ast.data);
			let value: Vec<u8> = (0..idents.len() as u8).collect();
			let mut enum_encode_snippets = Vec::new();
			let mut enum_encode_or_exit_snippets = Vec::new();

			for i in 0..types.len() {
				let type_list = &types[i];
				let ident = &idents[i];
				let val = &value[i];

				if type_list.is_empty() {
					enum_encode_snippets.push(quote! {
						#name::#ident => #val.dep_encode(dest)?,
					});

					enum_encode_or_exit_snippets.push(quote! {
						#name::#ident => #val.dep_encode_or_exit(dest, c.clone(), exit),
					});
				} else if type_list.len() == 1 {
					let local_var_ident =
						syn::Ident::new("_var_enum_local_ident", proc_macro2::Span::call_site());

					enum_encode_snippets.push(quote! {
						#name::#ident(#local_var_ident) => {
							#val.dep_encode(dest)?;
							#local_var_ident.dep_encode(dest)?;
						},
					});

					enum_encode_or_exit_snippets.push(quote! {
						#name::#ident(#local_var_ident) => {
							#val.dep_encode_or_exit(dest, c.clone(), exit);
							#local_var_ident.dep_encode_or_exit(dest, c.clone(), exit);
						},
					});
				} else {
					panic!("Only enums with one or less fields supported at the moment!");
				}
			}

			quote! {
				impl NestedEncode for #name {
					fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
						match self {
							#(#enum_encode_snippets)*
						};
						Ok(())
					}

					fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
						&self,
						dest: &mut O,
						c: ExitCtx,
						exit: fn(ExitCtx, EncodeError) -> !,
					) {
						match self {
							#(#enum_encode_or_exit_snippets)*
						};
					}
				}
			}
		},
		syn::Data::Union(_) => panic!("Union not supported!"),
	};

	gen.into()
}
