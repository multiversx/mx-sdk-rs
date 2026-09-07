use super::contract_impl::contract_implementation;
use crate::{
    parse::parse_contract_trait, preprocessing::trait_preprocessing, validate::validate_contract,
};

/// `#[multiversx_sc::contract(...)]` takes three optional arguments:
/// - `call = ProxyName`, which also generates a framework-agnostic call proxy under that name
///   (the same kind produced by `#[contract_abi(call = ...)]`);
/// - `implements_abi = Path` (repeatable), recording that this contract's ABI must contain the
///   referenced ABI spec's exports;
/// - `implements_abi_exactly = Path` (repeatable), recording that this contract's ABI must match
///   the referenced ABI spec's exports exactly.
///
/// See `contract_impl::contract_implementation`.
pub fn process_contract(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let macro_args = parse_macro_input!(
        args as multiversx_sc_abi_derive_common::contract::macro_args::ContractMacroArgs
    );

    let new_input = trait_preprocessing(input);
    let proc_input = parse_macro_input!(new_input as syn::ItemTrait);

    let contract = parse_contract_trait(proc_macro2::TokenStream::new(), &proc_input);

    validate_contract(&contract);

    let contract_impl = contract_implementation(
        &contract,
        true,
        macro_args.call.as_ref(),
        &macro_args.implements_abi,
        &macro_args.implements_abi_exactly,
    );

    proc_macro::TokenStream::from(contract_impl)
}
