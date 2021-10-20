use super::MethodPayableMetadata;

#[derive(Clone, Debug)]
pub struct InitMetadata {
    pub payable: MethodPayableMetadata,
}

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

#[derive(Clone, Debug)]
pub struct EndpointMetadata {
    pub public_name: syn::Ident,
    pub payable: MethodPayableMetadata,
    pub only_owner: bool,
    pub mutability: EndpointMutabilityMetadata,
}

#[derive(Clone, Debug)]
pub struct CallbackMetadata {
    pub callback_name: syn::Ident,
}

/// Method visibility from the point of view of the smart contract
#[derive(Clone, Debug)]
pub enum PublicRole {
    /// The smart contract constructor. There can be only one.
    Init(InitMetadata),

    /// Means it gets a smart contract function generated for it
    Endpoint(EndpointMetadata),

    Callback(CallbackMetadata),

    CallbackRaw,

    /// Can only called from within the smart contract.
    Private,
}
