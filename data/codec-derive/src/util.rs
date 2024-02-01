use quote::quote;
use syn::{punctuated::Punctuated, token::Comma, Variant};

pub struct ExplicitDiscriminant {
    pub variant_index: usize,
    pub value: u8,
}

pub fn is_fieldless_enum(data_enum: &syn::DataEnum) -> bool {
    data_enum
        .variants
        .iter()
        .all(|variant| variant.fields.is_empty())
}

pub fn self_field_expr(index: usize, field: &syn::Field) -> proc_macro2::TokenStream {
    if let Some(ident) = &field.ident {
        quote! {
            self.#ident
        }
    } else {
        let index_lit = proc_macro2::Literal::usize_unsuffixed(index);
        quote! {
            self.#index_lit
        }
    }
}

pub fn local_variable_for_field(index: usize, field: &syn::Field) -> proc_macro2::TokenStream {
    if let Some(ident) = &field.ident {
        quote! {
            #ident
        }
    } else {
        let local_var_name = format!("unnamed_{index}");
        let local_var_ident = syn::Ident::new(&local_var_name, proc_macro2::Span::call_site());
        quote! {
            #local_var_ident
        }
    }
}

pub fn fields_snippets<F>(fields: &syn::Fields, field_mapper: F) -> Vec<proc_macro2::TokenStream>
where
    F: Fn(usize, &syn::Field) -> proc_macro2::TokenStream,
{
    match fields {
        syn::Fields::Named(fields_named) => fields_named
            .named
            .iter()
            .enumerate()
            .map(|(index, field)| field_mapper(index, field))
            .collect(),
        syn::Fields::Unnamed(fields_unnamed) => fields_unnamed
            .unnamed
            .iter()
            .enumerate()
            .map(|(index, field)| field_mapper(index, field))
            .collect(),
        syn::Fields::Unit => Vec::new(),
    }
}

pub fn fields_decl_syntax<F>(fields: &syn::Fields, field_mapper: F) -> proc_macro2::TokenStream
where
    F: Fn(usize, &syn::Field) -> proc_macro2::TokenStream,
{
    match fields {
        syn::Fields::Named(fields_named) => {
            let local_variables: Vec<proc_macro2::TokenStream> = fields_named
                .named
                .iter()
                .enumerate()
                .map(|(index, field)| field_mapper(index, field))
                .collect();
            quote! {
                { #(#local_variables),* }
            }
        },
        syn::Fields::Unnamed(fields_unnamed) => {
            let local_variables: Vec<proc_macro2::TokenStream> = fields_unnamed
                .unnamed
                .iter()
                .enumerate()
                .map(|(index, field)| field_mapper(index, field))
                .collect();
            quote! {
                ( #(#local_variables),* )
            }
        },
        syn::Fields::Unit => quote! {},
    }
}

pub fn validate_enum_variants(variants: &Punctuated<Variant, Comma>) {
    assert!(
        variants.len() <= 256,
        "enums with more than 256 variants not supported"
    );
}

pub fn get_discriminant(
    variant_index: usize,
    variant: &syn::Variant,
    previous_disc: &mut Vec<ExplicitDiscriminant>,
) -> proc_macro2::TokenStream {
    // if it has explicit discriminant
    if let Some((_, syn::Expr::Lit(expr))) = &variant.discriminant {
        let lit = match &expr.lit {
            syn::Lit::Int(val) => {
                let value = val.base10_parse().unwrap_or_else(|_| {
                    panic!("Can not unwrap int value from explicit discriminant")
                });
                previous_disc.push(ExplicitDiscriminant {
                    variant_index,
                    value,
                });
                value
            },
            _ => panic!("Only integer values as discriminants"), // theoretically covered by the compiler
        };
        return quote! { #lit};
    }

    // if no explicit discriminant, check previous discriminants
    // get previous explicit + 1 if there has been any explicit before
    let next_value = match previous_disc.last() {
        //there are previous explicit discriminants
        Some(ExplicitDiscriminant {
            variant_index: prev_index,
            value: prev_value,
        }) if *prev_index < variant_index - 1 => prev_value + (variant_index - prev_index) as u8,
        Some(ExplicitDiscriminant {
            variant_index: _,
            value: prev_value,
        }) => prev_value + 1,

        // vec is empty, return index
        None => variant_index as u8,
    };

    quote! { #next_value}
}
