use super::arg_def::*;
use super::util::*;

pub fn generate_load_single_arg(
	arg: &MethodArg,
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
					elrond_wasm::load_single_arg::<T, BigInt, BigUint, Box<[#slice_elem]>>(self.api.clone(), #arg_index_expr, #arg_name_expr)
				}
			} else {
				// deserialize as owned object, so we can then have a reference to it
				let referenced_type = &*type_reference.elem;
				if let syn::Type::Path(syn::TypePath { path, .. }) = referenced_type {
					if let Some(ident) = path.get_ident() {
						if ident.to_string() == "str" {
							// TODO: generalize for all unsized types using Box
							return quote! {
								elrond_wasm::load_single_arg::<T, BigInt, BigUint, Box<str>>(self.api.clone(), #arg_index_expr, #arg_name_expr)
							};
						}
					}
				}

				quote! {
					elrond_wasm::load_single_arg::<T, BigInt, BigUint, #referenced_type>(self.api.clone(), #arg_index_expr, #arg_name_expr)
				}
			}
		},
		_ => {
			quote! {
				elrond_wasm::load_single_arg::<T, BigInt, BigUint, #arg_ty>(self.api.clone(), #arg_index_expr, #arg_name_expr)
			}
		},
	}
}

pub fn generate_load_dyn_arg(
	arg: &MethodArg,
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

pub fn generate_load_dyn_multi_arg(
	arg: &MethodArg,
	loader_expr: &proc_macro2::TokenStream,
	num_expr: &proc_macro2::TokenStream,
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
				let #pat: & #referenced_type = &elrond_wasm::load_dyn_multi_arg(#loader_expr, #arg_name_expr, #num_expr);
			}
		},
		_ => {
			quote! {
				let #pat: #arg_ty = elrond_wasm::load_dyn_multi_arg(#loader_expr, #arg_name_expr, #num_expr);
			}
		},
	}
}
