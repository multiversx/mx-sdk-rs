use super::contract_impl::contract_implementation;
use crate::{
    parse::parse_contract_trait, preprocessing::trait_preprocessing, validate::validate_contract,
};

/// `#[multiversx_sc::contract]` takes one optional argument: `call = ProxyName`, which also
/// generates a framework-agnostic call proxy under that name (the same kind produced by
/// `#[contract_abi(call = ...)]`); see `contract_impl::contract_implementation`.
pub fn process_contract(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let call_arg = parse_macro_input!(
        args as multiversx_sc_abi_derive_common::contract::proxy_gen::ProxyCallArg
    );

    let new_input = trait_preprocessing(input);
    let proc_input = parse_macro_input!(new_input as syn::ItemTrait);

    let contract = parse_contract_trait(proc_macro2::TokenStream::new(), &proc_input);

    validate_contract(&contract);

    let contract_impl =
        contract_implementation(&contract, true, call_arg.proxy_name.as_ref());

    proc_macro::TokenStream::from(contract_impl)
}
