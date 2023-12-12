use super::attr_names::*;

pub struct PayableAttribute {
    pub identifier: Option<String>,
}

impl PayableAttribute {
    pub fn parse(attr: &syn::Attribute) -> Option<PayableAttribute> {
        if let Some(first_seg) = attr.path().segments.first() {
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
    match attr.meta.clone() {
        syn::Meta::Path(_) => {
            panic!("attribute needs 1 string argument: Replace with #[payable(\"*\")] or #[payable(\"EGLD\")]")
        },
        syn::Meta::List(list) => {
            let mut iter = list.tokens.into_iter();
            let ticker = match iter.next() {
                Some(proc_macro2::TokenTree::Literal(literal)) => {
                    let clean = literal.to_string().trim_matches('\"').trim().to_string();
                    assert!(
                        !clean.is_empty(),
                        "ticker can not be empty. attribute needs 1 string argument: Replace with #[payable(\"*\")] or #[payable(\"EGLD\")"
                    );
                    clean
                },
                Some(_) => panic!("expected a string as argument"),
                None => panic!("argument can not be empty. attribute needs 1 string argument: Replace with #[payable(\"*\")] or #[payable(\"EGLD\")"),
            };

            assert!(
                iter.next().is_none(),
                "too many tokens in attribute argument"
            );
            Some(ticker)
        },
        syn::Meta::NameValue(_) => panic!(
            "attribute can not be name value. attribute needs 1 string argument: \"*\" or \"EGLD\""
        ),
    }
}
