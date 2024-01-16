use proc_macro::{quote, Literal};

use crate::format::format_tokenize;

pub fn semver_tuple(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let tokens: Vec<proc_macro::TokenTree> = format_tokenize::tokenize(input);

    tokens
        .iter()
        .map(|token| match token {
            proc_macro::TokenTree::Group(lit) => {
                let format_string = lit.stream().to_string();

                let version_tokens: Vec<&str> = format_string.split('.').collect();
                assert!(
                    version_tokens.len() == 3,
                    "The argument does not have the required format."
                );

                let major = u64_literal_from_str(version_tokens[0]);
                let minor = u64_literal_from_str(version_tokens[1]);
                let patch = u64_literal_from_str(version_tokens[2]);

                quote!(
                    ($major, $minor, $patch)
                )
            },
            _ => panic!("Tokentree does not match with the requirements"),
        })
        .collect()
}

fn u64_literal_from_str(s: &str) -> proc_macro::TokenTree {
    proc_macro::TokenTree::Literal(Literal::u64_suffixed(
        s.parse().expect("failed to parse token as u64"),
    ))
}
