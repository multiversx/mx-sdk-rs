
#[derive(Debug, Clone)]
pub enum EndpointMutabilityMetadata {
    Mutable,
    Readonly,
    _Pure,
}

impl EndpointMutabilityMetadata {
    pub fn to_tokens(&self) -> proc_macro2::TokenStream {
        match self {
            EndpointMutabilityMetadata::Mutable => {
                quote! { elrond_wasm::abi::EndpointMutabilityAbi::Mutable }
            },
            EndpointMutabilityMetadata::Readonly => {
                quote! { elrond_wasm::abi::EndpointMutabilityAbi::Readonly }
            },
            EndpointMutabilityMetadata::_Pure => {
                quote! { elrond_wasm::abi::EndpointMutabilityAbi::Pure }
            },
        }
    }
}