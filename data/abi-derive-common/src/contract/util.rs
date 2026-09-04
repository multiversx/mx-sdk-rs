/// Goes recursively through all generics (and nested generics)
/// and removes lifetime identifiers.
/// This is useful when generating static associated function trait calls.
pub fn clear_all_type_lifetimes(ty: &mut syn::Type) {
    match ty {
        syn::Type::Reference(r) => {
            r.lifetime = None;
        }
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
        }
        _ => {}
    }
}

/// Goes recursively through a type (and its nested generics) and replaces any occurrence of the
/// `Self::Api` path with `replacement`.
///
/// The framework's managed-type substitution (run before `#[multiversx_sc::contract]` parses its
/// input) rewrites bare `BigUint`, `ManagedAddress`, etc. into e.g.
/// `multiversx_sc::types::BigUint<Self::Api>`. That `Self::Api` only resolves inside a `Self:
/// ContractBase`-bound context (the contract trait itself); it does not resolve inside a
/// free-standing proxy struct's inherent impl blocks, which have no such `Self`. Since a type's
/// `TypeAbi::Abi` projection is API-erased by design (`BigUint<M>::Abi` is `BigUintAbi`
/// regardless of `M`), swapping in any concrete, arbitrary API type before taking `::Abi` is
/// always correct — the caller picks that placeholder (e.g. `multiversx_sc::api::uncallable::
/// UncallableApi` for the framework path; irrelevant, since never triggered, for the
/// framework-agnostic path where types never contain `Self` to begin with).
pub fn replace_self_api(ty: &mut syn::Type, replacement: &syn::Path) {
    if is_self_api_path(ty) {
        *ty = syn::Type::Path(syn::TypePath {
            qself: None,
            path: replacement.clone(),
        });
        return;
    }

    if let syn::Type::Path(type_path) = ty {
        type_path.path.segments.iter_mut().for_each(|path_segm| {
            if let syn::PathArguments::AngleBracketed(angle_backeted) = &mut path_segm.arguments {
                angle_backeted.args.iter_mut().for_each(|gen_arg| {
                    if let syn::GenericArgument::Type(gen_ty) = &mut *gen_arg {
                        replace_self_api(gen_ty, replacement);
                    }
                });
            }
        });
    }
}

fn is_self_api_path(ty: &syn::Type) -> bool {
    let syn::Type::Path(type_path) = ty else {
        return false;
    };
    if type_path.qself.is_some() {
        return false;
    }
    let idents: Vec<String> = type_path
        .path
        .segments
        .iter()
        .map(|seg| seg.ident.to_string())
        .collect();
    idents == ["Self", "Api"]
}
