use crate::ContractAbi;

/// Required by contract ABI generators.
/// Framework-agnostic: implementors only need to provide the ABI description,
/// with no dependency on any particular VM API.
pub trait ContractAbiProvider {
    /// Associated function that provides the contract or module ABI.
    /// Since ABI generation is static, no state from the contract is required.
    fn abi() -> ContractAbi;
}
