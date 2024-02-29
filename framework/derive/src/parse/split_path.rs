use syn::{punctuated::Punctuated, token::Colon2};

/// Splits off the last part of a path from the rest.
/// e.g. `some::module::Item` will be split into `some::module::` and `Item`.
/// Note that the last `::` is retained in the first part of the result.
/// Returns none if no module is specified.
/// The method is designed for contexts where explicit module specification is required.
pub fn split_path_last(
    path: &syn::Path,
) -> Option<(Punctuated<syn::PathSegment, Colon2>, syn::PathSegment)> {
    if path.segments.len() >= 2 {
        let mut leading_segments = path.segments.clone();
        let last_segment = leading_segments.pop().unwrap().into_value();
        Some((leading_segments, last_segment))
    } else {
        None
    }
}
