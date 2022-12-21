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
                quote! { mx_sc::abi::EndpointMutabilityAbi::Mutable }
            },
            EndpointMutabilityMetadata::Readonly => {
                quote! { mx_sc::abi::EndpointMutabilityAbi::Readonly }
            },
            EndpointMutabilityMetadata::_Pure => {
                quote! { mx_sc::abi::EndpointMutabilityAbi::Pure }
            },
        }
    }
}
