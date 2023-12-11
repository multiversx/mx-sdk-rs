use quote::ToTokens;

use super::{attr_names::*, util::*};

/// unlike the others, this is standard Rust,
/// all doc comments get automatically transformed into "doc" attributes
static ATTR_DOC: &str = "doc";

/// Doc comments are actually syntactic sugar for doc attributes,
/// so extracting doc comments means parsing "doc" attributes.
pub fn extract_doc(attrs: &[syn::Attribute]) -> Vec<String> {
    attrs
        .iter()
        .filter(|attr| {
            if let Some(first_seg) = attr.path().segments.first() {
                first_seg.ident == ATTR_DOC
            } else {
                false
            }
        })
        .map(|attr| match attr.meta.clone() {
            syn::Meta::Path(_) => panic!("wrong format. expected name value, received path"),
            syn::Meta::List(_) => panic!("wrong format. expected name value, received list"),
            syn::Meta::NameValue(meta_name_value) => {
                if let syn::Expr::Lit(lit_str) = meta_name_value.value {
                    if meta_name_value.path.is_ident("doc") {
                        let value = lit_str.lit;
                        if let Some(tuple) = value
                            .to_token_stream()
                            .to_string()
                            .split_once(char::is_whitespace)
                        {
                            remove_backslashes(tuple.1)
                        } else {
                            String::new()
                        }
                    } else {
                        panic!("Attribute doesn't have the 'doc' identifier");
                    }
                } else {
                    panic!("Value is not a string literal");
                }
            },
        })
        .collect()
}

fn remove_backslashes(input: &str) -> String {
    input
        .trim_matches('\"')
        .replace("\\\"", "\"")
        .replace("\\'", "'")
}

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
