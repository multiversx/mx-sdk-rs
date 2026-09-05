// ensure we don't run out of macro stack
#![recursion_limit = "1024"]

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
