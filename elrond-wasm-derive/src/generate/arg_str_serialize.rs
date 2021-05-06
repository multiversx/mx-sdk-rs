use crate::model::MethodArgument;

pub fn arg_serialize_push(
	arg: &MethodArgument,
	arg_accumulator: &proc_macro2::TokenStream,
	error_api_getter: &proc_macro2::TokenStream,

) -> proc_macro2::TokenStream {
	let pat = &arg.pat;
	let var_name = quote! { #pat };
	let arg_ty = &arg.ty;
	match arg_ty {
		syn::Type::Path(_) => {
			quote! {
				elrond_wasm::io::serialize_contract_call_arg(#var_name, #arg_accumulator, #error_api_getter);
			}
		},
		syn::Type::Reference(type_reference) => {
			if type_reference.mutability.is_some() {
				panic!("Mutable references not supported as contract method arguments");
			}
			quote! {
				elrond_wasm::io::serialize_contract_call_arg(#var_name, #arg_accumulator, #error_api_getter);
			}
		},
		other_arg => panic!(
			"Unsupported argument type: {:?}, neither path nor reference",
			other_arg
		),
	}
}
