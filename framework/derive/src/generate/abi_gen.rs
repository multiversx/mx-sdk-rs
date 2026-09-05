use crate::model::ContractTrait;

pub fn generate_abi_provider(
    contract: &ContractTrait,
    is_contract_main: bool,
) -> proc_macro2::TokenStream {
    multiversx_sc_abi_derive_common::contract::abi_gen::generate_abi_provider(
        contract,
        is_contract_main,
        multiversx_sc_abi_derive_common::TypeAbiImportCrate::MultiversxSc,
        &quote! { multiversx_sc::contract_base::ContractAbiProvider },
        quote! { type Api = multiversx_sc::api::uncallable::UncallableApi; },
    )
}

/// Generates a framework-agnostic call proxy (`proxy_name` / `proxy_name`+`Methods`), the same
/// kind produced by `data/abi-derive`'s `#[contract_abi(call = ...)]`, but from a real
/// `#[multiversx_sc::contract]`/`#[multiversx_sc::module]` trait (already gone through the
/// managed-type substitution, e.g. bare `BigUint` became `multiversx_sc::types::BigUint<Self::
/// Api>`). `UncallableApi` stands in for that `Self::Api` wherever it appears in an argument or
/// return type; which concrete API is chosen doesn't matter, since the proxy only ever uses the
/// resulting type's API-erased `TypeAbi::Abi` projection (e.g. `BigUint<UncallableApi>::Abi` is
/// `BigUintAbi`, same as for any other API).
pub fn generate_call_proxy(
    contract: &ContractTrait,
    proxy_name: &syn::Ident,
) -> proc_macro2::TokenStream {
    multiversx_sc_abi_derive_common::contract::proxy_gen::generate_abi_proxy(
        contract,
        multiversx_sc_abi_derive_common::TypeAbiImportCrate::MultiversxSc,
        proxy_name,
        &syn::parse_quote! { multiversx_sc::api::uncallable::UncallableApi },
    )
}
