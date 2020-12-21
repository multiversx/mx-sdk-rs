use proc_macro::TokenStream;
use quote::quote;
use syn;

use crate::nested_de_derive::*;
use crate::util::*;

fn fieldless_enum_match_arm_result_ok(
	name: &syn::Ident,
	data_enum: &syn::DataEnum,
) -> Vec<proc_macro2::TokenStream> {
	data_enum
		.variants
		.iter()
		.enumerate()
		.map(|(variant_index, variant)| {
			let variant_index_u8 = variant_index as u8;
			let variant_ident = &variant.ident;
			quote! {
				#variant_index_u8 => Result::Ok( #name::#variant_ident ),
			}
		})
		.collect()
}

fn fieldless_enum_match_arm(
	name: &syn::Ident,
	data_enum: &syn::DataEnum,
) -> Vec<proc_macro2::TokenStream> {
	data_enum
		.variants
		.iter()
		.enumerate()
		.map(|(variant_index, variant)| {
			let variant_index_u8 = variant_index as u8;
			let variant_ident = &variant.ident;
			quote! {
				#variant_index_u8 => #name::#variant_ident ,
			}
		})
		.collect()
}

fn zero_value_if_result_ok(
	name: &syn::Ident,
	data_enum: &syn::DataEnum,
) -> proc_macro2::TokenStream {
	if data_enum.variants.is_empty() {
		panic!("cannot deserialize enums without variants");
	}
	let first_variant = &data_enum.variants[0];
	if first_variant.fields.is_empty() {
		quote! {
			if top_input.byte_len() == 0 {
				return Result::Ok(#name::#first_variant);
			}
		}
	} else {
		quote! {}
	}
}

fn zero_value_if(name: &syn::Ident, data_enum: &syn::DataEnum) -> proc_macro2::TokenStream {
	if data_enum.variants.is_empty() {
		panic!("cannot deserialize enums without variants");
	}
	let first_variant = &data_enum.variants[0];
	if first_variant.fields.is_empty() {
		quote! {
			if top_input.byte_len() == 0 {
				return #name::#first_variant;
			}
		}
	} else {
		quote! {}
	}
}

pub fn impl_top_decode_macro(ast: &syn::DeriveInput) -> TokenStream {
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
				impl #impl_generics elrond_codec::TopDecode for #name #ty_generics #where_clause {
					fn top_decode<I: elrond_codec::TopDecodeInput>(top_input: I) -> Result<Self, elrond_codec::DecodeError> {
						let bytes = top_input.into_boxed_slice_u8();
						let input = &mut &*bytes;
						let result = #name #field_dep_decode_snippets ;
						if !input.is_empty() {
							return Result::Err(elrond_codec::DecodeError::INPUT_TOO_LONG);
						}
						Ok(result)
					}

					fn top_decode_or_exit<I: elrond_codec::TopDecodeInput, ExitCtx: Clone>(
						top_input: I,
						c: ExitCtx,
						exit: fn(ExitCtx, elrond_codec::DecodeError) -> !,
					) -> Self {
						let bytes = top_input.into_boxed_slice_u8();
						let input = &mut &*bytes;
						let result = #name #field_dep_encode_or_exit_snippets ;
						if !input.is_empty() {
							exit(c, elrond_codec::DecodeError::INPUT_TOO_LONG);
						}
						result
					}
				}
			}
		},
		syn::Data::Enum(data_enum) => {
			assert!(
				data_enum.variants.len() < 256,
				"enums with more than 256 variants not supported"
			);
			if is_fieldless_enum(&data_enum) {
				// fieldless enums are special, they can be top-decoded as u8 directly
				let top_decode_arms = fieldless_enum_match_arm_result_ok(&name, &data_enum);
				let top_decode_or_exit_arms = fieldless_enum_match_arm(&name, &data_enum);
				quote! {
					impl elrond_codec::TopDecode for #name {
						fn top_decode<I: elrond_codec::TopDecodeInput>(input: I) -> Result<Self, elrond_codec::DecodeError> {
							match u8::top_decode(input)? {
								#(#top_decode_arms)*
								_ => Result::Err(elrond_codec::DecodeError::INVALID_VALUE),
							}
						}

						fn top_decode_or_exit<I: elrond_codec::TopDecodeInput, ExitCtx: Clone>(
							input: I,
							c: ExitCtx,
							exit: fn(ExitCtx, elrond_codec::DecodeError) -> !,
						) -> Self {
							match u8::top_decode_or_exit(input, c.clone(), exit) {
								#(#top_decode_or_exit_arms)*
								_ => exit(c, elrond_codec::DecodeError::INVALID_VALUE),
							}
						}
					}
				}
			} else {
				let variant_dep_decode_snippets = variant_dep_decode_snippets(&name, &data_enum);
				let variant_dep_decode_or_exit_snippets =
					variant_dep_decode_or_exit_snippets(&name, &data_enum);
				let zero_value_if_result_ok = zero_value_if_result_ok(&name, &data_enum);
				let zero_value_if = zero_value_if(&name, &data_enum);

				quote! {
					impl elrond_codec::TopDecode for #name {
						fn top_decode<I: elrond_codec::TopDecodeInput>(top_input: I) -> Result<Self, elrond_codec::DecodeError> {
							#zero_value_if_result_ok
							let bytes = top_input.into_boxed_slice_u8();
							let input = &mut &*bytes;
							let result = match u8::dep_decode(input)? {
								#(#variant_dep_decode_snippets)*
								_ => Result::Err(elrond_codec::DecodeError::INVALID_VALUE),
							};
							if !input.is_empty() {
								return Result::Err(elrond_codec::DecodeError::INPUT_TOO_LONG);
							}
							result
						}

						fn top_decode_or_exit<I: elrond_codec::TopDecodeInput, ExitCtx: Clone>(
							top_input: I,
							c: ExitCtx,
							exit: fn(ExitCtx, elrond_codec::DecodeError) -> !,
						) -> Self {
							#zero_value_if
							let bytes = top_input.into_boxed_slice_u8();
							let input = &mut &*bytes;
							let result = match u8::dep_decode_or_exit(input, c.clone(), exit) {
								#(#variant_dep_decode_or_exit_snippets)*
								_ => exit(c, elrond_codec::DecodeError::INVALID_VALUE),
							};
							if !input.is_empty() {
								exit(c, elrond_codec::DecodeError::INPUT_TOO_LONG);
							}
							result
						}
					}
				}
			}
		},
		syn::Data::Union(_) => panic!("Union not supported"),
	};

	gen.into()
}
