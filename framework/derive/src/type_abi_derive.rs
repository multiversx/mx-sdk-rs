use crate::parse::attributes::extract_macro_attributes;

use super::parse::attributes::extract_doc;
use quote::{quote, ToTokens};

const BITFLAGS_PATH: &str = ":: __private :: PublicFlags :: Internal";
const BITFLAGS_PRIMITIVE: &str = "Primitive";
pub struct ExplicitDiscriminant {
    pub variant_index: usize,
    pub value: usize,
}

fn field_snippet(index: usize, field: &syn::Field) -> proc_macro2::TokenStream {
    let field_docs = extract_doc(field.attrs.as_slice());
    let field_name_str = if let Some(ident) = &field.ident {
        ident.to_string()
    } else {
        index.to_string()
    };
    let field_ty = sanitize_field_type_path(&field.ty);
    quote! {
        field_descriptions.push(multiversx_sc::abi::StructFieldDescription::new(
            &[ #(#field_docs),* ],
            #field_name_str,
            <#field_ty>::type_names(),
        ));
        <#field_ty>::provide_type_descriptions(accumulator);
    }
}

fn sanitize_field_type_path(field_type: &syn::Type) -> syn::Type {
    if let syn::Type::Path(p) = field_type {
        let mut path = p.path.clone();

        if path.to_token_stream().to_string().contains(BITFLAGS_PATH) {
            let modified_path = path.segments.last_mut().unwrap();
            modified_path.ident = syn::Ident::new(BITFLAGS_PRIMITIVE, modified_path.ident.span());

            return syn::Type::Path(syn::TypePath {
                qself: p.qself.clone(),
                path,
            });
        }
    }

    field_type.clone()
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

pub fn type_abi_derive(input: proc_macro::TokenStream) -> proc_macro2::TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let name_str = name.to_string();
    let type_docs = extract_doc(ast.attrs.as_slice());
    let macro_attributes = extract_macro_attributes(ast.attrs.as_slice());
    if macro_attributes.is_empty() {
        println!("Warning! {name_str} #[type_abi] implementation sees no derive traits. Make sure that the derive attribute comes after #[type_abi]");
    }

    let type_description_impl = match &ast.data {
        syn::Data::Struct(data_struct) => {
            let struct_field_snippets = fields_snippets(&data_struct.fields);
            quote! {
                fn provide_type_descriptions<TDC: multiversx_sc::abi::TypeDescriptionContainer>(accumulator: &mut TDC) {
                    let type_names = Self::type_names();
                    if !accumulator.contains_type(&type_names.abi) {
                        accumulator.reserve_type_name(type_names.clone());
                        let mut field_descriptions = multiversx_sc::types::heap::Vec::new();
                        #(#struct_field_snippets)*
                        accumulator.insert(
                            type_names.clone(),
                            multiversx_sc::abi::TypeDescription::new(
                                &[ #(#type_docs),* ],
                                type_names,
                                multiversx_sc::abi::TypeContents::Struct(field_descriptions),
                                &[ #(#macro_attributes),* ],
                            ),
                        );
                    }
                }
            }
        }
        syn::Data::Enum(data_enum) => {
            let mut previous_disc: Vec<ExplicitDiscriminant> = Vec::new();
            let enum_variant_snippets: Vec<proc_macro2::TokenStream> = data_enum
                .variants
                .iter()
                .enumerate()
                .map(|(variant_index, variant)| {
                    let variant_docs = extract_doc(variant.attrs.as_slice());
                    let variant_name_str = variant.ident.to_string();
                    let variant_field_snippets = fields_snippets(&variant.fields);
                    let variant_discriminant =
                        get_discriminant(variant_index, variant, &mut previous_disc);
                    quote! {
                        let mut field_descriptions = multiversx_sc::types::heap::Vec::new();
                        #(#variant_field_snippets)*
                        variant_descriptions.push(multiversx_sc::abi::EnumVariantDescription::new(
                            &[ #(#variant_docs),* ],
                            #variant_name_str,
                            #variant_discriminant,
                            field_descriptions,
                        ));
                    }
                })
                .collect();
            quote! {
                fn provide_type_descriptions<TDC: multiversx_sc::abi::TypeDescriptionContainer>(accumulator: &mut TDC) {
                    let type_names = Self::type_names();
                    if !accumulator.contains_type(&type_names.abi) {
                        accumulator.reserve_type_name(type_names.clone());
                        let mut variant_descriptions = multiversx_sc::types::heap::Vec::new();
                        #(#enum_variant_snippets)*
                        accumulator.insert(
                            type_names.clone(),
                            multiversx_sc::abi::TypeDescription::new(
                                &[ #(#type_docs),* ],
                                type_names,
                                multiversx_sc::abi::TypeContents::Enum(variant_descriptions),
                                &[ #(#macro_attributes),* ],
                            ),
                        );
                    }
                }
            }
        }
        syn::Data::Union(_) => panic!("Union not supported!"),
    };

    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
    quote! {
        impl #impl_generics multiversx_sc::abi::TypeAbiFrom<Self> for #name #ty_generics #where_clause {}
        impl #impl_generics multiversx_sc::abi::TypeAbiFrom<&Self> for #name #ty_generics #where_clause {}

        impl #impl_generics multiversx_sc::abi::TypeAbi for #name #ty_generics #where_clause {
            type Unmanaged = Self;

            fn type_name() -> multiversx_sc::abi::TypeName {
                #name_str.into()
            }
            #type_description_impl
        }
    }
}

pub fn type_abi_full(input: proc_macro::TokenStream) -> proc_macro2::TokenStream {
    let input_conv = proc_macro2::TokenStream::from(input.clone());
    let derive_code = type_abi_derive(input);
    quote! {
        #input_conv
        #derive_code
    }
}

pub fn get_discriminant(
    variant_index: usize,
    variant: &syn::Variant,
    previous_disc: &mut Vec<ExplicitDiscriminant>,
) -> proc_macro2::TokenStream {
    // if it has explicit discriminant
    if let Some((_, syn::Expr::Lit(expr))) = &variant.discriminant {
        let lit = match &expr.lit {
            syn::Lit::Int(val) => {
                let value = val.base10_parse().unwrap_or_else(|_| {
                    panic!("Can not unwrap int value from explicit discriminant")
                });
                previous_disc.push(ExplicitDiscriminant {
                    variant_index,
                    value,
                });
                value
            }
            _ => panic!("Only integer values as discriminants"), // theoretically covered by the compiler
        };
        return quote! { #lit};
    }

    // if no explicit discriminant, check previous discriminants
    // get previous explicit + 1 if there has been any explicit before
    let next_value = match previous_disc.last() {
        // there are previous explicit discriminants
        Some(ExplicitDiscriminant {
            variant_index: prev_index,
            value: prev_value,
        }) if *prev_index < variant_index - 1 => prev_value + variant_index - prev_index,
        Some(ExplicitDiscriminant {
            variant_index: _,
            value: prev_value,
        }) => prev_value + 1,

        // vec is empty, return index
        None => variant_index,
    };

    quote! { #next_value}
}
