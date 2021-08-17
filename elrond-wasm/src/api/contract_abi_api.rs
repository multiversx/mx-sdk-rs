use super::{ErrorApi, ManagedTypeApi, SendApi, StorageReadApi, StorageWriteApi};
use crate::abi::{ContractAbi, TypeAbi};

/// Required by contract ABI generators.
/// Provides the same associated types as the `ContractBase`,
/// so that associated types that show up in arguments and results match.
pub trait ContractAbiProvider {
    /// This associated type allows all managed types to make sense in the ABI context.
    type TypeManager: ManagedTypeApi + 'static;

    /// The generated ABI generation code uses the same types as the contract to provide `TypeAbi`s to endpoints.
    /// This associated type allows `Self::BigUint` to also make sense in the ABI context.
    type BigUint: TypeAbi;

    /// The generated ABI generation code uses the same types as the contract to provide `TypeAbi`s to endpoints.
    /// It sometimes references the contract storage manager type in with storage mappers,
    /// as for example in `SingleValueMapper<Self::Storage, i32>`.
    type Storage: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static;

    /// The generated ABI generation code uses the same types as the contract to provide `TypeAbi`s to endpoints.
    /// It is referenced by contract calls in general,
    /// as for example in `AsyncCall<Self::Send>`.
    type SendApi: SendApi<ProxyTypeManager = Self::TypeManager, AmountType = Self::BigUint>
        + Clone
        + 'static;

    /// Associated function that provides the contract or module ABI.
    /// Since ABI generation is static, no state from the contract is required.
    fn abi() -> ContractAbi;
}
