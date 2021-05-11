use super::MethodPayableMetadata;

#[derive(Clone, Debug)]
pub struct InitMetadata {
	pub payable: MethodPayableMetadata,
}

#[derive(Clone, Debug)]
pub struct EndpointMetadata {
	pub public_name: syn::Ident,
	pub payable: MethodPayableMetadata,
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
