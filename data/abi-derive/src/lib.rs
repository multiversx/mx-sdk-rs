// ensure we don't run out of macro stack
#![recursion_limit = "1024"]
// TODO: remove once minimum version is 1.87+
#![allow(unknown_lints)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::manual_is_multiple_of)]

mod parse;
mod type_abi_derive;

#[deprecated(
    since = "0.54.4",
    note = "Replace with attribute #[type_abi], which should be placed before all derives. More about this: https://docs.multiversx.com/developers/transactions/tx-migration/#replace-derivetypeabi-with-type_abi"
)]
#[proc_macro_derive(TypeAbi)]
pub fn type_abi_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    type_abi_derive::type_abi_derive(input).into()
}

#[proc_macro_attribute]
pub fn type_abi(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    assert!(args.is_empty(), "#[type_abi] attribute takes no args");
    type_abi_derive::type_abi_full(input).into()
}
