use quote::quote;

use crate::format::format_tokenize;

pub fn semver_tuple(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let tokens = format_tokenize::tokenize(input);

    tokens.iter().map(convert_token_tree).collect()
}

fn convert_token_tree(token: &proc_macro2::TokenTree) -> proc_macro2::TokenStream {
    match token {
        proc_macro2::TokenTree::Group(lit) => {
            let format_string = lit.stream().to_string();

            let version_tokens: Vec<&str> = format_string.split('.').collect();
            assert!(
                version_tokens.len() == 3,
                "The argument does not have the required format."
            );

            let major = u64_literal_from_str(version_tokens[0]);
            let minor = u64_literal_from_str(version_tokens[1]);
            let patch = u64_literal_from_str(version_tokens[2]);

            quote! {
                (#major, #minor, #patch)
            }
        }
        _ => panic!("Tokentree does not match with the requirements"),
    }
}

fn u64_literal_from_str(s: &str) -> proc_macro2::TokenTree {
    // For some reason a space creeps in at the end,
    // but only when running from rust-analyzer,
    // therefore also calling a trim()
    proc_macro2::TokenTree::Literal(proc_macro2::Literal::u64_suffixed(
        s.trim()
            .parse()
            .unwrap_or_else(|err| panic!("failed to parse token as u64 '{s}': {err}")),
    ))
}
