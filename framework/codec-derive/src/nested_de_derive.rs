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
            #ident: <#ty as codec::NestedDecode>::dep_decode_or_handle_err(#input_value, h)?
        }
    } else {
        quote! {
            <#ty as codec::NestedDecode>::dep_decode_or_handle_err(#input_value, h)?
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

pub fn nested_decode_impl(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
    let gen = match &ast.data {
        syn::Data::Struct(data_struct) => {
            let field_dep_decode_snippets =
                fields_decl_syntax(&data_struct.fields, |index, field| {
                    dep_decode_snippet(index, field, &quote! {input})
                });
            quote! {
                impl #impl_generics codec::NestedDecode for #name #ty_generics #where_clause {
                    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> core::result::Result<Self, H::HandledErr>
                    where
                        I: codec::NestedDecodeInput,
                        H: codec::DecodeErrorHandler,
                    {
                        core::result::Result::Ok(
                            #name #field_dep_decode_snippets
                        )
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

            quote! {
                impl #impl_generics codec::NestedDecode for #name #ty_generics #where_clause {
                    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> core::result::Result<Self, H::HandledErr>
                    where
                        I: codec::NestedDecodeInput,
                        H: codec::DecodeErrorHandler,
                    {
                        match <u8 as codec::NestedDecode>::dep_decode_or_handle_err(input, h)? {
                            #(#variant_dep_decode_snippets)*
                            _ => core::result::Result::Err(h.handle_error(codec::DecodeError::INVALID_VALUE)),
                        }
                    }
                }
            }
        },
        syn::Data::Union(_) => panic!("Union not supported"),
    };

    gen.into()
}
