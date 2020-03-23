
macro_rules! format_ident {
    ($ident:expr, $fstr:expr) => {
        syn::Ident::new(&format!($fstr, $ident), $ident.span())
    };
}

pub fn generate_call_method_name(method_ident: &proc_macro2::Ident) -> proc_macro2::Ident {
    format_ident!(method_ident, "call_{}")
}

pub fn generate_callable_interface_impl_struct_name(trait_ident: &proc_macro2::Ident) -> proc_macro2::Ident {
    format_ident!(trait_ident, "{}Impl")
}

pub fn extract_struct_name(args: syn::AttributeArgs) -> syn::Path {
    if args.len() != 1 {
        panic!("Exactly one argument expected in contract annotation, specifying the implementation struct name.");
    }

    if let syn::NestedMeta::Meta(syn::Meta::Path(path)) = args.get(0).unwrap() {
        path.clone()
    } else {
        panic!("Malformed contract implementation struct name")
    }
}

pub fn extract_methods(contract_trait: &syn::ItemTrait) -> Vec<syn::TraitItemMethod> {
    contract_trait
        .items
        .iter()
        .filter_map(|itm| match itm {
            syn::TraitItem::Method(m) => {
                let msig = &m.sig;
                let bad_self_ref = format!(
                    "ABI function `{}` must have `&self` as its first argument.",
                    msig.ident.to_string()
                );
                match msig.inputs[0] {
                    syn::FnArg::Receiver(ref selfref) => {
                        if !selfref.mutability.is_none() {
                            panic!(bad_self_ref)
                        }
                    }
                    _ => panic!(bad_self_ref),
                }

                Some(m.clone())
            }
            _ => None,
        }).collect()
}

pub fn array_literal(bytes: &[u8]) -> proc_macro2::TokenStream {
    quote! { [ #(#bytes),* ] }
}
