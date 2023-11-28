use super::contract_impl::contract_implementation;
use crate::{
    parse::parse_contract_trait, preprocessing::trait_preprocessing, validate::validate_contract,
};

pub fn process_contract(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let new_input = trait_preprocessing(input);

    //if empty => parse error
    // let args_input = parse_macro_input!(args as syn::MetaList);

    // let list;
    // match args_input {
    //     syn::Meta::Path(_) => panic!("path"),
    //     syn::Meta::List(metalist) => {
    //         list = metalist;
    //         println!("metalist")
    //     },
    //     syn::Meta::NameValue(_) => panic!("namevalue"),
    // };

    let args_input = syn::MetaList {
        path: syn::Path {
            leading_colon: Some(syn::token::PathSep::default()),
            segments: syn::punctuated::Punctuated::<syn::PathSegment, syn::token::PathSep>::default()
        },
        delimiter: syn::MacroDelimiter::Paren(syn::token::Paren::default()),
        tokens: proc_macro2::TokenStream::new()
    };

    let proc_input = &parse_macro_input!(new_input as syn::ItemTrait);
    let contract = parse_contract_trait(args_input, proc_input);
    validate_contract(&contract);

    let contract_impl = contract_implementation(&contract, true);

    proc_macro::TokenStream::from(contract_impl)
}
