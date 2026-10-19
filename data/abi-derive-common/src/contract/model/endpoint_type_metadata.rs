/// Used to generate the EndpointType in the ABI.
#[derive(Debug, Clone)]
pub enum EndpointTypeMetadata {
    Init,
    Upgrade,
    Endpoint,
    PromisesCallback,
}

impl EndpointTypeMetadata {
    pub fn to_tokens(&self, import: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        match self {
            EndpointTypeMetadata::Init => {
                quote! { #import::EndpointTypeAbi::Init }
            }
            EndpointTypeMetadata::Upgrade => {
                quote! { #import::EndpointTypeAbi::Upgrade }
            }
            EndpointTypeMetadata::Endpoint => {
                quote! { #import::EndpointTypeAbi::Endpoint }
            }
            EndpointTypeMetadata::PromisesCallback => {
                quote! { #import::EndpointTypeAbi::PromisesCallback }
            }
        }
    }
}
