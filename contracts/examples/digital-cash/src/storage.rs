use multiversx_sc::{derive_imports::*, imports::*};

pub type DepositKey<M> = ManagedByteArray<M, 32>;

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct DepositInfo<M: ManagedTypeApi> {
    pub depositor_address: ManagedAddress<M>,
    pub funds: ManagedVec<M, Payment<M>>,
    pub expiration: TimestampMillis,
    pub fees: Option<FungiblePayment<M>>,
}

pub type CollectedFees<M> = ManagedVec<M, FungiblePayment<M>>;

#[multiversx_sc::module]
pub trait StorageModule {
    /// Maps a deposit key (ED25519 public key) to its deposit information.
    ///
    /// Each deposit contains the depositor's address, funds, expiration timestamp, and fees.
    #[view]
    #[storage_mapper("deposit")]
    fn deposit(
        &self,
        deposit_key: &DepositKey<Self::Api>,
    ) -> SingleValueMapper<DepositInfo<Self::Api>>;

    /// Global toggle to enable or disable fee requirements.
    ///
    /// When true, no fees are required for deposit creation or claims.
    /// When false, fees are validated based on configured base fees.
    #[storage_mapper("feesDisabled")]
    fn fees_disabled(&self) -> SingleValueMapper<bool>;

    /// Base fee amount for a specific token.
    ///
    /// The total fee required for a deposit is calculated as:
    /// `total_fee = base_fee × number_of_funds`
    #[storage_mapper("baseFee")]
    fn base_fee(&self, token: &TokenId) -> SingleValueMapper<BigUint>;

    /// Fees collected from claims and forwards, aggregated by token.
    ///
    /// Stored as a simple vector for storage optimization.
    /// Not too many fee token types are expected, so linear search is acceptable.
    /// The contract owner can withdraw these fees using the `claimFees` endpoint.
    #[storage_mapper("collectedFees")]
    fn collected_fees(&self) -> SingleValueMapper<CollectedFees<Self::Api>>;
}
