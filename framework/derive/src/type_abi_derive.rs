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
        field_descriptions.push(multiversx_sc::abi::StructFieldDescription::new(
            &[ #(#field_docs),* ],
            #field_name_str,
            <#field_ty>::type_name(),
        ));
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
                            multiversx_sc::abi::TypeDescription::new(
                                &[ #(#type_docs),* ],
                                type_name,
                                multiversx_sc::abi::TypeContents::Struct(field_descriptions),
                            ),
                        );
                    }
                }
            }
        },
        syn::Data::Enum(data_enum) => {
            //(index of last explicit, value)
            let mut previous_disc: Vec<(usize, usize)> = Vec::new();
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
                    let type_name = Self::type_name();
                    if !accumulator.contains_type(&type_name) {
                        accumulator.reserve_type_name(type_name.clone());
                        let mut variant_descriptions = multiversx_sc::types::heap::Vec::new();
                        #(#enum_variant_snippets)*
                        accumulator.insert(
                            type_name.clone(),
                            multiversx_sc::abi::TypeDescription::new(
                                &[ #(#type_docs),* ],
                                type_name,
                                multiversx_sc::abi::TypeContents::Enum(variant_descriptions),
                            ),
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

pub fn get_discriminant(
    variant_index: usize,
    variant: &syn::Variant,
    previous_disc: &mut Vec<(usize, usize)>,
) -> proc_macro2::TokenStream {
    //if it has explicit discriminant
    if let Some((_, syn::Expr::Lit(expr))) = &variant.discriminant {
        let lit = match &expr.lit {
            syn::Lit::Int(val) => {
                let value = val.base10_parse().unwrap_or_else(|_| {
                    panic!("Can not unwrap int value from explicit discriminant")
                });
                previous_disc.push((variant_index, value));
                value
            },
            _ => panic!("Only integer values as discriminants"), //unreachable
        };
        return quote! { #lit};
    }

    //if no explicit discriminant, check previous discriminants
    //get previous explicit + 1 if there has been any explicit before
    let next_value = match previous_disc.last() {
        //there are previous explicit discriminants
        Some((prev_index, prev_value)) if *prev_index < variant_index - 1 => {
            prev_value + variant_index - prev_index
        },
        Some((_, prev_value)) => prev_value + 1,

        //vec is empty, first element is added
        None => 0,
    };

    quote! { #next_value}
}
