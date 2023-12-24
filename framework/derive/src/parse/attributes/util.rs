use crate::model::EsdtAttribute;

pub(super) fn is_attribute_with_no_args(attr: &syn::Attribute, name: &str) -> bool {
    if let Some(first_seg) = attr.path().segments.first() {
        if first_seg.ident == name {
            assert!(
                attr.path().segments.len() == 1,
                "no arguments allowed for attribute `{name}`"
            );
            return true;
        }
    };

    false
}

pub(super) fn get_attribute_with_one_type_arg(
    attr: &syn::Attribute,
    name: &str,
) -> Option<EsdtAttribute> {
    let attr_path = &attr.path();
    if let Some(first_seg) = attr_path.segments.first() {
        if first_seg.ident == name {
            let (ticker, ty) = match attr.meta.clone() {
                syn::Meta::Path(_) => {
                    panic!("attribute needs 2 arguments: ticker (string) and type")
                },
                syn::Meta::List(list) => {
                    assert!(!list.tokens.is_empty(), "argument can not be empty. attribute needs 2 arguments: ticker (string) and type");

                    let mut iter = list.tokens.into_iter();

                    let first_literal = match iter.next() {
                        Some(proc_macro2::TokenTree::Literal(literal)) => literal.to_string(),
                        _ => {
                            panic!("expected a string as the first token in the attribute argument")
                        },
                    };

                    let ticker = first_literal.trim_matches('\"').to_string();

                    let _ = match iter.next() {
                        Some(proc_macro2::TokenTree::Punct(punct)) => punct,
                        _ => panic!("expected a punctuation token after the first literal"),
                    };

                    let mut ty = proc_macro2::TokenStream::new();

                    for token in &mut iter {
                        match token {
                            proc_macro2::TokenTree::Punct(punct) => {
                                ty.extend(quote! { #punct });
                            },
                            proc_macro2::TokenTree::Ident(ident) => {
                                ty.extend(quote! { #ident });
                            },
                            _ => break,
                        }
                    }

                    if ticker.trim().is_empty() {
                        panic!("ticker field can't be empty");
                    }

                    (ticker, ty)
                },
                syn::Meta::NameValue(_) => panic!("arguments can not be name value"),
            };

            let esdt_attribute = EsdtAttribute { ticker, ty };

            return Some(esdt_attribute);
        }
    }

    None
}

pub(super) fn attr_one_string_arg(attr: &syn::Attribute) -> String {
    match attr.meta.clone() {
        syn::Meta::Path(path) => {
            let mut iter = path.segments.into_iter();
            match iter.next() {
                Some(syn::PathSegment {
                    ident: _,
                    arguments: syn::PathArguments::None,
                }) => String::new(),
                Some(_) => panic!("unexpected attribute argument tokens"),
                None => panic!("unexpected attribute argument tokens"),
            }
        },
        syn::Meta::List(list) => {
            assert!(
                list.delimiter == syn::MacroDelimiter::Paren(syn::token::Paren::default()),
                "attribute paranthesis expected"
            );

            assert!(
                !list.tokens.is_empty(),
                "attribute needs to have at least one argument"
            );

            let mut iter = list.tokens.into_iter();
            let arg_token_tree = match iter.next() {
                Some(proc_macro2::TokenTree::Literal(literal)) => {
                    assert!(
                        !literal.to_string().trim_matches('"').trim().is_empty(),
                        "the argument can not be an empty string or whitespace"
                    );
                    literal.to_string().trim_matches('\"').to_string()
                },
                Some(_) => {
                    panic!("unexpected attribute argument tokens: attribute has to be a string")
                },
                None => panic!("attribute needs to have at least one argument"),
            };

            assert!(iter.next().is_none(), "too many tokens in attribute");
            arg_token_tree
        },
        syn::Meta::NameValue(_) => {
            panic!("unexpected attribute argument tokens: argument can not be name value")
        },
    }
}

pub(super) fn is_attr_one_string_arg(attr: &syn::Attribute, attr_name: &str) -> Option<String> {
    if let Some(first_seg) = attr.path().segments.first() {
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
    match attr.clone().meta {
        syn::Meta::Path(val) => {
            let mut iter = val.segments.into_iter();
            let arg_token_tree: Option<proc_macro2::TokenTree> = match iter.next() {
                Some(syn::PathSegment {
                    ident: _,
                    arguments: syn::PathArguments::None,
                }) => None,
                Some(_) => panic!("unexpected attribute argument tokens"),
                None => None,
            };

            arg_token_tree
        },
        syn::Meta::List(val) => {
            assert!(
                val.delimiter == syn::MacroDelimiter::Paren(syn::token::Paren::default()),
                "attribute paranthesis expected"
            );

            assert!(
                !val.tokens.is_empty(),
                "attribute needs to have at least one argument"
            );

            let mut iter = val.tokens.into_iter();
            let arg_token_tree: Option<proc_macro2::TokenTree> = match iter.next() {
                Some(proc_macro2::TokenTree::Ident(ident)) => {
                    Some(proc_macro2::TokenTree::Ident(ident))
                },
                Some(_) => panic!("unexpected attribute argument tokens"),
                None => None,
            };

            assert!(iter.next().is_none(), "too many tokens in attribute");
            arg_token_tree
        },
        syn::Meta::NameValue(_) => panic!("unexpected attribute argument tokens"),
    }
}

/// Finds a method attribute with given name and 1 single optional argument.
/// In the result, the first option is for the attribute, the second for the argument.
pub(super) fn is_attr_with_one_opt_token_tree_arg(
    attr: &syn::Attribute,
    attr_name: &str,
) -> Option<Option<proc_macro2::TokenTree>> {
    if let Some(first_seg) = attr.path().segments.first() {
        if first_seg.ident == attr_name {
            Some(attr_one_opt_token_tree_arg(attr))
        } else {
            None
        }
    } else {
        None
    }
}
