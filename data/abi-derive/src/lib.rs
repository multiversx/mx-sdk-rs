// ensure we don't run out of macro stack
#![recursion_limit = "1024"]

/// Generates the `AbiProvider` (`fn abi() -> ContractAbi`) for a contract-like trait, with no
/// dependency on `multiversx-sc`/`VMApi`. Meant for traits written directly against pure ABI
/// types (e.g. `BigUintAbi`, `AddressAbi`, ...), so unlike `#[multiversx_sc::contract]` there is
/// no managed-type substitution step.
///
/// Pass `call = ProxyName` to also generate a call proxy (`ProxyName` / `ProxyNameMethods`),
/// e.g. `#[contract_abi(call = KittyGeneticAlgAbiProxy)]`; without it, only `AbiProvider` is
/// generated.
///
/// Like `#[multiversx_sc::contract]`, the annotated trait is a spec consumed by the macro,
/// not re-emitted as-is: it uses framework-style attributes (`#[init]`, `#[endpoint]`, ...)
/// that don't exist as real attribute macros, so only the generated code is emitted.
#[proc_macro_attribute]
pub fn contract_abi(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let proc_input = syn::parse_macro_input!(input as syn::ItemTrait);
    let call_arg = syn::parse_macro_input!(
        args as multiversx_sc_abi_derive_common::contract::proxy_gen::ProxyCallArg
    );

    let contract = multiversx_sc_abi_derive_common::contract::parse::parse_contract_trait(
        proc_macro2::TokenStream::new(),
        &proc_input,
    );
    multiversx_sc_abi_derive_common::contract::validate::validate_contract(&contract);

    let abi_provider = multiversx_sc_abi_derive_common::contract::abi_gen::generate_abi_provider(
        &contract,
        true,
        multiversx_sc_abi_derive_common::TypeAbiImportCrate::MultiversxScAbi,
        &quote::quote! { multiversx_sc_abi::ContractAbiProvider },
        quote::quote! {},
    );
    // `Self::Api` never appears in a framework-agnostic trait's argument/return types (there is
    // no managed-type substitution step here), so this placeholder is never actually spliced in.
    let unused_self_api_replacement: syn::Path = syn::parse_quote! { ::core::convert::Infallible };
    let abi_proxy = match &call_arg.proxy_name {
        Some(proxy_name) => {
            multiversx_sc_abi_derive_common::contract::proxy_gen::generate_abi_proxy(
                &contract,
                multiversx_sc_abi_derive_common::TypeAbiImportCrate::MultiversxScAbi,
                proxy_name,
                &unused_self_api_replacement,
            )
        }
        None => quote::quote! {},
    };

    proc_macro::TokenStream::from(quote::quote! {
        #abi_provider

        #abi_proxy
    })
}

#[proc_macro_derive(TypeAbi)]
pub fn type_abi_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    multiversx_sc_abi_derive_common::type_abi_derive(
        input.into(),
        multiversx_sc_abi_derive_common::TypeAbiImportCrate::MultiversxScAbi,
    )
    .into()
}

#[proc_macro_attribute]
pub fn type_abi(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    assert!(args.is_empty(), "#[type_abi] attribute takes no arguments");
    multiversx_sc_abi_derive_common::type_abi_full(
        input.into(),
        multiversx_sc_abi_derive_common::TypeAbiImportCrate::MultiversxScAbi,
    )
    .into()
}
