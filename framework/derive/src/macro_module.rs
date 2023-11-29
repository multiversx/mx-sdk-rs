use super::contract_impl::contract_implementation;
use crate::{
    parse::parse_contract_trait, preprocessing::trait_preprocessing, validate::validate_contract,
};

pub fn process_module(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let new_input = trait_preprocessing(input);
    let proc_input = &parse_macro_input!(new_input as syn::ItemTrait);

    let args_input = if args.is_empty() {
        syn::MetaList {
            path: syn::Path {
                leading_colon: Some(syn::token::PathSep::default()),
                segments:
                    syn::punctuated::Punctuated::<syn::PathSegment, syn::token::PathSep>::default(),
            },
            delimiter: syn::MacroDelimiter::Paren(syn::token::Paren::default()),
            tokens: proc_macro2::TokenStream::new(),
        }
    } else {
        parse_macro_input!(args as syn::MetaList)
    };

    let contract = parse_contract_trait(args_input, proc_input);
    validate_contract(&contract);

    let contract_impl = contract_implementation(&contract, false);

    proc_macro::TokenStream::from(quote! {
        #contract_impl
    })
}
