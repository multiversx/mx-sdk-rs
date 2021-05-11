use super::attr_names::*;
use super::util::*;

pub fn is_init(m: &syn::TraitItemMethod) -> bool {
	has_attribute(&m.attrs, ATTR_INIT)
}

pub fn is_callback_raw_decl(m: &syn::TraitItemMethod) -> bool {
	has_attribute(&m.attrs, ATTR_CALLBACK_RAW_DECL)
}

pub fn is_proxy(m: &syn::TraitItemMethod) -> bool {
	has_attribute(&m.attrs, ATTR_PROXY)
}

pub fn is_var_args(pat: &syn::PatType) -> bool {
	has_attribute(&pat.attrs, ATTR_VAR_ARGS)
}

pub fn is_callback_result_arg(pat: &syn::PatType) -> bool {
	has_attribute(&pat.attrs, ATTR_CALLBACK_CALL_RESULT)
}

#[derive(Clone, Debug)]
pub struct EndpointAttribute {
	pub endpoint_name: Option<syn::Ident>,
}

impl EndpointAttribute {
	pub fn parse(m: &syn::TraitItemMethod) -> Option<EndpointAttribute> {
		match find_attr_with_one_opt_token_tree_arg(m, ATTR_ENDPOINT) {
			None => None,
			Some(Some(proc_macro2::TokenTree::Ident(ident))) => Some(EndpointAttribute {
				endpoint_name: Some(ident),
			}),
			Some(None) => Some(EndpointAttribute {
				endpoint_name: None,
			}),
			_ => panic!("unexpected endpoint argument tokens"),
		}
	}
}

#[derive(Clone, Debug)]
pub struct ViewAttribute {
	pub view_name: Option<syn::Ident>,
}

impl ViewAttribute {
	pub fn parse(m: &syn::TraitItemMethod) -> Option<ViewAttribute> {
		match find_attr_with_one_opt_token_tree_arg(m, ATTR_VIEW) {
			None => None,
			Some(Some(proc_macro2::TokenTree::Ident(ident))) => Some(ViewAttribute {
				view_name: Some(ident),
			}),
			Some(None) => Some(ViewAttribute { view_name: None }),
			_ => panic!("unexpected view argument tokens"),
		}
	}
}

#[derive(Clone, Debug)]
pub struct CallbackAttribute {
	pub callback_name: Option<syn::Ident>,
}

impl CallbackAttribute {
	pub fn parse(m: &syn::TraitItemMethod) -> Option<CallbackAttribute> {
		match find_attr_with_one_opt_token_tree_arg(m, ATTR_CALLBACK_DECL) {
			None => None,
			Some(Some(proc_macro2::TokenTree::Ident(ident))) => Some(CallbackAttribute {
				callback_name: Some(ident),
			}),
			Some(None) => Some(CallbackAttribute {
				callback_name: None,
			}),
			_ => panic!("unexpected endpoint argument tokens"),
		}
	}
}
