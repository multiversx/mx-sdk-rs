use proc_macro::{Group, TokenStream, TokenTree};
use std::iter::FromIterator;

use super::substitution_map::SubstitutionsMap;

pub(super) fn perform_substitutions(
    input: TokenStream,
    substitutions: &SubstitutionsMap,
) -> TokenStream {
    let mut result = Vec::<TokenTree>::new();
    let mut tt_iter = input.into_iter();
    let mut to_skip: usize = 0;
    loop {
        if to_skip > 0 {
            to_skip -= 1;
            tt_iter.next();
            continue;
        }
        if let Some((sub_length, sub)) = substitutions.check_subsequence(tt_iter.clone()) {
            result.extend(sub.clone().into_iter());
            to_skip = sub_length;
            continue;
        }
        if let Some(tt) = tt_iter.next() {
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
        } else {
            break;
        }
    }
    proc_macro::TokenStream::from_iter(result.into_iter())
}
