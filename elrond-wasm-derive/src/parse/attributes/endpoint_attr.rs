use super::{attr_names::*, util::*};

pub fn is_init(attr: &syn::Attribute) -> bool {
    is_attribute_with_no_args(attr, ATTR_INIT)
}

pub fn is_only_owner(attr: &syn::Attribute) -> bool {
    is_attribute_with_no_args(attr, ATTR_ONLY_OWNER)
}

pub fn is_callback_raw(attr: &syn::Attribute) -> bool {
    is_attribute_with_no_args(attr, ATTR_CALLBACK_RAW_DECL)
}

pub fn is_proxy(attr: &syn::Attribute) -> bool {
    is_attribute_with_no_args(attr, ATTR_PROXY)
}

#[derive(Clone, Debug)]
pub struct EndpointAttribute {
    pub endpoint_name: Option<syn::Ident>,
}

impl EndpointAttribute {
    pub fn parse(attr: &syn::Attribute) -> Option<EndpointAttribute> {
        match is_attr_with_one_opt_token_tree_arg(attr, ATTR_ENDPOINT) {
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
    pub fn parse(attr: &syn::Attribute) -> Option<ViewAttribute> {
        match is_attr_with_one_opt_token_tree_arg(attr, ATTR_VIEW) {
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
    pub fn parse(attr: &syn::Attribute) -> Option<CallbackAttribute> {
        match is_attr_with_one_opt_token_tree_arg(attr, ATTR_CALLBACK_DECL) {
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
