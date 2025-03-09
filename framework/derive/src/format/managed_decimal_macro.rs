pub fn extract_number_data(input: syn::LitStr) -> (u64, usize) {
    let value_str = input.value();

    let parts: Vec<&str> = value_str.split('.').collect();
    let raw_val = parts.join("");
    let raw_int = raw_val.parse::<u64>().expect("Invalid integer value");

    let decimals = if parts.len() > 1 { parts[1].len() } else { 0 };

    (raw_int, decimals)
}

pub fn extract_number_type(input: syn::LitStr) -> (u64, syn::Type) {
    let span = input.span();
    let (raw_int, decimals) = extract_number_data(input);
    let ty = typenum_type(decimals, span);
    (raw_int, ty)
}

/// Creates expressions like `typenum::U5`
fn typenum_type(decimals: usize, span: proc_macro2::Span) -> syn::Type {
    let mut segments = syn::punctuated::Punctuated::new();
    segments.push(syn::PathSegment {
        ident: syn::Ident::new("typenum", span),
        arguments: syn::PathArguments::None,
    });
    let type_name = format!("U{decimals}");
    segments.push(syn::PathSegment {
        ident: syn::Ident::new(&type_name, span),
        arguments: syn::PathArguments::None,
    });
    syn::Type::Path(syn::TypePath {
        qself: None,
        path: syn::Path {
            leading_colon: None,
            segments,
        },
    })
}
