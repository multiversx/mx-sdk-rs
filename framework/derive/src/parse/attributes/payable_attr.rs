use super::attr_names::*;

pub struct PayableAttribute {
    pub identifier: Option<String>,
}

impl PayableAttribute {
    pub fn parse(attr: &syn::Attribute) -> Option<PayableAttribute> {
        if let Some(first_seg) = attr.path.segments.first() {
            if first_seg.ident == ATTR_PAYABLE {
                Some(PayableAttribute {
                    identifier: extract_token_identifier(attr),
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}

/// Current implementation only works with 1 token name.
/// Might be extended in the future.
fn extract_token_identifier(attr: &syn::Attribute) -> Option<String> {
    let mut iter = attr.clone().tokens.into_iter();
    let result_str = match iter.next() {
        Some(proc_macro2::TokenTree::Group(group)) => {
            assert!(
                group.delimiter() == proc_macro2::Delimiter::Parenthesis,
                "payable token name must be specified in parantheses"
            );
            let mut iter2 = group.stream().into_iter();
            match iter2.next() {
                Some(proc_macro2::TokenTree::Literal(lit)) => {
                    let str_val = lit.to_string();
                    assert!(
                        str_val.starts_with('\"') && str_val.ends_with('\"'),
                        "string literal expected as attribute argument"
                    );
                    let substr = &str_val[1..str_val.len() - 1];
                    Some(substr.to_string())
                },
                _ => panic!("literal expected as event identifier"),
            }
        },
        None => None,
        _ => panic!("unexpected payable attribute format"),
    };

    assert!(
        iter.next().is_none(),
        "event too many tokens in event attribute"
    );

    result_str
}
