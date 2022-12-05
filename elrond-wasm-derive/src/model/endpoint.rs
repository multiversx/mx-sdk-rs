use super::{EndpointMutabilityMetadata, MethodPayableMetadata};

#[derive(Clone, Debug)]
pub struct InitMetadata {
    pub payable: MethodPayableMetadata,
}

#[derive(Clone, Debug)]
pub struct EndpointMetadata {
    pub public_name: syn::Ident,
    pub payable: MethodPayableMetadata,
    pub only_owner: bool,
    pub only_admin: bool,
    pub only_user_account: bool,
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

    CallbackPromise(CallbackMetadata),

    /// Can only called from within the smart contract.
    Private,
}
