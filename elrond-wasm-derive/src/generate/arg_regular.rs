use super::util::*;
use crate::model::MethodArgument;

pub fn generate_load_single_arg(
	arg: &MethodArgument,
	arg_index_expr: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
	let arg_ty = &arg.ty;
	let arg_name_expr = arg_id_literal(&arg.pat);
	match &arg.ty {
		syn::Type::Reference(type_reference) => {
			if type_reference.mutability.is_some() {
				panic!("Mutable references not supported as contract method arguments");
			}
			if let syn::Type::Slice(slice_type) = &*type_reference.elem {
				// deserialize as boxed slice, so we have an owned object that we can reference
				let slice_elem = &slice_type.elem;
				quote! {
					elrond_wasm::load_single_arg::<Self::ArgumentApi, Box<[#slice_elem]>>(self.argument_api(), #arg_index_expr, #arg_name_expr)
				}
			} else {
				// deserialize as owned object, so we can then have a reference to it
				let referenced_type = &*type_reference.elem;
				if let syn::Type::Path(syn::TypePath { path, .. }) = referenced_type {
					if let Some(ident) = path.get_ident() {
						if *ident == "str" {
							// TODO: generalize for all unsized types using Box
							return quote! {
								elrond_wasm::load_single_arg::<Self::ArgumentApi, Box<str>>(self.argument_api(), #arg_index_expr, #arg_name_expr)
							};
						}
					}
				}

				quote! {
					elrond_wasm::load_single_arg::<Self::ArgumentApi, #referenced_type>(self.argument_api(), #arg_index_expr, #arg_name_expr)
				}
			}
		},
		_ => {
			quote! {
				elrond_wasm::load_single_arg::<Self::ArgumentApi, #arg_ty>(self.argument_api(), #arg_index_expr, #arg_name_expr)
			}
		},
	}
}

pub fn generate_load_dyn_arg(
	arg: &MethodArgument,
	loader_expr: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
	let pat = &arg.pat;
	let arg_ty = &arg.ty;
	let arg_name_expr = arg_id_literal(pat);
	match &arg.ty {
		syn::Type::Reference(type_reference) => {
			if type_reference.mutability.is_some() {
				panic!("Mutable references not supported as contract method arguments");
			}
			let referenced_type = &*type_reference.elem;
			quote! {
				let #pat: & #referenced_type = &elrond_wasm::load_dyn_arg(#loader_expr, #arg_name_expr);
			}
		},
		_ => {
			quote! {
				let #pat: #arg_ty = elrond_wasm::load_dyn_arg(#loader_expr, #arg_name_expr);
			}
		},
	}
}
