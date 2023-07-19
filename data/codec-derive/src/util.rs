use quote::quote;

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
