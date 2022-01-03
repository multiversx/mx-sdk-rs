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
            if let Some(first_seg) = attr.path.segments.first() {
                first_seg.ident == ATTR_DOC
            } else {
                false
            }
        })
        .map(|attr| {
            let mut tokens_iter = attr.clone().tokens.into_iter();

            // checking punctuation, the first token is '='
            if let Some(proc_macro2::TokenTree::Punct(punct)) = tokens_iter.next() {
                assert_eq!(punct.as_char(), '=');
            } else {
                panic!("malformed doc attribute");
            }

            if let Some(proc_macro2::TokenTree::Literal(lit)) = tokens_iter.next() {
                let lit_str = lit.to_string();
                let mut message_slice = lit_str.as_str();

                // the useful part of the message is between quotes
                assert!(
                    message_slice.starts_with('\"') && message_slice.ends_with('\"'),
                    "malformed doc attribute: string literal expected"
                );
                message_slice = &message_slice[1..message_slice.len() - 1];

                // most doc comments start with a space, so remove that too
                if message_slice.starts_with(' ') {
                    message_slice = &message_slice[1..];
                }

                // also unescape escaped single and double quotes
                message_slice.replace("\\\"", "\"").replace("\\'", "'")
            } else {
                panic!("malformed doc attribute");
            }
        })
        .collect()
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
