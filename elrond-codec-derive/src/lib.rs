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
    let name = &ast.ident;
    let idents = extract_field_names(&ast.data);
    let gen = match &ast.data {
        syn::Data::Struct(_) => {
            let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();

            if idents.len() > 0 {
                quote! {
                    impl #impl_generics NestedEncode for #name #ty_generics #where_clause {
                        fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
                            #(self.#idents.dep_encode(dest)?;)*
        
                            Ok(())
                        }
        
                        fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
                            &self,
                            dest: &mut O,
                            c: ExitCtx,
                            exit: fn(ExitCtx, EncodeError) -> !,
                        ) {
                            #(self.#idents.dep_encode_or_exit(dest, c.clone(), exit);)*
                        }
                    }
                }
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
        
                quote! {
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
                }
            }
        },
        syn::Data::Enum(_) => {
            let types = extract_enum_field_types(&ast.data);
            let value: Vec<u8> = (0..idents.len() as u8).collect();
            let mut enum_encode_snippets = Vec::new();
            let mut enum_encode_or_exit_snippets = Vec::new();

            for i in 0..types.len() {
                let type_list = &types[i];
                let ident = &idents[i];
                let val = &value[i];
        
                if type_list.is_empty() {
                    enum_encode_snippets.push(quote! {
                        #name::#ident => #val.dep_encode(dest)?,
                    });

                    enum_encode_or_exit_snippets.push(quote! {
                        #name::#ident => #val.dep_encode_or_exit(dest, c.clone(), exit),
                    });
                }
                else if type_list.len() == 1 {
                    let local_var_ident = syn::Ident::new("_var_enum_local_ident",
                        proc_macro2::Span::call_site()); 

                    enum_encode_snippets.push(quote! {
                        #name::#ident(#local_var_ident) => {
                            #val.dep_encode(dest)?;
                            #local_var_ident.dep_encode(dest)?;
                        },
                    });

                    enum_encode_or_exit_snippets.push(quote! {
                        #name::#ident(#local_var_ident) => {
                            #val.dep_encode_or_exit(dest, c.clone(), exit);
                            #local_var_ident.dep_encode_or_exit(dest, c.clone(), exit);
                        },
                    });
                }
                else {
                    panic!("Only enums with one or less fields supported at the moment!");
                }
            }

            quote! {
                impl NestedEncode for #name {
                    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
                        match self {
                            #(#enum_encode_snippets)*
                        };
                        Ok(())
                    }
                
                    fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
                        &self,
                        dest: &mut O,
                        c: ExitCtx,
                        exit: fn(ExitCtx, EncodeError) -> !,
                    ) {
                        match self {
                            #(#enum_encode_or_exit_snippets)*
                        };
                    }
                }
            }
        },
        syn::Data::Union(_) => panic!("Union not supported!")
    };

    gen.into()
}

fn impl_nested_decode_macro(ast: &syn::DeriveInput) -> TokenStream {
    if let syn::Data::Union(_) = &ast.data {
        panic!("Union not supported!");
    }

    let name = &ast.ident;
    let idents = extract_field_names(&ast.data);
    let gen = match &ast.data {
        syn::Data::Struct(_) => {
            let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
            let types = extract_struct_field_types(&ast.data);

            if idents.len() > 0 {
                quote! {
                    impl #impl_generics NestedDecode for #name #ty_generics #where_clause {
                        fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
                            Ok(#name {
                                #(#idents: <#types>::dep_decode(input)?,)*
                            })
                        }
                    
                        fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
                            input: &mut I,
                            c: ExitCtx,
                            exit: fn(ExitCtx, DecodeError) -> !,
                        ) -> Self {
                            #name {
                                #(#idents: <#types>::dep_decode_or_exit(input, c.clone(), exit),)*
                            }
                        }
                    }
                }
            }
            else {
                quote! {
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
                }
            }
        },
        syn::Data::Enum(_) => {
            let types = extract_enum_field_types(&ast.data);
            let value: Vec<u8> = (0..idents.len() as u8).collect();
            let mut enum_decode_snippets = Vec::new();
            let mut enum_decode_or_exit_snippets = Vec::new();

            for i in 0..types.len() {
                let type_list = &types[i];
                let ident = &idents[i];
                let val = &value[i];
        
                if type_list.is_empty() {
                    enum_decode_snippets.push(quote! {
                        #val => Some(#name::#ident),
                    });

                    enum_decode_or_exit_snippets.push(quote! {
                        #val => Some(#name::#ident),
                    });
                }
                else if type_list.len() == 1 {
                    let var_type = &type_list[i];

                    enum_decode_snippets.push(quote! {
                        #val => Some(#name::#ident(#var_type::dep_decode(input)?)),
                    });

                    enum_decode_or_exit_snippets.push(quote! {
                        #val => Some(#name::#ident(#var_type::dep_decode_or_exit(input, c.clone(), exit))),
                    });
                }
                else {
                    panic!("Only enums with one or less fields supported at the moment!");
                }
            }

            quote! {
                impl NestedDecode for #name {
                    fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
                        let return_value = match u8::dep_decode(input)? {
                            #(#enum_decode_snippets)*
                            _ => None
                        };

                        match return_value {
                            Some(r) => Ok(r),
                            None => Err(DecodeError::INVALID_VALUE)
                        }
                    }
                
                    fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
                        input: &mut I,
                        c: ExitCtx,
                        exit: fn(ExitCtx, DecodeError) -> !,
                    ) -> Self {
                        let return_value = match u8::dep_decode_or_exit(input, c.clone(), exit) {
                            #(#enum_decode_or_exit_snippets)*
                            _ => None
                        };

                        match return_value {
                            Some(r) => r,
                            None => exit(c, DecodeError::INVALID_VALUE)
                        }
                    }
                }
            }
        },
        syn::Data::Union(_) => panic!("Union not supported!")
    };

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
            if is_simple_enum(&ast.data) {
                let idents = extract_field_names(&ast.data);
                let value: Vec<u8> = (0..idents.len() as u8).collect();
                let name_repeated = std::iter::repeat(name);
                let name_repeated_again = name_repeated.clone();

                quote! {
                    impl TopEncode for #name {
                        fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
                            //self.to_u8().top_encode(output)
                            match self {
                                #(#name_repeated::#idents => #value.top_encode(output),)*
                            }
                        }
                    
                        fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
                            &self,
                            output: O,
                            c: ExitCtx,
                            exit: fn(ExitCtx, EncodeError) -> !,
                        ) {
                            match self {
                                #(#name_repeated_again::#idents => #value.top_encode_or_exit(output, c, exit),)*
                            }
                        }
                    }
                }
            }
            else {
                panic!("Only simple enums can have top encode!")
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
            if is_simple_enum(&ast.data) {
                let idents = extract_field_names(&ast.data);
                let value = 0..idents.len() as u8;
                let value_again = value.clone();
                let name_repeated = std::iter::repeat(name);
                let name_repeated_again = name_repeated.clone();

                quote! {
                    impl TopDecode for #name {
                        fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
                            match u8::top_decode(input)? {
                                #(#value => core::result::Result::Ok(#name_repeated::#idents),)*
                                _ => core::result::Result::Err(DecodeError::INVALID_VALUE),
                            }
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
            }
            else {
                panic!("Only simple enums can have top decode!")
            }
        },
        syn::Data::Union(_) => panic!("Union not supported")
    };

    gen.into()
}

fn is_simple_enum(data: &syn::Data) -> bool {
    let types = extract_enum_field_types(data);
    
    for type_list in &types {
        if type_list.len() > 0 {
            return false;
        }
    }

    return true;
}
