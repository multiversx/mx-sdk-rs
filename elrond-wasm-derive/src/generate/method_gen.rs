use crate::model::{Method, MethodArgument};

pub fn arg_declarations(method_args: &[MethodArgument]) -> Vec<proc_macro2::TokenStream> {
    method_args
        .iter()
        .map(|arg| {
            let unprocessed_attributes = &arg.unprocessed_attributes;
            let pat = &arg.original_pat;
            let ty = &arg.ty;
            quote! { #(#unprocessed_attributes)* #pat : #ty }
        })
        .collect()
}

pub fn generate_sig(m: &Method) -> proc_macro2::TokenStream {
    let method_name = &m.name;
    let generics = &m.generics;
    let generics_where = &m.generics.where_clause;
    let arg_decl = arg_declarations(&m.method_args);
    let ret_tok = match &m.return_type {
        syn::ReturnType::Default => quote! {},
        syn::ReturnType::Type(r_arrow_token, ty) => quote! { #r_arrow_token #ty },
    };
    let result = quote! {
        #[allow(clippy::too_many_arguments)]
        #[allow(clippy::type_complexity)]
        fn #method_name #generics ( &self , #(#arg_decl),* ) #ret_tok #generics_where
    };
    result
}

pub fn generate_sig_with_attributes(m: &Method) -> proc_macro2::TokenStream {
    let unprocessed_attributes = &m.unprocessed_attributes;
    let msig = generate_sig(m);
    quote! {
        #(#unprocessed_attributes)*
        #msig
    }
}

pub fn generate_arg_call_name(arg: &MethodArgument) -> proc_macro2::TokenStream {
    let pat = &arg.pat;
    match &arg.ty {
        syn::Type::Reference(_) => quote! { &#pat },
        _ => quote! { #pat },
    }
}
