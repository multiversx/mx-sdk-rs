use proc_macro::TokenStream;
use quote::quote;

use crate::{nested_de_derive::*, util::*};

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
                #variant_index_u8 => core::result::Result::Ok( #name::#variant_ident ),
            }
        })
        .collect()
}

/// Generates a default-value deserializer snippet automatically.
/// Currently only does so for enums whose first variant is fieldless.
/// Not called for TopDecodeOrDefault, since that one already provides an explicit default.
fn auto_default(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    if let syn::Data::Enum(data_enum) = &ast.data {
        assert!(
            !data_enum.variants.is_empty(),
            "cannot deserialize enums without variants"
        );
        let first_variant = &data_enum.variants[0];
        if first_variant.fields.is_empty() {
            let first_variant_ident = &first_variant.ident;
            let auto_default = quote! {
                if top_input.byte_len() == 0 {
                    return core::result::Result::Ok(#name::#first_variant_ident);
                }
            };
            return auto_default;
        }
    }

    // returns nothing by default
    quote! {}
}

/// Only returns the trait implementation method bodies, without the impl or method definitions.
fn top_decode_method_body(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    match &ast.data {
        syn::Data::Struct(data_struct) => {
            let field_dep_decode_snippets =
                fields_decl_syntax(&data_struct.fields, |index, field| {
                    dep_decode_snippet(index, field, &quote! {&mut nested_buffer})
                });

            quote! {
                let mut nested_buffer = top_input.into_nested_buffer();
                let result = #name #field_dep_decode_snippets ;
                if !codec::NestedDecodeInput::is_depleted(&nested_buffer) {
                    return core::result::Result::Err(h.handle_error(codec::DecodeError::INPUT_TOO_LONG));
                }
                core::result::Result::Ok(result)
            }
        },
        syn::Data::Enum(data_enum) => {
            assert!(
                data_enum.variants.len() < 256,
                "enums with more than 256 variants not supported"
            );
            if is_fieldless_enum(data_enum) {
                // fieldless enums are special, they can be top-decoded as u8 directly
                let top_decode_arms = fieldless_enum_match_arm_result_ok(name, data_enum);
                quote! {
                    match <u8 as codec::TopDecode>::top_decode_or_handle_err(top_input, h)? {
                        #(#top_decode_arms)*
                        _ => core::result::Result::Err(h.handle_error(codec::DecodeError::INVALID_VALUE)),
                    }
                }
            } else {
                let variant_dep_decode_snippets =
                    variant_dep_decode_snippets(name, data_enum, &quote! {&mut nested_buffer});

                quote! {
                    let mut nested_buffer = top_input.into_nested_buffer();
                    let result = match <u8 as codec::NestedDecode>::dep_decode_or_handle_err(&mut nested_buffer, h)? {
                        #(#variant_dep_decode_snippets)*
                        _ => core::result::Result::Err(
                            h.handle_error(codec::DecodeError::INVALID_VALUE),
                        ),
                    };
                    if !codec::NestedDecodeInput::is_depleted(&nested_buffer) {
                        return core::result::Result::Err(
                            h.handle_error(codec::DecodeError::INPUT_TOO_LONG),
                        );
                    }
                    result
                }
            }
        },
        syn::Data::Union(_) => panic!("Union not supported"),
    }
}

pub fn top_decode_impl(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
    let top_decode_body = top_decode_method_body(ast);
    let auto_default = auto_default(ast);

    let gen = quote! {
        impl #impl_generics codec::TopDecode for #name #ty_generics #where_clause {
            fn top_decode_or_handle_err<I, H>(top_input: I, h: H) -> core::result::Result<Self, H::HandledErr>
            where
                I: codec::TopDecodeInput,
                H: codec::DecodeErrorHandler,
            {
                #auto_default
                #top_decode_body
            }
        }
    };

    gen.into()
}

pub fn top_decode_or_default_impl(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
    let top_decode_body = top_decode_method_body(ast);

    let gen = quote! {
        impl #impl_generics codec::TopDecode for #name #ty_generics #where_clause {
            fn top_decode_or_handle_err<I, H>(top_input: I, h: H) -> core::result::Result<Self, H::HandledErr>
            where
                I: codec::TopDecodeInput,
                H: codec::DecodeErrorHandler,
            {
                if top_input.byte_len() == 0 {
                    core::result::Result::Ok(<#name #ty_generics as codec::DecodeDefault>::default())
                } else {
                    #top_decode_body
                }
            }
        }
    };

    gen.into()
}
