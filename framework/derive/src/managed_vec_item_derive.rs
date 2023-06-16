use proc_macro::TokenStream;
use quote::quote;

pub fn managed_vec_item_derive(ast: &syn::DeriveInput) -> TokenStream {
    match &ast.data {
        syn::Data::Struct(data_struct) => struct_derive(data_struct, ast),
        syn::Data::Enum(data_enum) => enum_derive(data_enum, ast),
        syn::Data::Union(_) => panic!("Union not supported!"),
    }
}

fn type_payload_size(type_name: &syn::Type) -> proc_macro2::TokenStream {
    quote! {
        <#type_name as multiversx_sc::types::ManagedVecItem>::PAYLOAD_SIZE
    }
}

fn generate_payload_snippets(fields: &syn::Fields) -> Vec<proc_macro2::TokenStream> {
    match fields {
        syn::Fields::Named(fields_named) => fields_named
            .named
            .iter()
            .map(|field| type_payload_size(&field.ty))
            .collect(),
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

fn generate_from_byte_reader_snippets(fields: &syn::Fields) -> Vec<proc_macro2::TokenStream> {
    match fields {
        syn::Fields::Named(fields_named) => fields_named
            .named
            .iter()
            .map(|field| {
                let field_ident = &field.ident;
                let type_name = &field.ty;
                quote! {
                    #field_ident: multiversx_sc::types::ManagedVecItem::from_byte_reader(|bytes| {
                        let next_index = index + <#type_name as multiversx_sc::types::ManagedVecItem>::PAYLOAD_SIZE;
                        bytes.copy_from_slice(&arr[index .. next_index]);
                        index = next_index;
                    }),
                }
            })
            .collect(),
        _ => {
            panic!("ManagedVecItem only supports named fields")
        }
    }
}

fn generate_to_byte_writer_snippets(fields: &syn::Fields) -> Vec<proc_macro2::TokenStream> {
    match fields {
        syn::Fields::Named(fields_named) => fields_named
            .named
            .iter()
            .map(|field| {
                let field_ident = &field.ident;
                let type_name = &field.ty;
                quote! {
                    multiversx_sc::types::ManagedVecItem::to_byte_writer(&self.#field_ident, |bytes| {
                        let next_index = index + <#type_name as multiversx_sc::types::ManagedVecItem>::PAYLOAD_SIZE;
                        arr[index .. next_index].copy_from_slice(bytes);
                        index = next_index;
                    });
                }
            })
            .collect(),
        _ => {
            panic!("ManagedVecItem only supports named fields")
        }
    }
}

fn generate_array_init_snippet(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let self_expr = if ast.generics.params.is_empty() {
        quote! { #name }
    } else {
        quote! { #name <multiversx_sc::api::uncallable::UncallableApi> }
    };
    quote! {
        const SELF_PAYLOAD_SIZE: usize = <#self_expr as multiversx_sc::types::ManagedVecItem>::PAYLOAD_SIZE;
        let mut arr: [u8; SELF_PAYLOAD_SIZE] = [0u8; SELF_PAYLOAD_SIZE];
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
            const PAYLOAD_SIZE: usize = 1;
            const SKIPS_RESERIALIZATION: bool = true;
            type Ref<'a> = Self;

            fn from_byte_reader<Reader: FnMut(&mut [u8])>(mut reader: Reader) -> Self {
                let mut arr: [u8; 1] = [0u8; 1];
                reader(&mut arr[..]);
                match arr[0] {
                    #(#reader_match_arms)*
                }
            }

            unsafe fn from_byte_reader_as_borrow<'a, Reader: FnMut(&mut [u8])>(reader: Reader) -> Self::Ref<'a> {
                Self::from_byte_reader(reader)
            }

            fn to_byte_writer<R, Writer: FnMut(&[u8]) -> R>(&self, mut writer: Writer) -> R {
                let mut arr: [u8; 1] = [0u8; 1];
                arr[0] = match self {
                    #(#writer_match_arms)*
                };
                writer(&arr[..])
            }
        }
    };
    gen.into()
}

fn struct_derive(data_struct: &syn::DataStruct, ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
    let payload_snippets = generate_payload_snippets(&data_struct.fields);
    let skips_reserialization_snippets =
        generate_skips_reserialization_snippets(&data_struct.fields);
    let from_byte_reader_snippets = generate_from_byte_reader_snippets(&data_struct.fields);
    let to_byte_writer_snippets = generate_to_byte_writer_snippets(&data_struct.fields);

    let array_init_snippet = generate_array_init_snippet(ast);

    let gen = quote! {
        impl #impl_generics multiversx_sc::types::ManagedVecItem for #name #ty_generics #where_clause {
            const PAYLOAD_SIZE: usize = #(#payload_snippets)+*;
            const SKIPS_RESERIALIZATION: bool = #(#skips_reserialization_snippets)&&*;
            type Ref<'a> = Self;

            fn from_byte_reader<Reader: FnMut(&mut [u8])>(mut reader: Reader) -> Self {
                #array_init_snippet
                reader(&mut arr[..]);
                let mut index = 0;

                #name {
                    #(#from_byte_reader_snippets)*
                }
            }

            unsafe fn from_byte_reader_as_borrow<'a, Reader: FnMut(&mut [u8])>(reader: Reader) -> Self::Ref<'a> {
                Self::from_byte_reader(reader)
            }

            fn to_byte_writer<R, Writer: FnMut(&[u8]) -> R>(&self, mut writer: Writer) -> R {
                #array_init_snippet
                let mut index = 0;

                #(#to_byte_writer_snippets)*

                writer(&arr[..])
            }
        }
    };
    gen.into()
}
