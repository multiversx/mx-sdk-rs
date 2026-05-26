use proc_macro::{TokenStream, token_stream::IntoIter};
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

    /// Checks whether the beginning of `tt_iter` matches any key in the substitution map.
    ///
    /// Consumes tokens from `tt_iter` one at a time, building up a candidate key and probing
    /// the trie on each step. Stops as soon as there are no more trie descendants for the
    /// current prefix (i.e. no longer key can possibly match).
    ///
    /// Returns the **longest** match found as `Some((length, substitution))`, where `length`
    /// is the number of tokens consumed that form the matching key and `substitution` is the
    /// corresponding replacement `TokenStream`. Returns `None` if no key matches.
    pub fn check_subsequence(&self, tt_iter: IntoIter) -> Option<(usize, &TokenStream)> {
        let mut result: Option<(usize, &TokenStream)> = None;
        let mut current_key = TokenStream::new();
        for (current_length, tt) in (1..).zip(tt_iter) {
            current_key.extend(once(tt));
            let trie_key = TrieTokenStream::new(current_key.clone());
            if let Some(sub) = self.trie.get(&trie_key) {
                result = Some((current_length, sub));
            }
            if self.trie.get_raw_descendant(&trie_key).is_none() {
                break;
            }
        }
        result
    }
}
