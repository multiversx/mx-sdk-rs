use multiversx_sc::imports::*;

use crate::{
    DepositKey,
    digital_cash_err_msg::*,
    storage::{self, DepositInfo, Fee},
};
#[multiversx_sc::module]
pub trait HelpersModule: storage::StorageModule {
    fn send_fee_to_address(&self, fee: &Payment, address: &ManagedAddress) {
        self.tx().to(address).payment(fee).transfer();
    }

    fn get_fee_for_token(&self, token: &TokenId) -> BigUint {
        require!(
            self.whitelisted_fee_tokens().contains(token),
            "invalid fee token provided"
        );
        let fee_mapper = self.fee(token);
        fee_mapper.get()
    }

    fn make_fund(
        &self,
        payment: ManagedVec<Payment>,
        deposit_key: DepositKey<Self::Api>,
        expiration: TimestampMillis,
    ) {
        let deposit_mapper = self.deposit(&deposit_key);

        deposit_mapper.update(|deposit| {
            require!(deposit.funds.is_empty(), "key already used");
            let num_tokens = payment.len();
            deposit.fees.num_token_to_transfer += num_tokens;
            deposit.expiration = expiration;
            deposit.funds = payment;
        });
    }

    fn check_fees_cover_number_of_tokens(
        &self,
        num_tokens: usize,
        fee: &BigUint,
        paid_fee: &BigUint,
    ) {
        require!(num_tokens > 0, "amount must be greater than 0");
        require!(
            fee * num_tokens as u64 <= *paid_fee,
            CANNOT_DEPOSIT_FUNDS_ERR_MSG
        );
    }

    fn update_fees(
        &self,
        caller_address: ManagedAddress,
        deposit_key: &DepositKey<Self::Api>,
        payment: Payment,
    ) {
        self.get_fee_for_token(&payment.token_identifier);
        let deposit_mapper = self.deposit(deposit_key);
        if !deposit_mapper.is_empty() {
            deposit_mapper.update(|deposit| {
                require!(
                    deposit.depositor_address == caller_address,
                    "invalid depositor address"
                );
                require!(
                    deposit.fees.value.token_identifier == payment.token_identifier,
                    "can only have 1 type of token as fee"
                );
                deposit.fees.value.amount += payment.amount;
            });
            return;
        }

        let new_deposit = DepositInfo {
            depositor_address: caller_address,
            funds: ManagedVec::new(),
            expiration: TimestampMillis::zero(),
            fees: Fee {
                num_token_to_transfer: 0,
                value: payment,
            },
        };
        deposit_mapper.set(new_deposit);
    }

    /// Deducts the specified fee amount and adds it to the collected fees for the token.
    fn deduct_and_collect_fees(&self, fee_token: &TokenId, fee_amount: BigUint) {
        self.collected_fees(fee_token)
            .update(|collected| *collected += fee_amount);
    }

    /// Calculates total fees for payments, considering whether the first payment covers fees.
    ///
    /// Returns two values:
    /// 1. `total_fee_with_first`: The fee required if all payments (including the first) are deposited as funds.
    ///    This is used when the first payment is large enough to cover both the fee AND serve as a fund.
    ///    Formula: fee_per_token × num_payments
    /// 2. `total_fee_without_first`: The fee required if only the remaining payments (excluding the first) are deposited.
    ///    This is used when the first payment is dedicated solely as the fee payment.
    ///    Formula: fee_per_token × (num_payments - 1)
    fn calculate_fee_adjustments(
        &self,
        num_payments: usize,
        fee_per_token: &BigUint,
    ) -> (BigUint, BigUint) {
        let total_fee_with_first = fee_per_token * num_payments as u64;
        let total_fee_without_first = fee_per_token * (num_payments as u64 - 1);
        (total_fee_with_first, total_fee_without_first)
    }
}
