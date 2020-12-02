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
                }
                _ => panic!("only named fields supported")
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

fn extract_field_types(data: &syn::Data) -> Vec<syn::Type> {
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

fn impl_nested_encode_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let fields = extract_field_names(&ast.data);

    let gen = quote! {
        impl NestedEncode for #name {
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

    gen.into()
}

fn impl_top_encode_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let gen = quote! {
        impl TopEncode for #name {
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
    };

    gen.into()
}

fn impl_nested_decode_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let fields = extract_field_names(&ast.data);
    let types = extract_field_types(&ast.data);

    let gen = quote! {
        impl NestedDecode for #name {
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

    gen.into()
}

fn impl_top_decode_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let gen = quote! {
        impl TopDecode for #name {
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
    };

    gen.into()
}
