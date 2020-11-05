use super::contract_gen_method::*;

pub fn generate_module_getter_impl(
	m: &Method,
	impl_path: &proc_macro2::TokenTree,
) -> proc_macro2::TokenStream {
	let msig = m.generate_sig();
	if !m.method_args.is_empty() {
		panic!("module getter cannot have arguments");
	}
	// TODO: check return type

	quote! {
		#msig {
			#impl_path::new(self.api.clone())
		}
	}
}
