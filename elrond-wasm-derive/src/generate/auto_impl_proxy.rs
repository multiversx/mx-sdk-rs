use crate::{
    generate::method_gen,
    model::{AutoImpl, ContractTrait, Method, MethodImpl},
    parse::split_path_last,
};
use syn::{punctuated::Punctuated, token::Colon2};

/// Path to a Rust module containing a contract call proxy.
pub type ProxyModulePath = Punctuated<syn::PathSegment, Colon2>;

pub struct ProxyGetterReturnType {
    pub module_path: ProxyModulePath,
    pub proxy_obj_name: syn::PathSegment,
}

/// Return type of the proxy getter method, split into module and type segment.
pub fn proxy_getter_return_type(m: &Method) -> ProxyGetterReturnType {
    match &m.return_type {
        syn::ReturnType::Default => panic!("Missing return type from proxy getter `{}`", m.name),
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

fn proxy_getter_address_snippet(m: &Method) -> proc_macro2::TokenStream {
    match m.method_args.len() {
        0 => quote! {},
        1 => {
            let address_arg_name = &m.method_args[0].pat;
            quote! {
                .contract(#address_arg_name)
            }
        },
        _ => panic!("Proxy getter can have at most 1 argument, which is the target address"),
    }
}

pub fn generate_proxy_getter_impl(m: &Method) -> proc_macro2::TokenStream {
    let msig = method_gen::generate_sig_with_attributes(m);
    let parsed_return_type = proxy_getter_return_type(m);
    let module_path = &parsed_return_type.module_path;
    let address_snippet = proxy_getter_address_snippet(m);

    quote! {
        #msig {
            #module_path Proxy::new_proxy_obj(self.raw_vm_api()) #address_snippet
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
