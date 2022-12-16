use proc_macro::TokenStream;

mod substitution_algorithm;
mod substitution_key;
mod substitution_list;
mod substitution_map;

pub fn trait_preprocessing(input: TokenStream) -> TokenStream {
    substitution_algorithm::perform_substitutions(input, &substitution_list::substitutions())
}
