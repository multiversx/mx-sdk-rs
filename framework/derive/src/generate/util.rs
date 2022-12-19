use proc_macro2::Span;

macro_rules! format_ident {
    ($ident:expr, $fstr:expr) => {
        syn::Ident::new(&format!($fstr, $ident), $ident.span())
    };
}

pub fn generate_call_method_name(method_ident: &proc_macro2::Ident) -> proc_macro2::Ident {
    format_ident!(method_ident, "call_{}")
}

pub fn generate_endpoints_mod_alias(index: usize) -> proc_macro2::Ident {
    syn::Ident::new(&format!("__endpoints_{index}__"), Span::call_site())
}

pub fn generate_proxy_type_generic(index: usize) -> proc_macro2::Ident {
    syn::Ident::new(&format!("Arg{index}"), Span::call_site())
}

pub fn array_literal(bytes: &[u8]) -> proc_macro2::TokenStream {
    quote! { [ #(#bytes),* ] }
}

pub fn byte_slice_literal(bytes: &[u8]) -> proc_macro2::TokenStream {
    let arr_lit = array_literal(bytes);
    quote! { &#arr_lit[..] }
}

pub fn byte_str_literal(bytes: &[u8]) -> proc_macro2::TokenStream {
    let lit = proc_macro2::Literal::byte_string(bytes);
    quote! { #lit }
}

pub fn byte_str_slice_literal(bytes: &[u8]) -> proc_macro2::TokenStream {
    let lit = byte_str_literal(bytes);
    quote! { &#lit[..] }
}

#[allow(unused)]
pub fn ident_str_literal(ident: &syn::Ident) -> proc_macro2::TokenStream {
    byte_str_slice_literal(ident.to_string().as_bytes())
}

pub fn pat_string(pat: &syn::Pat) -> String {
    quote::ToTokens::to_token_stream(pat).to_string()
}

#[allow(unused)]
pub fn pat_literal(pat: &syn::Pat) -> proc_macro2::TokenStream {
    let pat_str = pat_string(pat);
    byte_str_slice_literal(pat_str.as_bytes())
}

#[allow(unused)]
pub fn arg_id_literal(pat: &syn::Pat) -> proc_macro2::TokenStream {
    let arg_name_literal = pat_literal(pat);
    quote! { ArgId::from(#arg_name_literal) }
}

/// Goes recursively through all generics (and nested generics)
/// and removes lifetime identifiers.
/// This is useful when generating static associated function trait calls.
pub fn clear_all_type_lifetimes(ty: &mut syn::Type) {
    match ty {
        syn::Type::Reference(r) => {
            r.lifetime = None;
        },
        syn::Type::Path(type_path) => {
            type_path.path.segments.iter_mut().for_each(|path_segm| {
                if let syn::PathArguments::AngleBracketed(angle_backeted) = &mut path_segm.arguments
                {
                    angle_backeted.args.iter_mut().for_each(|gen_arg| {
                        if let syn::GenericArgument::Type(gen_ty) = &mut *gen_arg {
                            clear_all_type_lifetimes(gen_ty);
                        }
                    });
                }
            });
        },
        _ => {},
    }
}
