use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn api_imports(sc_crate_name: TokenStream) -> TokenStream {
    let crate_name = proc_macro2::TokenStream::from(sc_crate_name);
    let vm_api = quote! {multiversx_sc_wasm_adapter::api::VmApiImpl};
    let uncallable_api = quote! {#crate_name::api::uncallable::UncallableApi};
    let debug_api = quote! {multiversx_sc_scenario::DebugApi};
    let single_tx_api = quote! {multiversx_sc_scenario::api::SingleTxApi};

    let used_debug_api = if cfg!(feature = "single-tx-api") {
        single_tx_api
    } else if cfg!(feature = "no-debug-api") {
        uncallable_api
    } else {
        debug_api
    };

    let result = quote! {
        #[cfg(target_arch = "wasm32")]
        type CurrentApi = #vm_api;

        #[cfg(not(target_arch = "wasm32"))]
        type CurrentApi = #used_debug_api;
    };

    result.into()
}

