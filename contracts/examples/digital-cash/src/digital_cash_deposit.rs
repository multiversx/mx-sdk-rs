use multiversx_sc::imports::*;

use crate::storage::{self, CollectedFees, DepositInfo};
#[multiversx_sc::module]
pub trait DepositModule: storage::StorageModule {
    /// Validates that a deposit exists in storage.
    ///
    /// Panics with "non-existent key" if the deposit mapper is empty.
    fn require_deposit_exists(&self, deposit_mapper: &SingleValueMapper<DepositInfo<Self::Api>>) {
        require!(!deposit_mapper.is_empty(), "non-existent key");
    }

    /// Validates that the caller is the original depositor.
    ///
    /// Only the depositor who created a deposit can modify it (fund, add fees).
    /// Panics with "invalid depositor" if addresses don't match.
    fn require_deposit_caller_is_depositor(
        &self,
        caller_address: &ManagedAddress,
        deposit: &DepositInfo<Self::Api>,
    ) {
        require!(
            deposit.depositor_address == *caller_address,
            "invalid depositor"
        );
    }

    /// Validates that token matches, then adds the new fee to the existing one.
    fn add_deposit_fee(&self, deposit_info: &mut DepositInfo<Self::Api>, new_fee: FungiblePayment) {
        if let Some(existing_fee) = &mut deposit_info.fees {
            require!(
                existing_fee.token_identifier == new_fee.token_identifier,
                "fee token mismatch"
            );
            existing_fee.amount += &new_fee.amount;
        } else {
            deposit_info.fees = Some(new_fee);
        }
    }

    /// Appends funds to an existing deposit and updates the expiration time.
    ///
    /// Validates that the caller is the depositor and that fees are sufficient
    /// for the updated number of funds.
    fn perform_append_funds(
        &self,
        deposit: &mut DepositInfo<Self::Api>,
        caller_address: &ManagedAddress,
        expiration: TimestampMillis,
        funds: ManagedVec<Self::Api, Payment>,
    ) {
        self.require_deposit_caller_is_depositor(caller_address, deposit);

        deposit.expiration = expiration;
        deposit.funds.append_vec(funds);

        self.validate_deposit_fees(deposit);
    }

    /// Adds a fee payment to the contract's collected fees.
    ///
    /// Fees are aggregated by token and can be claimed by the contract owner.
    fn add_collected_fee(&self, payment: &FungiblePayment) {
        self.collected_fees().update(|fees| {
            add_collected_fee(fees, payment);
        });
    }

    /// Validates that a deposit has sufficient fees for its number of funds.
    ///
    /// If fees are disabled globally, this check is skipped.
    /// Required fee = base fee per token × number of funds in the deposit.
    fn validate_deposit_fees(&self, deposit: &DepositInfo<Self::Api>) {
        if self.fees_disabled().get() {
            return;
        }

        let fees = deposit
            .fees
            .as_ref()
            .unwrap_or_else(|| sc_panic!("no fees provided"));
        let required_fee = self.required_fees_for_deposit(&fees.token_identifier, deposit);

        require!(fees.amount >= required_fee, "insufficient fees provided");
    }

    /// Calculates the required fee amount for a deposit based on the number of funds.
    ///
    /// Returns `base_fee × number_of_funds` for the given token.
    /// Panics if the token is not configured as a valid fee token.
    fn required_fees_for_deposit(
        &self,
        token_id: &TokenId,
        deposit: &DepositInfo<Self::Api>,
    ) -> NonZeroBigUint {
        let num_funds = deposit.funds.len();
        let fee_amount_config = self.base_fee(token_id).get();
        let mut fee = fee_amount_config
            .into_non_zero()
            .unwrap_or_else(|| sc_panic!("invalid fee token"));
        fee *= num_funds as u64;
        fee
    }

    /// Returns fees to collect, then fees to return.
    fn take_fees_to_collect_and_leftover(
        &self,
        deposit: &mut DepositInfo<Self::Api>,
    ) -> (Option<FungiblePayment>, Option<FungiblePayment>) {
        let opt_fees = deposit.fees.take();
        if self.fees_disabled().get() {
            return (None, opt_fees);
        }

        let fees = opt_fees.unwrap_or_else(|| sc_panic!("no fees provided"));

        let required_fee = self.required_fees_for_deposit(&fees.token_identifier, deposit);

        match fees.amount.cmp(&required_fee) {
            core::cmp::Ordering::Less => sc_panic!("insufficient fees provided"),
            core::cmp::Ordering::Equal => {
                // exact fees provided
                (Some(fees), None)
            }
            core::cmp::Ordering::Greater => {
                // more than enough fees provided
                let leftover_amount = &fees.amount - &required_fee;
                let leftover = FungiblePayment::new(fees.token_identifier.clone(), leftover_amount);
                let fees_to_collect = FungiblePayment::new(fees.token_identifier, required_fee);
                (Some(fees_to_collect), Some(leftover))
            }
        }
    }
}

/// Adds a fee payment to the collected fees vector, aggregating by token.
///
/// If a fee entry for the token already exists, the amount is added to it.
/// Otherwise, a new entry is created.
fn add_collected_fee<M: ManagedTypeApi>(
    collected_fees: &mut CollectedFees<M>,
    payment: &FungiblePayment<M>,
) {
    // Find the index of existing fee entry with same token identifier
    if let Some(index) = collected_fees
        .iter()
        .position(|fee| fee.token_identifier == payment.token_identifier)
    {
        // Update existing fee amount
        collected_fees.get_mut(index).amount += &payment.amount;
    } else {
        // Add new fee entry if not found
        collected_fees.push(FungiblePayment::new(
            payment.token_identifier.clone(),
            payment.amount.clone(),
        ));
    }
}
