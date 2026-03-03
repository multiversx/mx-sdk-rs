use super::{attr_names::ATTR_TITLE, util::is_attr_one_string_arg};

pub struct TitleAttribute {
    pub title: String,
}

impl TitleAttribute {
    pub fn parse(attr: &syn::Attribute) -> Option<Self> {
        is_attr_one_string_arg(attr, ATTR_TITLE).map(|arg_str| TitleAttribute { title: arg_str })
    }
}
