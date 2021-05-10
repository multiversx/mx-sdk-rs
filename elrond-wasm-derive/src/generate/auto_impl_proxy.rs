use crate::{
	generate::method_gen,
	model::{AutoImpl, ContractTrait, Method, MethodImpl},
	parse::split_path_last,
};
use syn::punctuated::Punctuated;
use syn::token::Colon2;

/// Path to a Rust module containing a contract call proxy.
pub type ProxyModulePath = Punctuated<syn::PathSegment, Colon2>;

pub struct ProxyGetterReturnType {
	pub module_path: ProxyModulePath,
	pub proxy_obj_name: syn::PathSegment,
}

/// Return type of the proxy getter method, split into module and type segment.
pub fn proxy_getter_return_type(m: &Method) -> ProxyGetterReturnType {
	match &m.return_type {
		syn::ReturnType::Default => panic!(
			"Missing return type from proxy getter `{}`",
			m.name.to_string()
		),
		syn::ReturnType::Type(_, ty) => {
			if let syn::Type::Path(type_path) = ty.as_ref() {
				if let Some((leading_segments, last_segment)) = split_path_last(&type_path.path) {
					ProxyGetterReturnType {
						module_path: leading_segments,
						proxy_obj_name: last_segment,
					}
				} else {
					panic!("Proxy getter return type must be specfied with some module specifier (e.g. `path::to::module::Proxy`)");
				}
			} else {
				panic!("Invalid proxy getter return type")
			}
		},
	}
}

pub fn generate_proxy_getter_impl(m: &Method) -> proc_macro2::TokenStream {
	assert!(
		m.method_args.len() == 1,
		"Proxy getter must have 1 argument, which is the target address"
	);

	let msig = method_gen::generate_sig(&m);
	let address_arg_name = &m.method_args[0].pat;
	let parsed_return_type = proxy_getter_return_type(m);
	let module_path = &parsed_return_type.module_path;

	quote! {
		#msig {
			#module_path Proxy::new_proxy_obj(self.send(), #address_arg_name)
		}
	}
}

pub fn generate_all_proxy_trait_imports(c: &ContractTrait) -> Vec<proc_macro2::TokenStream> {
	c.methods
		.iter()
		.filter_map(|m| {
			if let MethodImpl::Generated(AutoImpl::ProxyGetter) = &m.implementation {
				let parsed_return_type = proxy_getter_return_type(m);
				let module_path = &parsed_return_type.module_path;
				Some(quote! {
					use #module_path ProxyTrait as _;
				})
			} else {
				None
			}
		})
		.collect()
}
