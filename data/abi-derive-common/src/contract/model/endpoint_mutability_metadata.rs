#[derive(Debug, Clone)]
pub enum EndpointMutabilityMetadata {
    Mutable,
    Readonly,
    _Pure,
}

impl EndpointMutabilityMetadata {
    pub fn to_tokens(&self, import: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        match self {
            EndpointMutabilityMetadata::Mutable => {
                quote! { #import::EndpointMutabilityAbi::Mutable }
            }
            EndpointMutabilityMetadata::Readonly => {
                quote! { #import::EndpointMutabilityAbi::Readonly }
            }
            EndpointMutabilityMetadata::_Pure => {
                quote! { #import::EndpointMutabilityAbi::Pure }
            }
        }
    }
}
