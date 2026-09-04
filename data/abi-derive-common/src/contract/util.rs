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
