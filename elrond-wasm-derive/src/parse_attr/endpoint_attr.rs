use super::attr_names::*;
use super::util::*;

pub fn is_callback_decl(m: &syn::TraitItemMethod) -> bool {
	has_attribute(&m.attrs, ATTR_CALLBACK_DECL)
}

pub fn is_callback_raw_decl(m: &syn::TraitItemMethod) -> bool {
	has_attribute(&m.attrs, ATTR_CALLBACK_RAW_DECL)
}

pub fn is_init(m: &syn::TraitItemMethod) -> bool {
	has_attribute(&m.attrs, ATTR_INIT)
}

pub fn is_var_args(pat: &syn::PatType) -> bool {
	has_attribute(&pat.attrs, ATTR_VAR_ARGS)
}

pub fn is_callback_arg(pat: &syn::PatType) -> bool {
	has_attribute(&pat.attrs, ATTR_CALLBACK_ARG)
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
			_ => panic!("unexpected endpoint argument tokens"),
		}
	}
}

#[derive(Clone, Debug)]
pub struct CallbackCallAttribute {
	pub arg: syn::Ident,
}

impl CallbackCallAttribute {
	pub fn parse(m: &syn::TraitItemMethod) -> Option<CallbackCallAttribute> {
		match find_attr_with_one_opt_token_tree_arg(m, ATTR_CALLBACK_CALL) {
			None => None,
			Some(Some(proc_macro2::TokenTree::Ident(ident))) => {
				Some(CallbackCallAttribute { arg: ident })
			},
			_ => panic!("single identifier expected as callback argument"),
		}
	}
}

#[derive(Clone, Debug)]
pub struct MultiAttribute {
	pub count_expr: proc_macro2::TokenStream,
}

impl MultiAttribute {
	pub fn parse(pat: &syn::PatType) -> Option<MultiAttribute> {
		let multi_attr = pat.attrs.iter().find(|attr| {
			if let Some(first_seg) = attr.path.segments.first() {
				first_seg.ident == ATTR_MULTI
			} else {
				false
			}
		});

		match multi_attr {
			None => None,
			Some(attr) => {
				let mut iter = attr.clone().tokens.into_iter();
				let count_expr: proc_macro2::TokenStream =
					match iter.next() {
						Some(count_expr_group) => {
							// some validation
							match &count_expr_group {
								proc_macro2::TokenTree::Group(group_data) => {
									match group_data.delimiter() {
									proc_macro2::Delimiter::Parenthesis | proc_macro2::Delimiter::Bracket => { /* ok */ },
									_ => panic!("paranetheses of brackets expected in #[multi] attribute"),
								}
								},
								_ => panic!("illegal argument in #[multi] attribute"),
							}

							// simply flatten to token stream and return
							quote! { #count_expr_group }
						},
						_ => panic!("callback argument expected"),
					};

				if iter.next().is_some() {
					panic!("too many tokens in payable attribute");
				}

				Some(MultiAttribute { count_expr })
			},
		}
	}
}
