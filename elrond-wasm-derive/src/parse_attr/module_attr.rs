use super::attr_names::*;
use super::util::*;

#[derive(Clone, Debug)]
pub struct ModuleAttribute {
	pub arg: proc_macro2::TokenTree,
}

impl ModuleAttribute {
	pub fn parse(m: &syn::TraitItemMethod) -> Option<ModuleAttribute> {
		match find_attr_with_one_opt_token_tree_arg(m, ATTR_MODULE) {
			None => None,
			Some(Some(arg)) => Some(ModuleAttribute { arg }),
			Some(_) => panic!("module name required"),
		}
	}
}
