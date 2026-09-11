use crate::{abi::ContractAbi, api::VMApi};

/// Required by contract ABI generators.
/// Provides the same associated types as the `ContractBase`,
/// so that associated types that show up in arguments and results match.
///
/// Shaped like `multiversx_sc_abi::ContractAbiProvider`, the framework-agnostic
/// equivalent used by contracts built directly against pure ABI types, but keeps
/// its own `Api` associated type: the generated `fn abi()` body references `Self::Api`
/// (from managed types such as `BigUint<Self::Api>`), which only resolves when
/// `Api` is declared on the very trait `fn abi()` is implemented for.
pub trait ContractAbiProvider {
    type Api: VMApi;

    /// Associated function that provides the contract or module ABI.
    /// Since ABI generation is static, no state from the contract is required.
    fn abi() -> ContractAbi;
}
