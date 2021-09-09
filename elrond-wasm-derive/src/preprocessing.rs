use proc_macro::{token_stream::IntoIter, Group, TokenStream, TokenTree};
use radix_trie::{Trie, TrieKey};
use std::{iter::once, iter::FromIterator};

struct TrieTokenStream(TokenStream);

impl PartialEq for TrieTokenStream {
    fn eq(&self, other: &Self) -> bool {
        self.encode_bytes() == other.encode_bytes()
    }
}

impl Eq for TrieTokenStream {}

fn determinant(tt: &TokenTree) -> u8 {
    match tt {
        TokenTree::Group(_) => 0,
        TokenTree::Ident(_) => 1,
        TokenTree::Punct(_) => 2,
        TokenTree::Literal(_) => 3,
    }
}

impl TrieKey for TrieTokenStream {
    fn encode_bytes(&self) -> Vec<u8> {
        self.0
            .clone()
            .into_iter()
            .flat_map(|tt| {
                once(determinant(&tt))
                    .chain(tt.to_string().as_bytes().iter().cloned())
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}

impl From<TokenStream> for TrieTokenStream {
    fn from(ts: TokenStream) -> Self {
        TrieTokenStream(ts)
    }
}

impl From<proc_macro2::TokenStream> for TrieTokenStream {
    fn from(ts: proc_macro2::TokenStream) -> Self {
        TrieTokenStream(ts.into())
    }
}

impl From<TokenTree> for TrieTokenStream {
    fn from(tt: TokenTree) -> Self {
        TrieTokenStream(tt.into())
    }
}

type SubstitutionsMap = Trie<TrieTokenStream, TokenStream>;

fn add_substitution(
    substitutions: &mut SubstitutionsMap,
    key: proc_macro2::TokenStream,
    value: proc_macro2::TokenStream,
) {
    substitutions.insert(key.into(), value.into());
}

fn substitutions() -> SubstitutionsMap {
    let mut substitutions = Trie::new();
    add_substitution(
        &mut substitutions,
        quote!(BigInt),
        quote!(elrond_wasm::types::BigInt<Self::TypeManager>),
    );
    add_substitution(
        &mut substitutions,
        quote!(BigUint),
        quote!(elrond_wasm::types::BigUint<Self::TypeManager>),
    );
    add_substitution(
        &mut substitutions,
        quote!(BigUint::from),
        quote!(self.types().big_uint_from),
    );
    add_substitution(
        &mut substitutions,
        quote!(.managed_into()),
        quote!(.managed_into(self.type_manager())),
    );
    add_substitution(
        &mut substitutions,
        quote!(ManagedBuffer),
        quote!(elrond_wasm::types::ManagedBuffer<Self::TypeManager>),
    );
    add_substitution(
        &mut substitutions,
        quote!(EllipticCurve),
        quote!(elrond_wasm::types::EllipticCurve<Self::TypeManager>),
    );
    add_substitution(
        &mut substitutions,
        quote!(ManagedAddress),
        quote!(elrond_wasm::types::ManagedAddress<Self::TypeManager>),
    );
    add_substitution(
        &mut substitutions,
        quote!(TokenIdentifier),
        quote!(elrond_wasm::types::TokenIdentifier<Self::TypeManager>),
    );
    substitutions
}

pub fn trait_preprocessing(input: TokenStream) -> TokenStream {
    perform_substitutions(input, &substitutions())
}

fn perform_substitutions(input: TokenStream, substitutions: &SubstitutionsMap) -> TokenStream {
    let mut result = Vec::<TokenTree>::new();
    let mut tt_iter = input.into_iter();
    let mut to_skip: usize = 0;
    while let Some(tt) = tt_iter.next() {
        if to_skip > 0 {
            to_skip -= 1;
            continue;
        }
        match tt {
            TokenTree::Group(g) => {
                result.push(TokenTree::Group(Group::new(
                    g.delimiter(),
                    perform_substitutions(g.stream(), substitutions),
                )));
                continue;
            },
            _ => result.push(tt),
        }
        if let Some((sub_length, sub)) = check_subsequence(substitutions, tt_iter.clone()) {
            result.extend(sub.clone().into_iter());
            to_skip = sub_length;
        }
    }
    proc_macro::TokenStream::from_iter(result.into_iter())
}

fn check_subsequence(
    substitutions: &SubstitutionsMap,
    tt_iter: IntoIter,
) -> Option<(usize, &TokenStream)> {
    let mut current_length: usize = 1;
    let mut result: Option<(usize, &TokenStream)> = None;
    let mut current_key = TokenStream::new();
    for tt in tt_iter {
        current_key.extend(once(tt));
        let trie_key = TrieTokenStream(current_key.clone());
        if let Some(sub) = substitutions.get(&trie_key) {
            result = Some((current_length, sub));
        }
        if substitutions.get_raw_descendant(&trie_key).is_none() {
            break;
        }
        current_length += 1;
    }
    result
}
