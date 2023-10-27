use crate::model::EsdtAttribute;

use super::{trait_prop_names::*, util::*};

pub fn is_only_owner_prop(attr: &syn::Attribute) -> bool {
    is_attribute_with_no_args(attr, PROP_ONLY_OWNER)
}

pub fn is_only_admin_prop(attr: &syn::Attribute) -> bool {
    is_attribute_with_no_args(attr, PROP_ADMIN_OWNER)
}

pub fn is_allow_multiple_var_args_prop(attr: &syn::Attribute) -> bool {
    is_attribute_with_no_args(attr, PROP_ALLOW_MULTIPLE_VAR_ARGS)
}

pub fn is_esdt_attribute_prop(attr: &syn::Attribute) -> (bool, Option<EsdtAttribute>) {
    is_attribute_with_one_type_arg(attr, PROP_ESDT_ATTRIBUTE)
}
