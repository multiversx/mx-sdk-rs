use syn::punctuated::Punctuated;

use crate::model::{ModulePath, Supertrait};

// TODO: would be nice to explicitly write `self::...` instead of no prefix.
pub fn self_module_path() -> ModulePath {
    Punctuated::new()
}

pub fn main_supertrait_decl(supertraits: &[Supertrait]) -> Vec<proc_macro2::TokenStream> {
    supertraits
        .iter()
        .map(|supertrait| {
            let full_path = &supertrait.full_path;
            quote! {
                + #full_path
            }
        })
        .collect()
}

pub fn endpoint_wrapper_supertrait_decl(
    supertraits: &[Supertrait],
) -> Vec<proc_macro2::TokenStream> {
    supertraits
        .iter()
        .map(|supertrait| {
            let module_path = &supertrait.module_path;
            quote! {
                + #module_path EndpointWrappers
            }
        })
        .collect()
}

pub fn proxy_supertrait_decl(supertraits: &[Supertrait]) -> Vec<proc_macro2::TokenStream> {
    supertraits
        .iter()
        .map(|supertrait| {
            let module_path = &supertrait.module_path;
            quote! {
                + #module_path ProxyTrait
            }
        })
        .collect()
}

fn impl_auto_impl(module_path: &ModulePath) -> proc_macro2::TokenStream {
    quote! {
        impl<A> #module_path AutoImpl for ContractObj<A>
        where
            A: multiversx_sc::api::VMApi,
        {
        }
    }
}

pub fn impl_all_auto_impl(supertraits: &[Supertrait]) -> Vec<proc_macro2::TokenStream> {
    let mut implementations: Vec<proc_macro2::TokenStream> = supertraits
        .iter()
        .map(|supertrait| impl_auto_impl(&supertrait.module_path))
        .collect();

    implementations.push(impl_auto_impl(&self_module_path()));

    implementations
}

// TODO: explore auto-implementations of supertraits
#[allow(dead_code)]
pub fn auto_impl_inheritance(supertraits: &[Supertrait]) -> Vec<proc_macro2::TokenStream> {
    supertraits
        .iter()
        .map(|supertrait| {
            let module_path = &supertrait.module_path;
            quote! {
                impl<C> #module_path AutoImpl for C where C: self::AutoImpl {}
            }
        })
        .collect()
}

fn impl_endpoint_wrappers(module_path: &ModulePath) -> proc_macro2::TokenStream {
    quote! {
        impl<A> #module_path EndpointWrappers for ContractObj<A>
        where
            A: multiversx_sc::api::VMApi,
        {
        }
    }
}

pub fn impl_all_endpoint_wrappers(supertraits: &[Supertrait]) -> Vec<proc_macro2::TokenStream> {
    let mut implementations: Vec<proc_macro2::TokenStream> = supertraits
        .iter()
        .map(|supertrait| impl_endpoint_wrappers(&supertrait.module_path))
        .collect();

    implementations.push(impl_endpoint_wrappers(&self_module_path()));

    implementations
}

#[allow(dead_code)]
pub fn endpoint_wrappers_inheritance(supertraits: &[Supertrait]) -> Vec<proc_macro2::TokenStream> {
    supertraits
        .iter()
        .map(|supertrait| {
            let module_path = &supertrait.module_path;
            quote! {
                impl<C> #module_path EndpointWrappers for C
                where
                    C: self::EndpointWrappers,
                {
                }
            }
        })
        .collect()
}

pub fn function_selector_module_calls(supertraits: &[Supertrait]) -> Vec<proc_macro2::TokenStream> {
    supertraits
        .iter()
        .map(|supertrait| {
            let module_path = &supertrait.module_path;
            quote! {
                if #module_path EndpointWrappers::call(self, fn_name) {
                    return true;
                }
            }
        })
        .collect()
}

fn impl_proxy_trait(module_path: &ModulePath) -> proc_macro2::TokenStream {
    quote! {
        impl<A> #module_path ProxyTrait for Proxy<A> where A: multiversx_sc::api::VMApi {}
    }
}

pub fn impl_all_proxy_traits(supertraits: &[Supertrait]) -> Vec<proc_macro2::TokenStream> {
    let mut implementations: Vec<proc_macro2::TokenStream> = supertraits
        .iter()
        .map(|supertrait| impl_proxy_trait(&supertrait.module_path))
        .collect();

    implementations.push(impl_proxy_trait(&self_module_path()));

    implementations
}
