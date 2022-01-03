use proc_macro::TokenStream;
use quote::quote;

use crate::util::*;

pub fn dep_decode_snippet(
    _index: usize,
    field: &syn::Field,
    input_value: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let ty = &field.ty;
    if let Some(ident) = &field.ident {
        quote! {
            #ident: <#ty as elrond_codec::NestedDecode>::dep_decode(#input_value)?
        }
    } else {
        quote! {
            <#ty as elrond_codec::NestedDecode>::dep_decode(#input_value)?
        }
    }
}

pub fn dep_decode_or_exit_snippet(
    _index: usize,
    field: &syn::Field,
    input_value: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let ty = &field.ty;
    if let Some(ident) = &field.ident {
        quote! {
            #ident: <#ty as elrond_codec::NestedDecode>::dep_decode_or_exit(#input_value, c.clone(), exit)
        }
    } else {
        quote! {
            <#ty as elrond_codec::NestedDecode>::dep_decode_or_exit(#input_value, c.clone(), exit)
        }
    }
}

pub fn variant_dep_decode_snippets(
    name: &syn::Ident,
    data_enum: &syn::DataEnum,
    input_value: &proc_macro2::TokenStream,
) -> Vec<proc_macro2::TokenStream> {
    data_enum
		.variants
		.iter()
		.enumerate()
		.map(|(variant_index, variant)| {
			let variant_index_u8 = variant_index as u8;
			let variant_ident = &variant.ident;
			let variant_field_snippets = fields_decl_syntax(&variant.fields, |index, field| {
				dep_decode_snippet(index, field, input_value)
			});
			quote! {
				#variant_index_u8 => core::result::Result::Ok( #name::#variant_ident #variant_field_snippets ),
			}
		})
		.collect()
}

pub fn variant_dep_decode_or_exit_snippets(
    name: &syn::Ident,
    data_enum: &syn::DataEnum,
    input_value: &proc_macro2::TokenStream,
) -> Vec<proc_macro2::TokenStream> {
    data_enum
        .variants
        .iter()
        .enumerate()
        .map(|(variant_index, variant)| {
            let variant_index_u8 = variant_index as u8;
            let variant_ident = &variant.ident;
            let variant_field_snippets = fields_decl_syntax(&variant.fields, |index, field| {
                dep_decode_or_exit_snippet(index, field, input_value)
            });
            quote! {
                #variant_index_u8 => #name::#variant_ident #variant_field_snippets ,
            }
        })
        .collect()
}

pub fn nested_decode_impl(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
    let gen = match &ast.data {
        syn::Data::Struct(data_struct) => {
            let field_dep_decode_snippets =
                fields_decl_syntax(&data_struct.fields, |index, field| {
                    dep_decode_snippet(index, field, &quote! {input})
                });
            let field_dep_encode_or_exit_snippets =
                fields_decl_syntax(&data_struct.fields, |index, field| {
                    dep_decode_or_exit_snippet(index, field, &quote! {input})
                });
            quote! {
                impl #impl_generics elrond_codec::NestedDecode for #name #ty_generics #where_clause {
                    fn dep_decode<I: elrond_codec::NestedDecodeInput>(input: &mut I) -> core::result::Result<Self, elrond_codec::DecodeError> {
                        core::result::Result::Ok(
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
            let variant_dep_decode_snippets =
                variant_dep_decode_snippets(name, data_enum, &quote! {input});
            let variant_dep_decode_or_exit_snippets =
                variant_dep_decode_or_exit_snippets(name, data_enum, &quote! {input});

            quote! {
                impl #impl_generics elrond_codec::NestedDecode for #name #ty_generics #where_clause {
                    fn dep_decode<I: elrond_codec::NestedDecodeInput>(input: &mut I) -> core::result::Result<Self, elrond_codec::DecodeError> {
                        match <u8 as elrond_codec::NestedDecode>::dep_decode(input)? {
                            #(#variant_dep_decode_snippets)*
                            _ => core::result::Result::Err(elrond_codec::DecodeError::INVALID_VALUE),
                        }
                    }

                    fn dep_decode_or_exit<I: elrond_codec::NestedDecodeInput, ExitCtx: Clone>(
                        input: &mut I,
                        c: ExitCtx,
                        exit: fn(ExitCtx, elrond_codec::DecodeError) -> !,
                    ) -> Self {
                        match <u8 as elrond_codec::NestedDecode>::dep_decode_or_exit(input, c.clone(), exit) {
                            #(#variant_dep_decode_or_exit_snippets)*
                            _ => exit(c, elrond_codec::DecodeError::INVALID_VALUE),
                        }
                    }
                }
            }
        },
        syn::Data::Union(_) => panic!("Union not supported"),
    };

    gen.into()
}
