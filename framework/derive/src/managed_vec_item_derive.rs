use proc_macro::TokenStream;
use quote::quote;

pub fn managed_vec_item_derive(ast: &syn::DeriveInput) -> TokenStream {
    match &ast.data {
        syn::Data::Struct(data_struct) => struct_derive(data_struct, ast),
        syn::Data::Enum(data_enum) => enum_derive(data_enum, ast),
        syn::Data::Union(_) => panic!("Union not supported!"),
    }
}

fn generate_struct_payload_nested_tuple(fields: &syn::Fields) -> proc_macro2::TokenStream {
    match fields {
        syn::Fields::Named(fields_named) => {
            let types: Vec<_> = fields_named.named.iter().map(|field| &field.ty).collect();
            let mut result = quote! { () };
            for ty in types.iter().rev() {
                result = quote! { (#ty, #result) };
            }
            result
        }
        _ => {
            panic!("ManagedVecItem only supports named fields")
        }
    }
}

fn generate_enum_payload_nested_tuple(data_enum: &syn::DataEnum) -> proc_macro2::TokenStream {
    let types: Vec<_> = data_enum
        .variants
        .iter()
        .filter_map(|variant| single_fields_type(&variant.fields))
        .collect();
    let mut result = quote! { () };
    for ty in types.iter().rev() {
        result = quote! { (#ty, #result) };
    }
    result
}

fn generate_skips_reserialization_snippets(fields: &syn::Fields) -> Vec<proc_macro2::TokenStream> {
    match fields {
        syn::Fields::Named(fields_named) => fields_named
            .named
            .iter()
            .map(|field| {
                let type_name = &field.ty;
                quote! {
                    <#type_name as multiversx_sc::types::ManagedVecItem>::SKIPS_RESERIALIZATION
                }
            })
            .collect(),
        _ => {
            panic!("ManagedVecItem only supports named fields")
        }
    }
}

fn generate_read_from_payload_snippets(fields: &syn::Fields) -> Vec<proc_macro2::TokenStream> {
    match fields {
        syn::Fields::Named(fields_named) => fields_named
            .named
            .iter()
            .map(|field| {
                let field_ident = &field.ident;
                quote! {
                    #field_ident: multiversx_sc::types::managed_vec_item_read_from_payload_index(payload, &mut index),
                }
            })
            .collect(),
        _ => {
            panic!("ManagedVecItem only supports named fields")
        }
    }
}

fn generate_save_to_payload_snippets(fields: &syn::Fields) -> Vec<proc_macro2::TokenStream> {
    match fields {
        syn::Fields::Named(fields_named) => fields_named
            .named
            .iter()
            .map(|field| {
                let field_ident = &field.ident;
                quote! {
                    multiversx_sc::types::managed_vec_item_save_to_payload_index(self.#field_ident, payload, &mut index);
                }
            })
            .collect(),
        _ => {
            panic!("ManagedVecItem only supports named fields")
        }
    }
}

fn variants_have_fields(data_enum: &syn::DataEnum) -> bool {
    data_enum
        .variants
        .iter()
        .any(|variant| !variant.fields.is_empty())
}

fn enum_derive(data_enum: &syn::DataEnum, ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
    let payload_nested_tuple = generate_enum_payload_nested_tuple(data_enum);
    let skips_reserialization = !variants_have_fields(data_enum);

    let mut reader_match_arms = Vec::<proc_macro2::TokenStream>::new();
    let mut writer_match_arms = Vec::<proc_macro2::TokenStream>::new();
    let last_index = data_enum.variants.len() - 1;
    for (variant_index, variant) in data_enum.variants.iter().enumerate() {
        let variant_index_u8 = variant_index as u8;
        let variant_ident = &variant.ident;
        let has_field = single_fields_type(&variant.fields).is_some();
        let field_init = if has_field {
            quote! {
                (multiversx_sc::types::managed_vec_item_read_from_payload_index(
                    payload,
                    &mut index
                ))
            }
        } else {
            quote! {}
        };

        if variant_index == last_index {
            // last one gets a `_`
            // we don't treat bad discrminant errors
            // (there is currently no error mechanism for ManagedVecItem)
            reader_match_arms.push(quote! {
                _ => #name::#variant_ident #field_init ,
            });
        } else {
            reader_match_arms.push(quote! {
                #variant_index_u8 => #name::#variant_ident #field_init ,
            });
        }

        if has_field {
            writer_match_arms.push(quote! {
                #name::#variant_ident (__self_0) => {
                    multiversx_sc::types::managed_vec_item_save_to_payload_index(#variant_index_u8, payload, &mut index);
                    multiversx_sc::types::managed_vec_item_save_to_payload_index(__self_0, payload, &mut index);
                }
            });
        } else {
            writer_match_arms.push(quote! {
                #name::#variant_ident => {
                    multiversx_sc::types::managed_vec_item_save_to_payload_index(#variant_index_u8, payload, &mut index);
                }
            });
        }
    }

    let result = quote! {
        impl #impl_generics multiversx_sc::types::ManagedVecItem for #name #ty_generics #where_clause {
            type PAYLOAD = <#payload_nested_tuple as multiversx_sc::types::ManagedVecItemEnumPayloadTuple>::EnumPayload;
            const SKIPS_RESERIALIZATION: bool = #skips_reserialization;
            type Ref<'a> = multiversx_sc::types::Ref<'a, Self>;

            fn read_from_payload(payload: &Self::PAYLOAD) -> Self {
                let mut index = 0;

                unsafe {
                    let discriminant: u8 = multiversx_sc::types::managed_vec_item_read_from_payload_index(
                        payload,
                        &mut index,
                    );

                    match discriminant {
                        #(#reader_match_arms)*
                    }
                }
            }

            unsafe fn borrow_from_payload<'a>(payload: &Self::PAYLOAD) -> Self::Ref<'a> {
                multiversx_sc::types::Ref::new(Self::read_from_payload(payload))
            }

            fn save_to_payload(self, payload: &mut Self::PAYLOAD) {
                let mut index = 0;

                unsafe {
                    match self {
                        #(#writer_match_arms)*
                    };
                }
            }
        }
    };
    result.into()
}

fn single_fields_type(fields: &syn::Fields) -> Option<syn::Type> {
    match fields {
        syn::Fields::Named(_) => {
            panic!("named fields currently not supported, only single unnamed fields supported, of type Variant(T)")
        }
        syn::Fields::Unnamed(fields_unnamed) => {
            assert_eq!(
                fields_unnamed.unnamed.len(),
                1,
                "only single unnamed fields supported, of type Variant(T)"
            );
            Some(fields_unnamed.unnamed.first().unwrap().ty.clone())
        }
        syn::Fields::Unit => None,
    }
}

fn struct_derive(data_struct: &syn::DataStruct, ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
    let payload_nested_tuple = generate_struct_payload_nested_tuple(&data_struct.fields);
    let skips_reserialization_snippets =
        generate_skips_reserialization_snippets(&data_struct.fields);
    let read_from_payload_snippets = generate_read_from_payload_snippets(&data_struct.fields);
    let save_to_payload_snippets = generate_save_to_payload_snippets(&data_struct.fields);

    let result = quote! {
        impl #impl_generics multiversx_sc::types::ManagedVecItem for #name #ty_generics #where_clause {
            type PAYLOAD = <#payload_nested_tuple as multiversx_sc::types::ManagedVecItemStructPayloadTuple>::StructPayload;
            const SKIPS_RESERIALIZATION: bool = #(#skips_reserialization_snippets)&&*;
            type Ref<'a> = multiversx_sc::types::Ref<'a, Self>;

            fn read_from_payload(payload: &Self::PAYLOAD) -> Self {
                let mut index = 0;

                unsafe {
                    #name {
                        #(#read_from_payload_snippets)*
                    }
                }
            }

            unsafe fn borrow_from_payload<'a>(payload: &Self::PAYLOAD) -> Self::Ref<'a> {
                multiversx_sc::types::Ref::new(Self::read_from_payload(payload))
            }

            fn save_to_payload(self, payload: &mut Self::PAYLOAD) {
                let mut index = 0;
                unsafe {
                    #(#save_to_payload_snippets)*
                }
            }
        }
    };
    result.into()
}
