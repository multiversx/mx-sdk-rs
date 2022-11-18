use super::{util::is_attr_one_string_arg, attr_names::ATTR_LABEL};

pub struct LabelAttribute {
    pub label: String,
}

impl LabelAttribute{
    pub fn parse(attr: &syn::Attribute) -> Option<Self> {
        is_attr_one_string_arg(attr, ATTR_LABEL).map(|arg_str| LabelAttribute {
            label: arg_str,
        })
    }
}
