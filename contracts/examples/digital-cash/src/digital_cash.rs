#![no_std]
#![allow(unused_attributes)]

use multiversx_sc::imports::*;

mod digital_cash_deposit;
pub mod digital_cash_proxy;
mod pay_fee_and_fund;
mod signature_operations;
mod storage;

#[multiversx_sc::contract]
pub trait DigitalCash:
    pay_fee_and_fund::PayFeeAndFund
    + signature_operations::SignatureOperationsModule
    + digital_cash_deposit::DepositModule
    + storage::StorageModule
{
    /// Initializes the digital cash contract with fee configuration.
    ///
    /// # Arguments
    /// * `fees_disabled` - Global toggle to disable fee requirements
    /// * `fee_variants` - List of (token_id, fee_amount) pairs to configure accepted fee tokens
    #[init]
    fn init(
        &self,
        fees_disabled: bool,
        fee_variants: MultiValueEncoded<MultiValue2<TokenId, BigUint>>,
    ) {
        self.set_fees_disabled(fees_disabled);
        for fee_variant in fee_variants.into_iter() {
            let (token_id, fee_amount) = fee_variant.into_tuple();
            self.set_fee(token_id, fee_amount);
        }
    }

    /// Enables or disables the fee requirement globally.
    ///
    /// # Preconditions
    /// - None
    ///
    /// # Requirements
    /// - Must be called by the contract owner
    ///
    /// # Outcomes
    /// - When set to `true`: fee validation is skipped for all operations
    /// - When set to `false`: fees are required based on configured fee variants
    /// - Affects all future deposit creation and claim operations
    ///
    /// # Panics
    /// - If caller is not the contract owner (enforced by #[only_owner])
    #[endpoint(setFeesDisabled)]
    #[only_owner]
    fn set_fees_disabled(&self, fees_disabled: bool) {
        self.fees_disabled().set(fees_disabled);
    }

    /// Updates the fee configuration for a specific token.
    ///
    /// This unified endpoint handles adding new fee tokens, updating existing fees,
    /// and removing fee tokens (by setting amount to zero).
    ///
    /// # Preconditions
    /// - None
    ///
    /// # Requirements
    /// - Must be called by the contract owner
    ///
    /// # Outcomes
    /// - If fee_amount > 0: token is configured as valid fee token with specified base fee
    /// - If fee_amount == 0: effectively disables fees for that token (will panic if used)
    /// - Fee is charged per fund: total_fee = base_fee × number_of_funds_in_deposit
    ///
    /// # Panics
    /// - If caller is not the contract owner (enforced by #[only_owner])
    #[endpoint(setFee)]
    #[only_owner]
    fn set_fee(&self, fee_token: TokenId, fee_amount: BigUint) {
        self.base_fee(&fee_token).set(fee_amount);
    }

    /// Withdraws all collected fees to the contract owner.
    ///
    /// # Preconditions
    /// - None (works even if no fees collected)
    ///
    /// # Requirements
    /// - Must be called by the contract owner
    ///
    /// # Outcomes
    /// - All collected fees across all tokens are transferred to the caller
    /// - Collected fees storage is cleared
    /// - If no fees collected, no transfer occurs (returns early)
    /// - Supports multiple tokens in a single multi-transfer transaction
    ///
    /// # Panics
    /// - If caller is not the contract owner (enforced by #[only_owner])
    #[endpoint(claimFees)]
    #[only_owner]
    fn claim_fees(&self) {
        let collected_fees = self.collected_fees().take();

        if collected_fees.is_empty() {
            return;
        }

        let mut payments = ManagedVec::new();
        for collected_fee in collected_fees.into_iter() {
            payments.push(collected_fee.into_payment());
        }

        let caller_address = self.blockchain().get_caller();
        self.tx().to(caller_address).payment(payments).transfer();
    }
}
