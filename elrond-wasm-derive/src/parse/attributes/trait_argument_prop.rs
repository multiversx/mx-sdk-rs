use super::{trait_prop_names::*, util::*};

pub fn is_only_owner_prop(attr: &syn::Attribute) -> bool {
    is_attribute_with_no_args(attr, PROP_ONLY_OWNER)
}

pub fn is_only_admin_prop(attr: &syn::Attribute) -> bool {
    is_attribute_with_no_args(attr, PROP_ADMIN_OWNER)
}


pub struct TargetAttribute {
    pub location: String,
}

impl TargetAttribute{
    pub fn parse(attr: &syn::Attribute) -> Option<Self> {
        is_attr_one_string_arg(attr, PROP_TARGET).map(|arg_str| TargetAttribute {
            location: arg_str,
        })
    }
}
