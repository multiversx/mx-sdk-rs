use proc_macro::TokenStream;
use quote::quote;

fn type_payload_size(type_name: &syn::Type) -> proc_macro2::TokenStream {
    quote! {
        <#type_name as elrond_wasm::types::ManagedVecItem<elrond_wasm::api::uncallable::UncallableApi>>::PAYLOAD_SIZE
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
                    <#type_name as elrond_wasm::types::ManagedVecItem<M>>::SKIPS_RESERIALIZATION
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
                    #field_ident: elrond_wasm::types::ManagedVecItem::<M>::from_byte_reader(api.clone(), |bytes| {
                        let next_index = index + <#type_name as elrond_wasm::types::ManagedVecItem<M>>::PAYLOAD_SIZE;
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
                    elrond_wasm::types::ManagedVecItem::<M>::to_byte_writer(&self.#field_ident, |bytes| {
                        let next_index = index + <#type_name as elrond_wasm::types::ManagedVecItem<M>>::PAYLOAD_SIZE;
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

fn generate_array_init_snippet(fields: &syn::Fields) -> proc_macro2::TokenStream {
    let payload_snippets = generate_payload_snippets(fields);
    quote! {
        const SELF_PAYLOAD_SIZE: usize = #(#payload_snippets)+*;
        let mut arr: [u8; SELF_PAYLOAD_SIZE] = [0u8; SELF_PAYLOAD_SIZE];
    }
}

pub fn managed_vec_item_derive(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    // let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
    // let (top_encode_body, top_encode_or_exit_body) = top_encode_method_bodies(ast);

    let payload_snippets: Vec<proc_macro2::TokenStream>;
    let skips_reserialization_snippets: Vec<proc_macro2::TokenStream>;
    let from_byte_reader_snippets: Vec<proc_macro2::TokenStream>;
    let to_byte_writer_snippets: Vec<proc_macro2::TokenStream>;
    let array_init_snippet: proc_macro2::TokenStream;
    if let syn::Data::Struct(data_struct) = &ast.data {
        payload_snippets = generate_payload_snippets(&data_struct.fields);
        skips_reserialization_snippets =
            generate_skips_reserialization_snippets(&data_struct.fields);
        from_byte_reader_snippets = generate_from_byte_reader_snippets(&data_struct.fields);
        to_byte_writer_snippets = generate_to_byte_writer_snippets(&data_struct.fields);
        array_init_snippet = generate_array_init_snippet(&data_struct.fields);
    } else {
        panic!("ManagedVecItem can only be implemented for struct")
    }

    let gen = quote! {
        impl <M: elrond_wasm::api::ManagedTypeApi> elrond_wasm::types::ManagedVecItem<M> for #name {
            const PAYLOAD_SIZE: usize = #(#payload_snippets)+*;
            const SKIPS_RESERIALIZATION: bool = #(#skips_reserialization_snippets)&&*;

            fn from_byte_reader<Reader: FnMut(&mut [u8])>(api: M, mut reader: Reader) -> Self {
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
