use super::{attr_names::ATTR_OUTPUT_NAME, util::is_attr_one_string_arg};

pub struct OutputNameAttribute {
    pub output_name: String,
}

impl OutputNameAttribute {
    pub fn parse(attr: &syn::Attribute) -> Option<Self> {
        is_attr_one_string_arg(attr, ATTR_OUTPUT_NAME).map(|arg_str| OutputNameAttribute {
            output_name: arg_str,
        })
    }
}
