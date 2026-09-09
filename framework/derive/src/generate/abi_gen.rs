use crate::model::ContractTrait;

/// Given a path to a `#[contract_abi]`-annotated ABI spec trait (e.g. `some_module::SomeTrait`,
/// or just `SomeTrait` when it's declared in the current module), returns the path to its
/// generated `AbiProvider` sibling: the same leading module path, with the trait name swapped
/// out for the fixed `AbiProvider` ident `#[contract_abi]` always generates alongside it
/// (regardless of any `call = ProxyName` argument it was given) - the same "resolve a sibling
/// generated item from a module/trait path" trick contract module supertraits use to reach their
/// own `EndpointWrappers`/`ProxyTrait`/etc. (see `parse::split_path_last`, `model::Supertrait`,
/// and `abi_gen::generate_supertrait_snippets`'s `<#module_path AbiProvider as ...>` in
/// `multiversx-sc-abi-derive-common`).
fn abi_provider_path(trait_path: &syn::Path) -> syn::Path {
    let mut path = trait_path.clone();
    if let Some(last_segment) = path.segments.last_mut() {
        last_segment.ident = syn::Ident::new("AbiProvider", last_segment.ident.span());
        last_segment.arguments = syn::PathArguments::None;
    }
    path
}

/// `implements_abi`/`implements_abi_exactly` are the paths given via `#[multiversx_sc::contract(
/// implements_abi = ..., implements_abi_exactly = ...)]`: each names a `#[contract_abi]`-annotated
/// ABI spec trait (its generated `AbiProvider` sibling is resolved via `abi_provider_path`).
/// Their `abi()` is called and recorded on the contract's own `ContractAbi`, for the meta crate to
/// check conformance against later.
pub fn generate_abi_provider(
    contract: &ContractTrait,
    is_contract_main: bool,
    implements_abi: &[syn::Path],
    implements_abi_exactly: &[syn::Path],
) -> proc_macro2::TokenStream {
    let implements_abi: Vec<syn::Path> = implements_abi.iter().map(abi_provider_path).collect();
    let implements_abi_exactly: Vec<syn::Path> = implements_abi_exactly
        .iter()
        .map(abi_provider_path)
        .collect();
    let extra_abi_body_stmts = quote! {
        #(contract_abi.implements_abi.push(<#implements_abi as multiversx_sc_abi::ContractAbiProvider>::abi());)*
        #(contract_abi.implements_abi_exactly.push(<#implements_abi_exactly as multiversx_sc_abi::ContractAbiProvider>::abi());)*
    };
    multiversx_sc_abi_derive_common::contract::abi_gen::generate_abi_provider(
        contract,
        is_contract_main,
        multiversx_sc_abi_derive_common::TypeAbiImportCrate::MultiversxSc,
        &quote! { multiversx_sc::contract_base::ContractAbiProvider },
        quote! { type Api = multiversx_sc::api::uncallable::UncallableApi; },
        extra_abi_body_stmts,
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
