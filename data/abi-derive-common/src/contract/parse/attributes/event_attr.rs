use super::{attr_names::*, util::*};

pub struct EventAttribute {
    pub identifier: String,
}

impl EventAttribute {
    pub fn parse(attr: &syn::Attribute) -> Option<Self> {
        is_attr_one_string_arg(attr, ATTR_EVENT).map(|arg_str| EventAttribute {
            identifier: arg_str,
        })
    }
}
