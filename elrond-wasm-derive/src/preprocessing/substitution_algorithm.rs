use proc_macro::{Group, TokenStream, TokenTree};

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
            let first_token_span = tt_iter.clone().next().unwrap().span();
            let final_sub = sub.clone().into_iter().map(|mut tt| {
                tt.set_span(first_token_span);
                tt
            });
            result.extend(final_sub);
            to_skip = sub_length;
            continue;
        }
        if let Some(tt) = tt_iter.next() {
            match tt {
                TokenTree::Group(g) => {
                    let mut substituted_group = TokenTree::Group(Group::new(
                        g.delimiter(),
                        perform_substitutions(g.stream(), substitutions),
                    ));
                    substituted_group.set_span(g.span());
                    result.push(substituted_group);
                    continue;
                },
                _ => result.push(tt),
            }
        } else {
            break;
        }
    }
    result.into_iter().collect()
}
