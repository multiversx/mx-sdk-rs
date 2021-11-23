use proc_macro::TokenStream;
use quote::quote;

fn type_payload_size(type_name: &syn::Type) -> proc_macro2::TokenStream {
    quote! {
        <#type_name as elrond_wasm::types::ManagedVecItem>::PAYLOAD_SIZE
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
                    <#type_name as elrond_wasm::types::ManagedVecItem>::SKIPS_RESERIALIZATION
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
                    #field_ident: elrond_wasm::types::ManagedVecItem::from_byte_reader(|bytes| {
                        let next_index = index + <#type_name as elrond_wasm::types::ManagedVecItem>::PAYLOAD_SIZE;
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
                    elrond_wasm::types::ManagedVecItem::to_byte_writer(&self.#field_ident, |bytes| {
                        let next_index = index + <#type_name as elrond_wasm::types::ManagedVecItem>::PAYLOAD_SIZE;
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
        quote! { #name <elrond_wasm::api::uncallable::UncallableApi> }
    };
    quote! {
        const SELF_PAYLOAD_SIZE: usize = <#self_expr as elrond_wasm::types::ManagedVecItem>::PAYLOAD_SIZE;
        let mut arr: [u8; SELF_PAYLOAD_SIZE] = [0u8; SELF_PAYLOAD_SIZE];
    }
}

pub fn managed_vec_item_derive(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();

    let payload_snippets: Vec<proc_macro2::TokenStream>;
    let skips_reserialization_snippets: Vec<proc_macro2::TokenStream>;
    let from_byte_reader_snippets: Vec<proc_macro2::TokenStream>;
    let to_byte_writer_snippets: Vec<proc_macro2::TokenStream>;
    if let syn::Data::Struct(data_struct) = &ast.data {
        payload_snippets = generate_payload_snippets(&data_struct.fields);
        skips_reserialization_snippets =
            generate_skips_reserialization_snippets(&data_struct.fields);
        from_byte_reader_snippets = generate_from_byte_reader_snippets(&data_struct.fields);
        to_byte_writer_snippets = generate_to_byte_writer_snippets(&data_struct.fields);
    } else {
        panic!("ManagedVecItem can only be implemented for struct")
    }

    let array_init_snippet = generate_array_init_snippet(ast);

    let gen = quote! {
        impl #impl_generics elrond_wasm::types::ManagedVecItem for #name #ty_generics #where_clause {
            const PAYLOAD_SIZE: usize = #(#payload_snippets)+*;
            const SKIPS_RESERIALIZATION: bool = #(#skips_reserialization_snippets)&&*;

            fn from_byte_reader<Reader: FnMut(&mut [u8])>(mut reader: Reader) -> Self {
                #array_init_snippet
                reader(&mut arr[..]);
                let mut index = 0;

                #name {
                    #(#from_byte_reader_snippets)*
                }
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
