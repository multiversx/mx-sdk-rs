use proc_macro::{TokenStream, TokenTree};
use radix_trie::TrieKey;
use std::iter::once;

pub(super) struct TrieTokenStream(TokenStream);

impl TrieTokenStream {
    pub fn new(token_stream: TokenStream) -> Self {
        TrieTokenStream(token_stream)
    }
}

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
