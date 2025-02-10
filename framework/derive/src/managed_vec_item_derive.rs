use proc_macro::TokenStream;
use quote::quote;

pub fn managed_vec_item_derive(ast: &syn::DeriveInput) -> TokenStream {
    match &ast.data {
        syn::Data::Struct(data_struct) => struct_derive(data_struct, ast),
        syn::Data::Enum(data_enum) => enum_derive(data_enum, ast),
        syn::Data::Union(_) => panic!("Union not supported!"),
    }
}

fn generate_payload_nested_tuple(fields: &syn::Fields) -> proc_macro2::TokenStream {
    match fields {
        syn::Fields::Named(fields_named) => {
            let types: Vec<_> = fields_named.named.iter().map(|field| &field.ty).collect();
            let mut result = quote! { () };
            for ty in types.iter().rev() {
                result = quote! { (#ty, #result) };
            }
            result
        },
        _ => {
            panic!("ManagedVecItem only supports named fields")
        },
    }
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
        },
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

fn enum_derive(data_enum: &syn::DataEnum, ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();

    let mut reader_match_arms = Vec::<proc_macro2::TokenStream>::new();
    let mut writer_match_arms = Vec::<proc_macro2::TokenStream>::new();
    for (variant_index, variant) in data_enum.variants.iter().enumerate() {
        let variant_index_u8 = variant_index as u8;
        let variant_ident = &variant.ident;
        assert!(variant.fields.is_empty(), "Only fieldless enums supported");
        reader_match_arms.push(quote! {
            #variant_index_u8 => #name::#variant_ident,
        });
        writer_match_arms.push(quote! {
            #name::#variant_ident => #variant_index_u8,
        });
    }

    let first_variant_ident = &data_enum.variants[0];
    reader_match_arms.push(quote! {
        _ => #name::#first_variant_ident,
    });

    let gen = quote! {
        impl #impl_generics multiversx_sc::types::ManagedVecItem for #name #ty_generics #where_clause {
            type PAYLOAD = multiversx_sc::types::ManagedVecItemPayloadBuffer<1>;
            const SKIPS_RESERIALIZATION: bool = true;
            type Ref<'a> = Self;

            fn read_from_payload(payload: &Self::PAYLOAD) -> Self {
                let discriminant = <u8 as multiversx_sc::types::ManagedVecItem>::read_from_payload(payload);
                match discriminant {
                    #(#reader_match_arms)*
                }
            }

            unsafe fn borrow_from_payload<'a>(payload: &Self::PAYLOAD) -> Self::Ref<'a> {
                Self::read_from_payload(payload)
            }

            fn save_to_payload(self, payload: &mut Self::PAYLOAD) {
                let discriminant = match self {
                    #(#writer_match_arms)*
                };
                <u8 as multiversx_sc::types::ManagedVecItem>::save_to_payload(discriminant, payload);
            }
        }
    };
    gen.into()
}

fn struct_derive(data_struct: &syn::DataStruct, ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
    let payload_nested_tuple = generate_payload_nested_tuple(&data_struct.fields);
    let skips_reserialization_snippets =
        generate_skips_reserialization_snippets(&data_struct.fields);
    let read_from_payload_snippets = generate_read_from_payload_snippets(&data_struct.fields);
    let save_to_payload_snippets = generate_save_to_payload_snippets(&data_struct.fields);

    let gen = quote! {
        impl #impl_generics multiversx_sc::types::ManagedVecItem for #name #ty_generics #where_clause {
            type PAYLOAD = <#payload_nested_tuple as multiversx_sc::types::ManagedVecItemStructPlTuple>::StructPayload;
            const SKIPS_RESERIALIZATION: bool = #(#skips_reserialization_snippets)&&*;
            type Ref<'a> = multiversx_sc::types::ManagedVecRef<'a, Self>;

            fn read_from_payload(payload: &Self::PAYLOAD) -> Self {
                let mut index = 0;

                unsafe {
                    #name {
                        #(#read_from_payload_snippets)*
                    }
                }
            }

            unsafe fn borrow_from_payload<'a>(payload: &Self::PAYLOAD) -> Self::Ref<'a> {
                multiversx_sc::types::ManagedVecRef::new(Self::read_from_payload(payload))
            }

            fn save_to_payload(self, payload: &mut Self::PAYLOAD) {
                let mut index = 0;
                unsafe {
                    #(#save_to_payload_snippets)*
                }
            }
        }
    };
    gen.into()
}
