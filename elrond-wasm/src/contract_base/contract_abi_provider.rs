// use super::{ErrorApi, ManagedTypeApi, SendApi, StorageReadApi, StorageWriteApi};
use crate::{abi::ContractAbi, api::VMApi};

/// Required by contract ABI generators.
/// Provides the same associated types as the `ContractBase`,
/// so that associated types that show up in arguments and results match.
pub trait ContractAbiProvider {
    type Api: VMApi;

    /// Associated function that provides the contract or module ABI.
    /// Since ABI generation is static, no state from the contract is required.
    fn abi() -> ContractAbi;
}
