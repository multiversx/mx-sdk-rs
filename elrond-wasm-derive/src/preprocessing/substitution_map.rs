use proc_macro::{token_stream::IntoIter, TokenStream};
use radix_trie::Trie;
use std::iter::once;

use super::substitution_key::TrieTokenStream;

pub struct SubstitutionsMap {
    trie: Trie<TrieTokenStream, TokenStream>,
}

impl SubstitutionsMap {
    pub fn new() -> Self {
        SubstitutionsMap { trie: Trie::new() }
    }

    pub fn add_substitution(
        &mut self,
        key: proc_macro2::TokenStream,
        value: proc_macro2::TokenStream,
    ) {
        self.trie.insert(key.into(), value.into());
    }

    pub fn check_subsequence(&self, tt_iter: IntoIter) -> Option<(usize, &TokenStream)> {
        let mut current_length: usize = 1;
        let mut result: Option<(usize, &TokenStream)> = None;
        let mut current_key = TokenStream::new();
        for tt in tt_iter {
            current_key.extend(once(tt));
            let trie_key = TrieTokenStream::new(current_key.clone());
            if let Some(sub) = self.trie.get(&trie_key) {
                result = Some((current_length, sub));
            }
            if self.trie.get_raw_descendant(&trie_key).is_none() {
                break;
            }
            current_length += 1;
        }
        result
    }
}
