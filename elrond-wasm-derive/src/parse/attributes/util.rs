pub(super) fn is_attribute_with_no_args(attr: &syn::Attribute, name: &str) -> bool {
    if let Some(first_seg) = attr.path.segments.first() {
        if first_seg.ident == name {
            assert!(
                attr.path.segments.len() == 1,
                "no arguments allowed for attribute `{}`",
                name
            );
            return true;
        }
    };

    false
}

pub(super) fn attr_one_string_arg(attr: &syn::Attribute) -> String {
    let result_str: String;
    let mut iter = attr.clone().tokens.into_iter();
    match iter.next() {
        Some(proc_macro2::TokenTree::Group(group)) => {
            assert!(
                group.delimiter() == proc_macro2::Delimiter::Parenthesis,
                "annotation paranthesis expected (check events and storage)"
            );
            let mut iter2 = group.stream().into_iter();
            match iter2.next() {
                Some(proc_macro2::TokenTree::Literal(lit)) => {
                    let str_val = lit.to_string();
                    assert!(
                        str_val.starts_with('\"') && str_val.ends_with('\"'),
                        "string literal expected as attribute argument (check events and storage)"
                    );
                    let substr = &str_val[1..str_val.len() - 1];
                    result_str = substr.to_string();
                },
                _ => panic!("literal expected as annotation identifier (check events and storage)"),
            }
        },
        _ => panic!("missing annotation identifier (check events and storage)"),
    }

    assert!(
        iter.next().is_none(),
        "too many tokens in attribute (check events and storage)"
    );

    result_str
}

pub(super) fn is_attr_one_string_arg(attr: &syn::Attribute, attr_name: &str) -> Option<String> {
    if let Some(first_seg) = attr.path.segments.first() {
        if first_seg.ident == attr_name {
            Some(attr_one_string_arg(attr))
        } else {
            None
        }
    } else {
        None
    }
}

fn attr_one_opt_token_tree_arg(attr: &syn::Attribute) -> Option<proc_macro2::TokenTree> {
    let mut iter = attr.clone().tokens.into_iter();
    let arg_token_tree: Option<proc_macro2::TokenTree> = match iter.next() {
        Some(proc_macro2::TokenTree::Group(group)) => {
            assert!(
                group.delimiter() == proc_macro2::Delimiter::Parenthesis,
                "attribute paranthesis expected"
            );
            let mut iter2 = group.stream().into_iter();
            match iter2.next() {
                Some(token_tree) => Some(token_tree),
                _ => panic!("attribute argument expected"),
            }
        },
        Some(_) => panic!("unexpected attribute argument tokens"),
        None => None,
    };

    assert!(iter.next().is_none(), "too many tokens in attribute");

    arg_token_tree
}

/// Finds a method attribute with given name and 1 single optional argument.
/// In the result, the first option is for the attribute, the second for the argument.
pub(super) fn is_attr_with_one_opt_token_tree_arg(
    attr: &syn::Attribute,
    attr_name: &str,
) -> Option<Option<proc_macro2::TokenTree>> {
    if let Some(first_seg) = attr.path.segments.first() {
        if first_seg.ident == attr_name {
            Some(attr_one_opt_token_tree_arg(attr))
        } else {
            None
        }
    } else {
        None
    }
}
