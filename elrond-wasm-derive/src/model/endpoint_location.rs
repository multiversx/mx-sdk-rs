#[derive(Clone, Debug)]
pub enum EndpointLocationMetadata {
    MainContract,
    ViewContract,
}

impl EndpointLocationMetadata {
    pub fn to_tokens(&self) -> proc_macro2::TokenStream {
        match self {
            EndpointLocationMetadata::MainContract => {
                quote! { elrond_wasm::abi::EndpointLocationAbi{location: "main"} }
            },
            EndpointLocationMetadata::ViewContract => {
                quote! { elrond_wasm::abi::EndpointLocationAbi{location: "view"} }
            },
        }
    }
}
