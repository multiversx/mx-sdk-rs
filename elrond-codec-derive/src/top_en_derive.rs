use proc_macro::TokenStream;
use quote::quote;

use crate::{
    nested_en_derive::{dep_encode_or_exit_snippet, dep_encode_snippet},
    util::*,
};

pub fn variant_top_encode_snippets(
    name: &syn::Ident,
    data_enum: &syn::DataEnum,
) -> Vec<proc_macro2::TokenStream> {
    data_enum
        .variants
        .iter()
        .enumerate()
        .map(|(variant_index, variant)| {
            let variant_index_u8 = variant_index as u8;
            let variant_ident = &variant.ident;
            if variant.fields.is_empty() {
                // top-encode discriminant directly
                quote! {
                    #name::#variant_ident =>
                        elrond_codec::TopEncode::top_encode(&#variant_index_u8, output),
                }
            } else {
                // dep-encode to buffer first
                let local_var_declarations =
                    fields_decl_syntax(&variant.fields, local_variable_for_field);
                let variant_field_snippets = fields_snippets(&variant.fields, |index, field| {
                    dep_encode_snippet(&local_variable_for_field(index, field))
                });
                quote! {
                    #name::#variant_ident #local_var_declarations => {
                        let mut buffer = output.start_nested_encode();
                        let dest = &mut buffer;
                        elrond_codec::NestedEncode::dep_encode(&#variant_index_u8, dest)?;
                        #(#variant_field_snippets)*
                        output.finalize_nested_encode(buffer);
                        core::result::Result::Ok(())
                    },
                }
            }
        })
        .collect()
}

pub fn variant_top_encode_or_exit_snippets(
    name: &syn::Ident,
    data_enum: &syn::DataEnum,
) -> Vec<proc_macro2::TokenStream> {
    data_enum
		.variants
		.iter()
		.enumerate()
		.map(|(variant_index, variant)| {
			let variant_index_u8 = variant_index as u8;
			let variant_ident = &variant.ident;
			if variant.fields.is_empty() {
				// top-encode discriminant directly
				quote! {
					#name::#variant_ident =>
						elrond_codec::TopEncode::top_encode_or_exit(&#variant_index_u8, output, c.clone(), exit),
				}
			} else {
				// dep-encode to buffer first
				let local_var_declarations =
					fields_decl_syntax(&variant.fields, local_variable_for_field);
				let variant_field_snippets = fields_snippets(&variant.fields, |index, field| {
					dep_encode_or_exit_snippet(&local_variable_for_field(index, field))
				});
				quote! {
					#name::#variant_ident #local_var_declarations => {
						let mut buffer = output.start_nested_encode();
						let dest = &mut buffer;
						elrond_codec::NestedEncode::dep_encode_or_exit(&#variant_index_u8, dest, c.clone(), exit);
						#(#variant_field_snippets)*
						output.finalize_nested_encode(buffer);
					},
				}
			}
		})
		.collect()
}

/// Only returns the trait implementation method bodies, without the impl or method definitions.
fn top_encode_method_bodies(
    ast: &syn::DeriveInput,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let name = &ast.ident;
    match &ast.data {
        syn::Data::Struct(data_struct) => {
            let field_dep_encode_snippets = fields_snippets(&data_struct.fields, |index, field| {
                dep_encode_snippet(&self_field_expr(index, field))
            });
            let field_dep_encode_or_exit_snippets =
                fields_snippets(&data_struct.fields, |index, field| {
                    dep_encode_or_exit_snippet(&self_field_expr(index, field))
                });
            let top_encode_body = quote! {
                let mut buffer = output.start_nested_encode();
                let dest = &mut buffer;
                #(#field_dep_encode_snippets)*
                output.finalize_nested_encode(buffer);
                core::result::Result::Ok(())
            };
            let top_encode_or_exit_body = quote! {
                let mut buffer = output.start_nested_encode();
                let dest = &mut buffer;
                #(#field_dep_encode_or_exit_snippets)*
                output.finalize_nested_encode(buffer);
            };
            (top_encode_body, top_encode_or_exit_body)
        },
        syn::Data::Enum(data_enum) => {
            assert!(
                data_enum.variants.len() < 256,
                "enums with more than 256 variants not supported"
            );
            let variant_top_encode_snippets = variant_top_encode_snippets(name, data_enum);
            let variant_top_encode_or_exit_snippets =
                variant_top_encode_or_exit_snippets(name, data_enum);

            let top_encode_body = quote! {
                match self {
                    #(#variant_top_encode_snippets)*
                }
            };
            let top_encode_or_exit_body = quote! {
                match self {
                    #(#variant_top_encode_or_exit_snippets)*
                }
            };
            (top_encode_body, top_encode_or_exit_body)
        },
        syn::Data::Union(_) => panic!("Union not supported"),
    }
}

pub fn top_encode_impl(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
    let (top_encode_body, top_encode_or_exit_body) = top_encode_method_bodies(ast);

    let gen = quote! {
        impl #impl_generics elrond_codec::TopEncode for #name #ty_generics #where_clause {
            fn top_encode<O: elrond_codec::TopEncodeOutput>(&self, output: O) -> core::result::Result<(), elrond_codec::EncodeError> {
                #top_encode_body
            }

            fn top_encode_or_exit<O: elrond_codec::TopEncodeOutput, ExitCtx: Clone>(
                &self,
                output: O,
                c: ExitCtx,
                exit: fn(ExitCtx, elrond_codec::EncodeError) -> !,
            ) {
                #top_encode_or_exit_body
            }
        }
    };
    gen.into()
}

pub fn top_encode_or_default_impl(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
    let (top_encode_body, top_encode_or_exit_body) = top_encode_method_bodies(ast);

    let gen = quote! {
        impl #impl_generics elrond_codec::TopEncode for #name #ty_generics #where_clause {
            fn top_encode<O: elrond_codec::TopEncodeOutput>(&self, output: O) -> core::result::Result<(), elrond_codec::EncodeError> {
                if elrond_codec::EncodeDefault::is_default(self) {
                    output.set_slice_u8(&[]);
                    Ok(())
                } else {
                    #top_encode_body
                }
            }

            fn top_encode_or_exit<O: elrond_codec::TopEncodeOutput, ExitCtx: Clone>(
                &self,
                output: O,
                c: ExitCtx,
                exit: fn(ExitCtx, elrond_codec::EncodeError) -> !,
            ) {
                if elrond_codec::EncodeDefault::is_default(self) {
                    output.set_slice_u8(&[]);
                } else {
                    #top_encode_or_exit_body
                }
            }
        }
    };
    gen.into()
}
