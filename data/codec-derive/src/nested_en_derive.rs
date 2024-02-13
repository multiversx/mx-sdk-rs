use crate::util::*;
use proc_macro::TokenStream;
use quote::quote;

pub fn dep_encode_snippet(value: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    quote! {
        codec::NestedEncode::dep_encode_or_handle_err(&#value, __dest__, __h__)?;
    }
}

fn variant_dep_encode_snippets(
    name: &syn::Ident,
    data_enum: &syn::DataEnum,
) -> Vec<proc_macro2::TokenStream> {
    let mut previous_disc: Vec<ExplicitDiscriminant> = Vec::new();
    data_enum
        .variants
        .iter()
        .enumerate()
        .map(|(variant_index, variant)| {
            let variant_discriminant = get_discriminant(variant_index, variant, &mut previous_disc);
            let variant_ident = &variant.ident;
            let local_var_declarations =
                fields_decl_syntax(&variant.fields, local_variable_for_field);
            let variant_field_snippets = fields_snippets(&variant.fields, |index, field| {
                dep_encode_snippet(&local_variable_for_field(index, field))
            });
            quote! {
                #name::#variant_ident #local_var_declarations => {
                    codec::NestedEncode::dep_encode_or_handle_err(&#variant_discriminant, __dest__, __h__)?;
                    #(#variant_field_snippets)*
                },
            }
        })
        .collect()
}

pub fn nested_encode_impl(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
    let gen = match &ast.data {
        syn::Data::Struct(data_struct) => {
            let field_dep_encode_snippets = fields_snippets(&data_struct.fields, |index, field| {
                dep_encode_snippet(&self_field_expr(index, field))
            });
            quote! {
                impl #impl_generics codec::NestedEncode for #name #ty_generics #where_clause {
                    fn dep_encode_or_handle_err<O, H>(&self, __dest__: &mut O, __h__: H) -> core::result::Result<(), H::HandledErr>
                    where
                        O: codec::NestedEncodeOutput,
                        H: codec::EncodeErrorHandler,
                    {
                        #(#field_dep_encode_snippets)*
                        core::result::Result::Ok(())
                    }
                }
            }
        },
        syn::Data::Enum(data_enum) => {
            validate_enum_variants(&data_enum.variants);

            let variant_dep_encode_snippets = variant_dep_encode_snippets(name, data_enum);

            quote! {
                impl #impl_generics codec::NestedEncode for #name #ty_generics #where_clause {
                    fn dep_encode_or_handle_err<O, H>(&self, __dest__: &mut O, __h__: H) -> core::result::Result<(), H::HandledErr>
                    where
                        O: codec::NestedEncodeOutput,
                        H: codec::EncodeErrorHandler,
                    {
                        match self {
                            #(#variant_dep_encode_snippets)*
                        };
                        core::result::Result::Ok(())
                    }
                }
            }
        },
        syn::Data::Union(_) => panic!("Union not supported"),
    };

    gen.into()
}
