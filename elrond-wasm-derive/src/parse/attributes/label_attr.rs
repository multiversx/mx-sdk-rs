use super::{util::is_attr_one_string_arg, trait_prop_names::PROP_LABEL};

pub struct LabelAttribute {
    pub label: String,
}

impl LabelAttribute{
    pub fn parse(attr: &syn::Attribute) -> Option<Self> {
        is_attr_one_string_arg(attr, PROP_LABEL).map(|arg_str| LabelAttribute {
            label: arg_str,
        })
    }
}
