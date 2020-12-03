extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(NestedEncode)]
pub fn nested_encode_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_nested_encode_macro(&ast)
}

#[proc_macro_derive(TopEncode)]
pub fn top_encode_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_top_encode_macro(&ast)
}

#[proc_macro_derive(NestedDecode)]
pub fn nested_decode_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_nested_decode_macro(&ast)
}

#[proc_macro_derive(TopDecode)]
pub fn top_decode_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_top_decode_macro(&ast)
}

fn extract_field_names(data: &syn::Data) -> Vec<syn::Ident> {
    match data {
        syn::Data::Struct(s) => {
            match &s.fields {
                syn::Fields::Named(fields) => {
                    fields.named.iter().map(|f| {
                        f.clone().ident.unwrap()
                    }).collect()
                },
                syn::Fields::Unnamed(_) => Vec::new(),
                syn::Fields::Unit => panic!("unit not supported")
            }
        },
        syn::Data::Enum(e) => {
            e.variants.iter().map(|v| {
                if v.fields.len() > 0 {
                    panic!("only simple enums supported")
                }

                v.clone().ident
            }).collect()
        },
        syn::Data::Union(_) => panic!("unions not supported")
    }
}

fn extract_struct_field_types(data: &syn::Data) -> Vec<syn::Type> {
    match data {
        syn::Data::Struct(s) => {
            match &s.fields {
                syn::Fields::Named(fields) => {
                    fields.named.iter().map(|f| {
                        f.ty.clone()
                    }).collect()
                },
                syn::Fields::Unnamed(fields) => {
                    fields.unnamed.iter().map(|f| {
                        f.ty.clone()
                    }).collect()
                },
                syn::Fields::Unit => panic!("unit not supported")
            }
        },
        _ => panic!("only structs supported")
    }
}

fn extract_enum_field_types(data: &syn::Data) -> Vec<Vec<syn::Type>> {
    match data {
        syn::Data::Enum(e) => {
            e.variants.iter().map(|v| {
                let mut field_types = Vec::new();
                for field in &v.fields {
                    field_types.push(field.ty.clone());
                }

                field_types
            }).collect()
        },
        _ => panic!("only enums supported")
    }
}

// Nested

fn impl_nested_encode_macro(ast: &syn::DeriveInput) -> TokenStream {
    if let syn::Data::Struct(_) = &ast.data {
           
    }
    else {
        panic!("Only structs may implement nested encode!");
    }

    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
    let fields = extract_field_names(&ast.data);
    let gen;

    if fields.len() > 0 {
        gen = quote! {
            impl #impl_generics NestedEncode for #name #ty_generics #where_clause {
                fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
                    #(self.#fields.dep_encode(dest)?;)*

                    Ok(())
                }

                fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
                    &self,
                    dest: &mut O,
                    c: ExitCtx,
                    exit: fn(ExitCtx, EncodeError) -> !,
                ) {
                    #(self.#fields.dep_encode_or_exit(dest, c.clone(), exit);)*
                }
            }
        };
    }
    else {
        let total_fields = match &ast.data {
            syn::Data::Struct(s) => {
                match &s.fields {
                    syn::Fields::Unnamed(u) => u.unnamed.len(),
                    _ => panic!("only structs with unnamed fields should reach here!")
                }
            },
            _ => panic!("only structs should reach here!")
        };
        let nameless_field_ident = (0..total_fields).map(syn::Index::from);
        let nameless_field_ident_again = nameless_field_ident.clone();

        gen = quote! {
            impl #impl_generics NestedEncode for #name #ty_generics #where_clause {
                fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
                    #(self.#nameless_field_ident.dep_encode(dest)?;)*

                    Ok(())
                }

                fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
                    &self,
                    dest: &mut O,
                    c: ExitCtx,
                    exit: fn(ExitCtx, EncodeError) -> !,
                ) {
                    #(self.#nameless_field_ident_again.dep_encode_or_exit(dest, c.clone(), exit);)*
                }
            }
        };
    }

    gen.into()
}

fn impl_nested_decode_macro(ast: &syn::DeriveInput) -> TokenStream {
    if let syn::Data::Struct(_) = &ast.data {
           
    }
    else {
        panic!("Only structs may implement nested decode!");
    }

    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
    let fields = extract_field_names(&ast.data);
    let types = extract_struct_field_types(&ast.data);
    let gen;
    
    if fields.len() > 0 {
        gen = quote! {
            impl #impl_generics NestedDecode for #name #ty_generics #where_clause {
                fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
                    Ok(#name {
                        #(#fields: <#types>::dep_decode(input)?,)*
                    })
                }
            
                fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
                    input: &mut I,
                    c: ExitCtx,
                    exit: fn(ExitCtx, DecodeError) -> !,
                ) -> Self {
                    #name {
                        #(#fields: <#types>::dep_decode_or_exit(input, c.clone(), exit),)*
                    }
                }
            }
        };
    }
    else {
        let total_fields = match &ast.data {
            syn::Data::Struct(s) => {
                match &s.fields {
                    syn::Fields::Unnamed(u) => u.unnamed.len(),
                    _ => panic!("only structs with unnamed fields should reach here!")
                }
            },
            _ => panic!("only structs should reach here!")
        };

        gen = quote! {
            impl #impl_generics NestedDecode for #name #ty_generics #where_clause {
                fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
                    Ok(#name (
                        #(<#types>::dep_decode(input)?),*
                    ))
                }
            
                fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
                    input: &mut I,
                    c: ExitCtx,
                    exit: fn(ExitCtx, DecodeError) -> !,
                ) -> Self {
                    #name (
                        #(<#types>::dep_decode_or_exit(input, c.clone(), exit)),*
                    )
                }
            }
        };
    }

    gen.into()
}

// Top

fn impl_top_encode_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = match &ast.data {
        syn::Data::Struct(_) => {
            let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();

            quote! {
                impl #impl_generics TopEncode for #name #ty_generics #where_clause {
                    #[inline]
                    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
                        top_encode_from_nested(self, output)
                    }
                
                    #[inline]
                    fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
                        &self,
                        output: O,
                        c: ExitCtx,
                        exit: fn(ExitCtx, EncodeError) -> !,
                    ) {
                        top_encode_from_nested_or_exit(self, output, c, exit);
                    }
                }
            }
        },
        syn::Data::Enum(_) => {
            let idents = extract_field_names(&ast.data);
            let value = 0..idents.len() as u8;
            let name_repeated = std::iter::repeat(name);

            quote! {
                impl #name {
                    pub fn to_u8(&self) -> u8 {
                        match self {
                            #(#name_repeated::#idents => #value,)*
                        }
                    }
                }

                impl TopEncode for #name {
                    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
                        self.to_u8().top_encode(output)
                    }
                
                    fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
                        &self,
                        output: O,
                        c: ExitCtx,
                        exit: fn(ExitCtx, EncodeError) -> !,
                    ) {
                        self.to_u8().top_encode_or_exit(output, c, exit)
                    }
                }
            }
        },
        syn::Data::Union(_) => panic!("Union not supported")
    };

    gen.into()
}

fn impl_top_decode_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = match &ast.data {
        syn::Data::Struct(_) => {
            let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();

            quote! {
                impl #impl_generics TopDecode for #name #ty_generics #where_clause {
                    fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
                        top_decode_from_nested(input)
                    }
                
                    fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
                        input: I,
                        c: ExitCtx,
                        exit: fn(ExitCtx, DecodeError) -> !,
                    ) -> Self {
                        top_decode_from_nested_or_exit(input, c, exit)
                    }
                }
            }
        },
        syn::Data::Enum(_) => {
            let idents = extract_field_names(&ast.data);
            let value = 0..idents.len() as u8;
            let value_again = value.clone();
            let name_repeated = std::iter::repeat(name);
            let name_repeated_again = name_repeated.clone();

            quote! {
                impl #name {
                    pub fn from_u8(v: u8) -> Result<Self, DecodeError> {
                        match v {
                            #(#value => core::result::Result::Ok(#name_repeated::#idents),)*
                            _ => core::result::Result::Err(DecodeError::INVALID_VALUE),
                        }
                    }
                }

                impl TopDecode for #name {
                    fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
                        #name::from_u8(u8::top_decode(input)?)
                    }
                
                    fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
                        input: I,
                        c: ExitCtx,
                        exit: fn(ExitCtx, DecodeError) -> !,
                    ) -> Self {
                        match u8::top_decode_or_exit(input, c.clone(), exit) {
                            #(#value_again => #name_repeated_again::#idents,)*
                            _ => exit(c, DecodeError::INVALID_VALUE),
                        }
                    }
                }
            }
        },
        syn::Data::Union(_) => panic!("Union not supported")
    };

    gen.into()
}
