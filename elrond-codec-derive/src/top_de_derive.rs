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

/// Generates a default-value deserializer snippet automatically.
/// Currently only does so for enums whose first variant is fieldless.
/// Also generates the snippet for `top_decode_or_exit`.
/// Not called for TopDecodeOrDefault, since that one already provides an explicit default.
fn auto_default(ast: &syn::DeriveInput) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
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
            let auto_default_or_exit = quote! {
                if top_input.byte_len() == 0 {
                    return #name::#first_variant_ident;
                }
            };
            return (auto_default, auto_default_or_exit);
        }
    }

    // returns nothing by default
    (quote! {}, quote! {})
}

/// Only returns the trait implementation method bodies, without the impl or method definitions.
fn top_decode_method_bodies(
    ast: &syn::DeriveInput,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let name = &ast.ident;
    match &ast.data {
        syn::Data::Struct(data_struct) => {
            let field_dep_decode_snippets =
                fields_decl_syntax(&data_struct.fields, |index, field| {
                    dep_decode_snippet(index, field, &quote! {&mut nested_buffer})
                });
            let field_dep_encode_or_exit_snippets =
                fields_decl_syntax(&data_struct.fields, |index, field| {
                    dep_decode_or_exit_snippet(index, field, &quote! {&mut nested_buffer})
                });

            let top_decode_body = quote! {
                let mut nested_buffer = top_input.into_nested_buffer();
                let result = #name #field_dep_decode_snippets ;
                if !elrond_codec::NestedDecodeInput::is_depleted(&nested_buffer) {
                    return core::result::Result::Err(elrond_codec::DecodeError::INPUT_TOO_LONG);
                }
                core::result::Result::Ok(result)
            };
            let top_decode_or_exit_body = quote! {
                let mut nested_buffer = top_input.into_nested_buffer();
                let result = #name #field_dep_encode_or_exit_snippets ;
                if !elrond_codec::NestedDecodeInput::is_depleted(&nested_buffer) {
                    exit(c, elrond_codec::DecodeError::INPUT_TOO_LONG);
                }
                result
            };
            (top_decode_body, top_decode_or_exit_body)
        },
        syn::Data::Enum(data_enum) => {
            assert!(
                data_enum.variants.len() < 256,
                "enums with more than 256 variants not supported"
            );
            if is_fieldless_enum(data_enum) {
                // fieldless enums are special, they can be top-decoded as u8 directly
                let top_decode_arms = fieldless_enum_match_arm_result_ok(name, data_enum);
                let top_decode_or_exit_arms = fieldless_enum_match_arm(name, data_enum);

                let top_decode_body = quote! {
                    match <u8 as elrond_codec::TopDecode>::top_decode(top_input)? {
                        #(#top_decode_arms)*
                        _ => core::result::Result::Err(elrond_codec::DecodeError::INVALID_VALUE),
                    }
                };
                let top_decode_or_exit_body = quote! {
                    match <u8 as elrond_codec::TopDecode>::top_decode_or_exit(top_input, c.clone(), exit) {
                        #(#top_decode_or_exit_arms)*
                        _ => exit(c, elrond_codec::DecodeError::INVALID_VALUE),
                    }
                };
                (top_decode_body, top_decode_or_exit_body)
            } else {
                let variant_dep_decode_snippets =
                    variant_dep_decode_snippets(name, data_enum, &quote! {&mut nested_buffer});
                let variant_dep_decode_or_exit_snippets = variant_dep_decode_or_exit_snippets(
                    name,
                    data_enum,
                    &quote! {&mut nested_buffer},
                );

                let top_decode_body = quote! {
                    let mut nested_buffer = top_input.into_nested_buffer();
                    let result = match <u8 as elrond_codec::NestedDecode>::dep_decode(&mut nested_buffer)? {
                        #(#variant_dep_decode_snippets)*
                        _ => core::result::Result::Err(elrond_codec::DecodeError::INVALID_VALUE),
                    };
                    if !elrond_codec::NestedDecodeInput::is_depleted(&nested_buffer) {
                        return core::result::Result::Err(elrond_codec::DecodeError::INPUT_TOO_LONG);
                    }
                    result
                };
                let top_decode_or_exit_body = quote! {
                    let mut nested_buffer = top_input.into_nested_buffer();
                    let result = match <u8 as elrond_codec::NestedDecode>::dep_decode_or_exit(&mut nested_buffer, c.clone(), exit) {
                        #(#variant_dep_decode_or_exit_snippets)*
                        _ => exit(c, elrond_codec::DecodeError::INVALID_VALUE),
                    };
                    if !elrond_codec::NestedDecodeInput::is_depleted(&nested_buffer) {
                        exit(c, elrond_codec::DecodeError::INPUT_TOO_LONG);
                    }
                    result
                };
                (top_decode_body, top_decode_or_exit_body)
            }
        },
        syn::Data::Union(_) => panic!("Union not supported"),
    }
}

pub fn top_decode_impl(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
    let (top_decode_body, top_decode_or_exit_body) = top_decode_method_bodies(ast);
    let (auto_default, auto_default_or_exit) = auto_default(ast);

    let gen = quote! {
        impl #impl_generics elrond_codec::TopDecode for #name #ty_generics #where_clause {
            fn top_decode<I: elrond_codec::TopDecodeInput>(top_input: I) -> core::result::Result<Self, elrond_codec::DecodeError> {
                #auto_default
                #top_decode_body
            }

            fn top_decode_or_exit<I: elrond_codec::TopDecodeInput, ExitCtx: Clone>(
                top_input: I,
                c: ExitCtx,
                exit: fn(ExitCtx, elrond_codec::DecodeError) -> !,
            ) -> Self {
                #auto_default_or_exit
                #top_decode_or_exit_body
            }
        }
    };

    gen.into()
}

pub fn top_decode_or_default_impl(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
    let (top_decode_body, top_decode_or_exit_body) = top_decode_method_bodies(ast);

    let gen = quote! {
        impl #impl_generics elrond_codec::TopDecode for #name #ty_generics #where_clause {
            fn top_decode<I: elrond_codec::TopDecodeInput>(top_input: I) -> core::result::Result<Self, elrond_codec::DecodeError> {
                if top_input.byte_len() == 0 {
                    Ok(<#name #ty_generics as elrond_codec::DecodeDefault>::default())
                } else {
                    #top_decode_body
                }
            }

            fn top_decode_or_exit<I: elrond_codec::TopDecodeInput, ExitCtx: Clone>(
                top_input: I,
                c: ExitCtx,
                exit: fn(ExitCtx, elrond_codec::DecodeError) -> !,
            ) -> Self {
                if top_input.byte_len() == 0 {
                    <#name #ty_generics as elrond_codec::DecodeDefault>::default()
                } else {
                    #top_decode_or_exit_body
                }
            }
        }
    };

    gen.into()
}
