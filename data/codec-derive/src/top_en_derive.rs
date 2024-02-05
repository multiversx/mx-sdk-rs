use proc_macro::TokenStream;
use quote::quote;

use crate::{nested_en_derive::dep_encode_snippet, util::*};

pub fn variant_top_encode_snippets(
    name: &syn::Ident,
    data_enum: &syn::DataEnum,
) -> Vec<proc_macro2::TokenStream> {
    data_enum
        .variants
        .iter()
        .enumerate()
        .map(|(variant_index, variant)| {
            let discriminant_u8 = variant_index as u8;
            let variant_ident = &variant.ident;
            if variant.fields.is_empty() {
                // top-encode discriminant directly
                quote! {
                    #name::#variant_ident =>
                        codec::TopEncode::top_encode_or_handle_err(&#discriminant_u8, output, __h__),
                }
            } else {
                // dep-encode to buffer first
                let local_var_declarations =
                    fields_decl_syntax(&variant.fields, local_variable_for_field);
                let variant_field_snippets = fields_snippets(&variant.fields, |index, field| {
                    dep_encode_snippet(&local_variable_for_field(index, field))
                });
                quote! {
                    #name::#variant_ident #local_var_declarations => {
                        let mut __buffer__ = output.start_nested_encode();
                        let __dest__ = &mut __buffer__;
                        codec::NestedEncode::dep_encode_or_handle_err(&#discriminant_u8, __dest__, __h__)?;
                        #(#variant_field_snippets)*
                        output.finalize_nested_encode(__buffer__);
                        core::result::Result::Ok(())
                    },
                }
            }
        })
        .collect()
}

/// Only returns the trait implementation method bodies, without the impl or method definitions.
fn top_encode_method_body(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    match &ast.data {
        syn::Data::Struct(data_struct) => {
            let field_dep_encode_snippets = fields_snippets(&data_struct.fields, |index, field| {
                dep_encode_snippet(&self_field_expr(index, field))
            });
            quote! {
                let mut __buffer__ = output.start_nested_encode();
                let __dest__ = &mut __buffer__;
                #(#field_dep_encode_snippets)*
                output.finalize_nested_encode(__buffer__);
                core::result::Result::Ok(())
            }
        },
        syn::Data::Enum(data_enum) => {
            validate_enum_variants(&data_enum.variants);

            let variant_top_encode_snippets = variant_top_encode_snippets(name, data_enum);

            quote! {
                match self {
                    #(#variant_top_encode_snippets)*
                }
            }
        },
        syn::Data::Union(_) => panic!("Union not supported"),
    }
}

pub fn top_encode_impl(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
    let top_encode_body = top_encode_method_body(ast);

    let gen = quote! {
        impl #impl_generics codec::TopEncode for #name #ty_generics #where_clause {
            fn top_encode_or_handle_err<O, H>(&self, output: O, __h__: H) -> core::result::Result<(), H::HandledErr>
            where
                O: codec::TopEncodeOutput,
                H: codec::EncodeErrorHandler,
            {
                #top_encode_body
            }
        }
    };
    gen.into()
}

pub fn top_encode_or_default_impl(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
    let top_encode_body = top_encode_method_body(ast);

    let gen = quote! {
        impl #impl_generics codec::TopEncode for #name #ty_generics #where_clause {
            fn top_encode_or_handle_err<O, H>(&self, output: O, __h__: H) -> core::result::Result<(), H::HandledErr>
            where
                O: codec::TopEncodeOutput,
                H: codec::EncodeErrorHandler,
            {
                if codec::EncodeDefault::is_default(self) {
                    output.set_slice_u8(&[]);
                    core::result::Result::Ok(())
                } else {
                    #top_encode_body
                }
            }
        }
    };
    gen.into()
}
