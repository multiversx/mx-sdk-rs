use super::parse::attributes::extract_doc;
use proc_macro::TokenStream;
use quote::quote;

fn field_snippet(index: usize, field: &syn::Field) -> proc_macro2::TokenStream {
    let field_docs = extract_doc(field.attrs.as_slice());
    let field_name_str = if let Some(ident) = &field.ident {
        ident.to_string()
    } else {
        index.to_string()
    };
    let field_ty = &field.ty;
    quote! {
        field_descriptions.push(multiversx_sc::abi::StructFieldDescription {
            docs: &[ #(#field_docs),* ],
            name: #field_name_str,
            field_type: <#field_ty>::type_name(),
        });
        <#field_ty>::provide_type_descriptions(accumulator);
    }
}

fn fields_snippets(fields: &syn::Fields) -> Vec<proc_macro2::TokenStream> {
    match fields {
        syn::Fields::Named(fields_named) => fields_named
            .named
            .iter()
            .enumerate()
            .map(|(index, field)| field_snippet(index, field))
            .collect(),
        syn::Fields::Unnamed(fields_unnamed) => fields_unnamed
            .unnamed
            .iter()
            .enumerate()
            .map(|(index, field)| field_snippet(index, field))
            .collect(),
        syn::Fields::Unit => Vec::new(),
    }
}

pub fn type_abi_derive(ast: &syn::DeriveInput) -> TokenStream {
    let type_docs = extract_doc(ast.attrs.as_slice());
    let type_description_impl = match &ast.data {
        syn::Data::Struct(data_struct) => {
            let struct_field_snippets = fields_snippets(&data_struct.fields);
            quote! {
                fn provide_type_descriptions<TDC: multiversx_sc::abi::TypeDescriptionContainer>(accumulator: &mut TDC) {
                    let type_name = Self::type_name();
                    if !accumulator.contains_type(&type_name) {
                        accumulator.reserve_type_name(type_name.clone());
                        let mut field_descriptions = multiversx_sc::types::heap::Vec::new();
                        #(#struct_field_snippets)*
                        accumulator.insert(
                            type_name.clone(),
                            multiversx_sc::abi::TypeDescription {
                                docs: &[ #(#type_docs),* ],
                                name: type_name,
                                contents: multiversx_sc::abi::TypeContents::Struct(field_descriptions),
                            },
                        );
                    }
                }
            }
        },
        syn::Data::Enum(data_enum) => {
            let enum_variant_snippets: Vec<proc_macro2::TokenStream> = data_enum
                .variants
                .iter()
                .enumerate()
                .map(|(variant_index, variant)| {
                    let variant_docs = extract_doc(variant.attrs.as_slice());
                    let variant_name_str = variant.ident.to_string();
                    let variant_field_snippets = fields_snippets(&variant.fields);
                    quote! {
                        let mut field_descriptions = multiversx_sc::types::heap::Vec::new();
                        #(#variant_field_snippets)*
                        variant_descriptions.push(multiversx_sc::abi::EnumVariantDescription {
                            docs: &[ #(#variant_docs),* ],
                            discriminant: #variant_index,
                            name: #variant_name_str,
                            fields: field_descriptions,
                        });
                    }
                })
                .collect();
            quote! {
                fn provide_type_descriptions<TDC: multiversx_sc::abi::TypeDescriptionContainer>(accumulator: &mut TDC) {
                    let type_name = Self::type_name();
                    if !accumulator.contains_type(&type_name) {
                        accumulator.reserve_type_name(type_name.clone());
                        let mut variant_descriptions = multiversx_sc::types::heap::Vec::new();
                        #(#enum_variant_snippets)*
                        accumulator.insert(
                            type_name.clone(),
                            multiversx_sc::abi::TypeDescription {
                                docs: &[ #(#type_docs),* ],
                                name: type_name,
                                contents: multiversx_sc::abi::TypeContents::Enum(variant_descriptions),
                            },
                        );
                    }
                }
            }
        },
        syn::Data::Union(_) => panic!("Union not supported!"),
    };

    let name = &ast.ident;
    let name_str = name.to_string();
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
    let type_abi_impl = quote! {
        impl #impl_generics multiversx_sc::abi::TypeAbi for #name #ty_generics #where_clause {
            fn type_name() -> multiversx_sc::abi::TypeName {
                #name_str.into()
            }

            #type_description_impl
        }
    };
    type_abi_impl.into()
}
