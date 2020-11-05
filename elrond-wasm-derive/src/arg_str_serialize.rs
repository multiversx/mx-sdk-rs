use super::arg_def::*;

pub fn arg_serialize_push(
	arg: &MethodArg,
	arg_accumulator: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
	let pat = &arg.pat;
	let var_name = quote! { #pat };
	let arg_ty = &arg.ty;
	match arg_ty {
		syn::Type::Path(_) => {
			quote! {
				if let Result::Err(sc_err) = AsyncCallArg::push_async_arg(&#var_name, &mut #arg_accumulator) {
					self.api.signal_error(sc_err.as_bytes());
				}
			}
		},
		syn::Type::Reference(type_reference) => {
			if type_reference.mutability.is_some() {
				panic!("Mutable references not supported as contract method arguments");
			}
			quote! {
				if let Result::Err(sc_err) = AsyncCallArg::push_async_arg(#var_name, &mut #arg_accumulator) {
					self.api.signal_error(sc_err.as_bytes());
				}
			}
		},
		other_arg => panic!(
			"Unsupported argument type: {:?}, neither path nor reference",
			other_arg
		),
	}
}

pub fn arg_serialize_push_multi(
	arg: &MethodArg,
	arg_accumulator: &proc_macro2::TokenStream,
	expected_count_expr: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
	let pat = &arg.pat;
	let var_name = quote! { #pat };
	let arg_ty = &arg.ty;
	match arg_ty {
		syn::Type::Path(_) => {
			quote! {
				if let Result::Err(sc_err) = AsyncCallArg::push_async_arg_exact(&#var_name, &mut #arg_accumulator, #expected_count_expr) {
					self.api.signal_error(sc_err.as_bytes());
				}
			}
		},
		syn::Type::Reference(type_reference) => {
			if type_reference.mutability.is_some() {
				panic!("Mutable references not supported as contract method arguments");
			}
			quote! {
				if let Result::Err(sc_err) = AsyncCallArg::push_async_arg_exact(#var_name, &mut #arg_accumulator, #expected_count_expr) {
					self.api.signal_error(sc_err.as_bytes());
				}
			}
		},
		other_arg => panic!(
			"Unsupported argument type: {:?}, neither path nor reference",
			other_arg
		),
	}
}
