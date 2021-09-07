use proc_macro::{Group, TokenStream, TokenTree};
use std::{collections::HashMap, iter::FromIterator};

fn substitutions() -> HashMap<String, TokenStream> {
    let mut substitutions = HashMap::<String, TokenStream>::new();
    substitutions.insert(
        "BigInt".to_string(),
        quote!(elrond_wasm::types::BigInt<Self::TypeManager>).into(),
    );
    substitutions.insert(
        "BigUint".to_string(),
        quote!(elrond_wasm::types::BigUint<Self::TypeManager>).into(),
    );
    substitutions.insert(
        "ManagedBuffer".to_string(),
        quote!(elrond_wasm::types::ManagedBuffer<Self::TypeManager>).into(),
    );
    substitutions.insert(
        "EllipticCurve".to_string(),
        quote!(elrond_wasm::types::EllipticCurve<Self::TypeManager>).into(),
    );
    substitutions.insert(
        "ManagedAddress".to_string(),
        quote!(elrond_wasm::types::ManagedAddress<Self::TypeManager>).into(),
    );
    substitutions.insert(
        "TokenIdentifier".to_string(),
        quote!(elrond_wasm::types::TokenIdentifier<Self::TypeManager>).into(),
    );
    substitutions
}

pub fn trait_preprocessing(input: TokenStream) -> TokenStream {
    perform_substitutions(input, &substitutions())
}

fn perform_substitutions(
    input: TokenStream,
    substitutions: &HashMap<String, TokenStream>,
) -> TokenStream {
    let mut result = Vec::<TokenTree>::new();
    for tt in input.into_iter() {
        match tt {
            TokenTree::Group(g) => {
                result.push(TokenTree::Group(Group::new(
                    g.delimiter(),
                    perform_substitutions(g.stream(), substitutions),
                )));
            },
            TokenTree::Ident(ident) => {
                if let Some(sub) = substitutions.get(&ident.to_string()) {
                    result.extend(sub.clone().into_iter());
                } else {
                    result.push(TokenTree::Ident(ident));
                }
            },
            other => {
                result.push(other);
            },
        }
    }
    proc_macro::TokenStream::from_iter(result.into_iter())
}
