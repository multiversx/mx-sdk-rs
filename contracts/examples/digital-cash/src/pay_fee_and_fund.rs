use multiversx_sc::imports::*;

use crate::{
    digital_cash_deposit,
    storage::{self, DepositInfo, DepositKey},
};

#[multiversx_sc::module]
pub trait PayFeeAndFund: storage::StorageModule + digital_cash_deposit::DepositModule {
    /// Pays the required fee and funds a deposit in one transaction.
    ///
    /// # Preconditions
    /// - At least one payment must be provided
    /// - If fees are enabled: first payment is taken entirely as fee, remaining are funds
    /// - If fees are disabled: all payments are treated as funds
    /// - If updating existing deposit: caller must be the original depositor
    /// - Fee token must match existing fee token if deposit already has fees
    ///
    /// # Requirements
    /// - Must be called with at least one payment (fee and/or funds)
    /// - If fees enabled and creating new deposit: first payment amount must be >= (base_fee × number_of_funds)
    /// - If updating existing deposit: total fees must be >= (base_fee × total_number_of_funds)
    ///
    /// # Outcomes
    /// - If deposit doesn't exist: creates new deposit with caller as depositor
    /// - If deposit exists: appends funds and fees to existing deposit, updates expiration
    /// - First payment is consumed as fee (if fees enabled)
    /// - Remaining payments are stored as funds
    ///
    /// # Panics
    /// - "no payment was provided" if called without payment
    /// - "invalid depositor" if updating existing deposit and caller is not the depositor
    /// - "fee token mismatch" if fee token differs from existing deposit's fee token
    /// - "insufficient fees provided" if total fees don't cover all funds
    /// - "invalid fee token" if fee token is not configured in contract
    #[endpoint(payFeeAndFund)]
    #[payable]
    fn pay_fee_and_fund(&self, deposit_key: DepositKey<Self::Api>, expiration: TimestampMillis) {
        let mut payments = self.call_value().all().clone_value();
        require!(!payments.is_empty(), "no payment was provided");

        let opt_fees = if !self.fees_disabled().get() {
            Some(payments.take(0).fungible_or_panic())
        } else {
            None
        };
        let funds = payments;

        let caller_address = self.blockchain().get_caller();

        let deposit_mapper = self.deposit(&deposit_key);
        if deposit_mapper.is_empty() {
            let new_deposit = DepositInfo {
                depositor_address: caller_address,
                funds,
                expiration,
                fees: opt_fees,
            };
            self.validate_deposit_fees(&new_deposit);
            deposit_mapper.set(new_deposit);
        } else {
            deposit_mapper.update(|deposit| {
                self.require_deposit_caller_is_depositor(&caller_address, deposit);

                if let Some(fees) = opt_fees {
                    self.add_deposit_fee(deposit, fees);
                }
                deposit.expiration = expiration;
                deposit.funds.append_vec(funds);
                self.validate_deposit_fees(deposit);
            });
        }
    }

    /// Adds funds to an existing deposit without paying additional fees.
    ///
    /// # Preconditions
    /// - Deposit must already exist for the given deposit_key
    /// - Caller must be the original depositor
    /// - Existing deposit must have sufficient fees to cover the new total number of funds
    /// - At least one payment must be provided
    ///
    /// # Requirements
    /// - Must be called with payment (the funds to add)
    /// - Only the depositor who created the deposit can call this
    /// - Deposit must have been created first (via payFeeAndFund or depositFees)
    ///
    /// # Outcomes
    /// - Payments are appended to the deposit's funds
    /// - Expiration timestamp is updated to the new value
    /// - No new fees are collected (uses existing fees)
    ///
    /// # Panics
    /// - "deposit needs to exist before funding, with fees paid" if deposit doesn't exist
    /// - "invalid depositor" if caller is not the original depositor
    /// - "insufficient fees provided" if existing fees don't cover total funds after addition
    #[endpoint]
    #[payable]
    fn fund(&self, deposit_key: DepositKey<Self::Api>, expiration: TimestampMillis) {
        let payment = self.call_value().all().clone_value();
        let caller_address = self.blockchain().get_caller();
        let deposit_mapper = self.deposit(&deposit_key);

        require!(
            !deposit_mapper.is_empty(),
            "deposit needs to exist before funding, with fees paid"
        );
        self.perform_append_funds(&deposit_mapper, &caller_address, expiration, payment);
    }

    /// Deposits fees for a new or existing deposit without adding funds.
    ///
    /// This allows paying fees in advance before adding the actual funds,
    /// which is required for the forward operation's destination deposit.
    ///
    /// # Preconditions
    /// - Exactly one payment must be provided (the fee)
    /// - If updating existing deposit: caller must be the original depositor
    /// - Fee token must match existing fee token if deposit already has fees
    ///
    /// # Requirements
    /// - Must be called with a single fungible payment
    /// - If updating existing deposit: only the depositor can add more fees
    ///
    /// # Outcomes
    /// - If deposit doesn't exist: creates new deposit with empty funds and zero expiration
    /// - If deposit exists: adds fee amount to existing fees (must be same token)
    /// - Caller is recorded as depositor for new deposits
    ///
    /// # Panics
    /// - "invalid depositor" if updating existing deposit and caller is not the depositor
    /// - "fee token mismatch" if fee token differs from existing deposit's fee token
    #[endpoint(depositFees)]
    #[payable]
    fn deposit_fees(&self, deposit_key: &DepositKey<Self::Api>) {
        let payment = self.call_value().single().clone().fungible_or_panic();
        let caller_address = self.blockchain().get_caller();

        let deposit_mapper = self.deposit(deposit_key);
        if deposit_mapper.is_empty() {
            let new_deposit = DepositInfo {
                depositor_address: caller_address,
                funds: ManagedVec::new(),
                expiration: TimestampMillis::zero(),
                fees: Some(payment),
            };
            deposit_mapper.set(new_deposit);
        } else {
            deposit_mapper.update(|deposit| {
                self.require_deposit_caller_is_depositor(&caller_address, deposit);
                self.add_deposit_fee(deposit, payment);
            });
        }
    }
}
