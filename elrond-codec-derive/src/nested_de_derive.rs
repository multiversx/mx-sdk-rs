use proc_macro::TokenStream;
use quote::quote;
use syn;

use crate::util::*;

pub fn dep_decode_snippet(_index: usize, field: &syn::Field) -> proc_macro2::TokenStream {
	let ty = &field.ty;
	if let Some(ident) = &field.ident {
		quote! {
			#ident: <#ty>::dep_decode(input)?
		}
	} else {
		quote! {
			<#ty>::dep_decode(input)?
		}
	}
}

pub fn dep_decode_or_exit_snippet(_index: usize, field: &syn::Field) -> proc_macro2::TokenStream {
	let ty = &field.ty;
	if let Some(ident) = &field.ident {
		quote! {
			#ident: <#ty>::dep_decode_or_exit(input, c.clone(), exit)
		}
	} else {
		quote! {
			<#ty>::dep_decode_or_exit(input, c.clone(), exit)
		}
	}
}

pub fn impl_nested_decode_macro(ast: &syn::DeriveInput) -> TokenStream {
	let name = &ast.ident;
	let gen = match &ast.data {
		syn::Data::Struct(data_struct) => {
			let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
			let field_dep_decode_snippets =
				fields_decl_syntax(&data_struct.fields, |index, field| {
					dep_decode_snippet(index, field)
				});
			let field_dep_encode_or_exit_snippets =
				fields_decl_syntax(&data_struct.fields, |index, field| {
					dep_decode_or_exit_snippet(index, field)
				});
			quote! {
				impl #impl_generics elrond_codec::NestedDecode for #name #ty_generics #where_clause {
					fn dep_decode<I: elrond_codec::NestedDecodeInput>(input: &mut I) -> Result<Self, elrond_codec::DecodeError> {
						Result::Ok(
							#name #field_dep_decode_snippets
						)
					}

					fn dep_decode_or_exit<I: elrond_codec::NestedDecodeInput, ExitCtx: Clone>(
						input: &mut I,
						c: ExitCtx,
						exit: fn(ExitCtx, elrond_codec::DecodeError) -> !,
					) -> Self {
						#name #field_dep_encode_or_exit_snippets
					}
				}
			}
		},
		syn::Data::Enum(data_enum) => {
			assert!(
				data_enum.variants.len() < 256,
				"enums with more than 256 variants not supported"
			);
			let variant_dep_decode_snippets: Vec<proc_macro2::TokenStream> = data_enum
				.variants
				.iter()
				.enumerate()
				.map(|(variant_index, variant)| {
					let variant_index_u8 = variant_index as u8;
					let variant_ident = &variant.ident;
					let variant_field_snippets =
						fields_decl_syntax(&variant.fields, |index, field| {
							dep_decode_snippet(index, field)
						});
					quote! {
						#variant_index_u8 => Result::Ok( #name::#variant_ident #variant_field_snippets ),
					}
				})
				.collect();

			let variant_dep_decode_or_exit_snippets: Vec<proc_macro2::TokenStream> = data_enum
				.variants
				.iter()
				.enumerate()
				.map(|(variant_index, variant)| {
					let variant_index_u8 = variant_index as u8;
					let variant_ident = &variant.ident;
					let variant_field_snippets =
						fields_decl_syntax(&variant.fields, |index, field| {
							dep_decode_or_exit_snippet(index, field)
						});
					quote! {
						#variant_index_u8 => #name::#variant_ident #variant_field_snippets ,
					}
				})
				.collect();

			quote! {
				impl elrond_codec::NestedDecode for #name {
					fn dep_decode<I: elrond_codec::NestedDecodeInput>(input: &mut I) -> Result<Self, elrond_codec::DecodeError> {
						match u8::dep_decode(input)? {
							#(#variant_dep_decode_snippets)*
							_ => Result::Err(elrond_codec::DecodeError::INVALID_VALUE),
						}
					}

					fn dep_decode_or_exit<I: elrond_codec::NestedDecodeInput, ExitCtx: Clone>(
						input: &mut I,
						c: ExitCtx,
						exit: fn(ExitCtx, elrond_codec::DecodeError) -> !,
					) -> Self {
						match u8::dep_decode_or_exit(input, c.clone(), exit) {
							#(#variant_dep_decode_or_exit_snippets)*
							_ => exit(c, elrond_codec::DecodeError::INVALID_VALUE),
						}
					}
				}
			}
		},
		syn::Data::Union(_) => panic!("Union not supported!"),
	};

	gen.into()
}
