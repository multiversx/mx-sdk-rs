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
				elrond_wasm::io::serialize_contract_call_arg(#var_name, #arg_accumulator, self.api.clone());
			}
		},
		syn::Type::Reference(type_reference) => {
			if type_reference.mutability.is_some() {
				panic!("Mutable references not supported as contract method arguments");
			}
			quote! {
				elrond_wasm::io::serialize_contract_call_arg(#var_name, #arg_accumulator, self.api.clone());
			}
		},
		other_arg => panic!(
			"Unsupported argument type: {:?}, neither path nor reference",
			other_arg
		),
	}
}
