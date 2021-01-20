macro_rules! format_ident {
	($ident:expr, $fstr:expr) => {
		syn::Ident::new(&format!($fstr, $ident), $ident.span())
	};
}

pub fn generate_call_method_name(method_ident: &proc_macro2::Ident) -> proc_macro2::Ident {
	format_ident!(method_ident, "call_{}")
}

pub fn generate_callable_interface_impl_struct_name(
	trait_ident: &proc_macro2::Ident,
) -> proc_macro2::Ident {
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
						if selfref.mutability.is_some() {
							panic!(bad_self_ref)
						}
					},
					_ => panic!(bad_self_ref),
				}

				Some(m.clone())
			},
			_ => None,
		})
		.collect()
}

pub fn array_literal(bytes: &[u8]) -> proc_macro2::TokenStream {
	quote! { [ #(#bytes),* ] }
}

pub fn byte_slice_literal(bytes: &[u8]) -> proc_macro2::TokenStream {
	let arr_lit = array_literal(bytes);
	quote! { &#arr_lit[..] }
}

pub fn pat_literal(pat: &syn::Pat) -> proc_macro2::TokenStream {
	let pat_str = quote::ToTokens::to_token_stream(pat).to_string();
	byte_slice_literal(pat_str.as_bytes())
}

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

pub fn generic_type_single_arg_segment(
	type_name: &str,
	parent_path_segment: &syn::PathSegment,
) -> syn::PathSegment {
	match &parent_path_segment.arguments {
		syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
			args, ..
		}) => {
			if args.len() != 1 {
				panic!(
					"{} type must have exactly 1 generic type argument",
					type_name
				);
			}
			if let syn::GenericArgument::Type(vec_type) = args.first().unwrap() {
				match vec_type {
					syn::Type::Path(type_path) => type_path.path.segments.last().unwrap().clone(),
					other_type => panic!(
						"Unsupported {} generic type: {:?}, not a path",
						type_name, other_type
					),
				}
			} else {
				panic!("{} type arguments must be types", type_name)
			}
		},
		_ => panic!("{} angle brackets expected", type_name),
	}
}
