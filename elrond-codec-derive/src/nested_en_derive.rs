use crate::util::*;
use proc_macro::TokenStream;
use quote::quote;

pub fn dep_encode_snippet(value: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    quote! {
        elrond_codec::NestedEncode::dep_encode(&#value, dest)?;
    }
}

pub fn dep_encode_or_exit_snippet(value: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    quote! {
        elrond_codec::NestedEncode::dep_encode_or_exit(&#value, dest, c.clone(), exit);
    }
}

fn variant_dep_encode_snippets(
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
            let local_var_declarations =
                fields_decl_syntax(&variant.fields, local_variable_for_field);
            let variant_field_snippets = fields_snippets(&variant.fields, |index, field| {
                dep_encode_snippet(&local_variable_for_field(index, field))
            });
            quote! {
                #name::#variant_ident #local_var_declarations => {
                    elrond_codec::NestedEncode::dep_encode(&#variant_index_u8, dest)?;
                    #(#variant_field_snippets)*
                },
            }
        })
        .collect()
}

fn variant_dep_encode_or_exit_snippets(
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
			let local_var_declarations =
				fields_decl_syntax(&variant.fields, local_variable_for_field);
			let variant_field_snippets = fields_snippets(&variant.fields, |index, field| {
				dep_encode_or_exit_snippet(&local_variable_for_field(index, field))
			});
			quote! {
				#name::#variant_ident #local_var_declarations => {
					elrond_codec::NestedEncode::dep_encode_or_exit(&#variant_index_u8, dest, c.clone(), exit);
					#(#variant_field_snippets)*
				},
			}
		})
		.collect()
}

pub fn nested_encode_impl(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
    let gen = match &ast.data {
        syn::Data::Struct(data_struct) => {
            let field_dep_encode_snippets = fields_snippets(&data_struct.fields, |index, field| {
                dep_encode_snippet(&self_field_expr(index, field))
            });
            let field_dep_encode_or_exit_snippets =
                fields_snippets(&data_struct.fields, |index, field| {
                    dep_encode_or_exit_snippet(&self_field_expr(index, field))
                });
            quote! {
                impl #impl_generics elrond_codec::NestedEncode for #name #ty_generics #where_clause {
                    fn dep_encode<O: elrond_codec::NestedEncodeOutput>(&self, dest: &mut O) -> core::result::Result<(), elrond_codec::EncodeError> {
                        #(#field_dep_encode_snippets)*
                        core::result::Result::Ok(())
                    }

                    fn dep_encode_or_exit<O: elrond_codec::NestedEncodeOutput, ExitCtx: Clone>(
                        &self,
                        dest: &mut O,
                        c: ExitCtx,
                        exit: fn(ExitCtx, elrond_codec::EncodeError) -> !,
                    ) {
                        #(#field_dep_encode_or_exit_snippets)*
                    }
                }
            }
        },
        syn::Data::Enum(data_enum) => {
            assert!(
                data_enum.variants.len() < 256,
                "enums with more than 256 variants not supported"
            );
            let variant_dep_encode_snippets = variant_dep_encode_snippets(name, data_enum);
            let variant_dep_encode_or_exit_snippets =
                variant_dep_encode_or_exit_snippets(name, data_enum);

            quote! {
                impl #impl_generics elrond_codec::NestedEncode for #name #ty_generics #where_clause {
                    fn dep_encode<O: elrond_codec::NestedEncodeOutput>(&self, dest: &mut O) -> core::result::Result<(), elrond_codec::EncodeError> {
                        match self {
                            #(#variant_dep_encode_snippets)*
                        };
                        core::result::Result::Ok(())
                    }

                    fn dep_encode_or_exit<O: elrond_codec::NestedEncodeOutput, ExitCtx: Clone>(
                        &self,
                        dest: &mut O,
                        c: ExitCtx,
                        exit: fn(ExitCtx, elrond_codec::EncodeError) -> !,
                    ) {
                        match self {
                            #(#variant_dep_encode_or_exit_snippets)*
                        };
                    }
                }
            }
        },
        syn::Data::Union(_) => panic!("Union not supported"),
    };

    gen.into()
}
