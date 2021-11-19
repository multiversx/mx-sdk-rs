use proc_macro::TokenStream;
use quote::quote;

/// Gets the first generic of the main type, for more accurate replacements in the field types.
fn extract_first_generic_ident(generics: &syn::Generics) -> Option<syn::Ident> {
    if let Some(syn::GenericParam::Type(type_param)) = generics.params.first() {
        return Some(type_param.ident.clone());
    }

    None
}

fn type_matches_ident(ty: &syn::Type, ident: &syn::Ident) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(first_segment) = type_path.path.segments.first() {
            if &first_segment.ident == ident {
                return true;
            }
        }
    }
    false
}

/// Replaces every generic with `<elrond_wasm::api::uncallable::UncallableApi>` in a type, indiscriminately.
/// A bit of a hack, but it works.
fn type_generic_replace_with_uncallable(
    parent_generics: &syn::Generics,
    type_name: &syn::Type,
) -> syn::Type {
    let mut replaced = type_name.clone();
    if let Some(parent_first_generic) = extract_first_generic_ident(parent_generics) {
        if let syn::Type::Path(path) = &mut replaced {
            for segment in path.path.segments.iter_mut() {
                if let syn::PathArguments::AngleBracketed(args) = &mut segment.arguments {
                    for arg in args.args.iter_mut() {
                        if let syn::GenericArgument::Type(generic_type) = arg {
                            if type_matches_ident(&*generic_type, &parent_first_generic) {
                                *generic_type =
                                    syn::parse_str("elrond_wasm::api::uncallable::UncallableApi")
                                        .unwrap();
                            }
                        }
                    }
                }
            }
        }
    }
    replaced
}

fn type_payload_size(
    parent_generics: &syn::Generics,
    type_name: &syn::Type,
) -> proc_macro2::TokenStream {
    let type_name_replaced = type_generic_replace_with_uncallable(parent_generics, type_name);
    quote! {
        <#type_name_replaced as elrond_wasm::types::ManagedVecItem<elrond_wasm::api::uncallable::UncallableApi>>::PAYLOAD_SIZE
    }
}

fn generate_payload_snippets(
    parent_generics: &syn::Generics,
    fields: &syn::Fields,
) -> Vec<proc_macro2::TokenStream> {
    match fields {
        syn::Fields::Named(fields_named) => fields_named
            .named
            .iter()
            .map(|field| type_payload_size(parent_generics, &field.ty))
            .collect(),
        _ => {
            panic!("ManagedVecItem only supports named fields")
        },
    }
}

fn generate_skips_reserialization_snippets(
    parent_generics: &syn::Generics,
    fields: &syn::Fields,
) -> Vec<proc_macro2::TokenStream> {
    match fields {
        syn::Fields::Named(fields_named) => fields_named
            .named
            .iter()
            .map(|field| {
                let type_name = type_generic_replace_with_uncallable(parent_generics, &field.ty);
                quote! {
                    <#type_name as elrond_wasm::types::ManagedVecItem<elrond_wasm::api::uncallable::UncallableApi>>::SKIPS_RESERIALIZATION
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

fn generate_array_init_snippet(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let self_expr = if ast.generics.params.is_empty() {
        quote! { #name }
    } else {
        quote! { #name <elrond_wasm::api::uncallable::UncallableApi> }
    };
    quote! {
        const SELF_PAYLOAD_SIZE: usize = <#self_expr as elrond_wasm::types::ManagedVecItem<elrond_wasm::api::uncallable::UncallableApi>>::PAYLOAD_SIZE;
        let mut arr: [u8; SELF_PAYLOAD_SIZE] = [0u8; SELF_PAYLOAD_SIZE];
    }
}

pub fn managed_vec_item_derive(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (original_impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
    let impl_generics = if ast.generics.params.is_empty() {
        quote! { <M: elrond_wasm::api::ManagedTypeApi> }
    } else {
        quote! { #original_impl_generics }
    };

    let payload_snippets: Vec<proc_macro2::TokenStream>;
    let skips_reserialization_snippets: Vec<proc_macro2::TokenStream>;
    let from_byte_reader_snippets: Vec<proc_macro2::TokenStream>;
    let to_byte_writer_snippets: Vec<proc_macro2::TokenStream>;
    if let syn::Data::Struct(data_struct) = &ast.data {
        payload_snippets = generate_payload_snippets(&ast.generics, &data_struct.fields);
        skips_reserialization_snippets =
            generate_skips_reserialization_snippets(&ast.generics, &data_struct.fields);
        from_byte_reader_snippets = generate_from_byte_reader_snippets(&data_struct.fields);
        to_byte_writer_snippets = generate_to_byte_writer_snippets(&data_struct.fields);
    } else {
        panic!("ManagedVecItem can only be implemented for struct")
    }

    let array_init_snippet = generate_array_init_snippet(ast);

    let gen = quote! {
        impl #impl_generics elrond_wasm::types::ManagedVecItem<M> for #name #ty_generics #where_clause {
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
