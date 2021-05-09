use crate::model::Method;

pub fn generate_module_getter_impl(
	_m: &Method,
	_impl_path: &proc_macro2::TokenTree,
) -> proc_macro2::TokenStream {
	panic!("Module getters are no longer allowed. Use supertraits to import modules.")
}
