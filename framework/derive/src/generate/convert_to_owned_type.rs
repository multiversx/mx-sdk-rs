// use super::util::*;
// use crate::model::MethodArgument;

/// I now consider this a hack, a proper solution should be using traits somehow.
///
/// We basically change the type in the macro until then, to make things easier for now.
///
/// TODO: investigate something along the lines of IntoOwned.
pub fn convert_to_owned_type(ty: &syn::Type) -> proc_macro2::TokenStream {
    if let syn::Type::Reference(type_reference) = &ty {
        assert!(
            type_reference.mutability.is_none(),
            "Mutable references not supported as contract method arguments"
        );
        if let syn::Type::Slice(slice_type) = &*type_reference.elem {
            // deserialize as boxed slice, so we have an owned object that we can reference
            let slice_elem = &slice_type.elem;
            return quote! {
                multiversx_sc::types::Box<[#slice_elem]>
            };
        } else if let syn::Type::Path(syn::TypePath { path, .. }) = &*type_reference.elem {
            if let Some(ident) = path.get_ident() {
                if *ident == "str" {
                    // TODO: generalize for all unsized types using Box
                    return quote! {
                        multiversx_sc::types::Box<str>
                    };
                }
            }
        }

        let referenced_type = &*type_reference.elem;
        return quote! { #referenced_type };
    }

    quote! { #ty }
}
