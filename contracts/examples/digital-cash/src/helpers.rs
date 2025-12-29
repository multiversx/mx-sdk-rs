use multiversx_sc::imports::*;

use crate::{
    constants::*,
    deposit_info::{DepositInfo, Fee},
    storage,
};
#[multiversx_sc::module]
pub trait HelpersModule: storage::StorageModule {
    fn send_fee_to_address(&self, fee: &Payment, address: &ManagedAddress) {
        self.tx().to(address).payment(fee).transfer();
    }

    fn get_expiration_round(&self, valability: u64) -> u64 {
        let valability_rounds = valability / SECONDS_PER_ROUND;
        self.blockchain().get_block_round() + valability_rounds
    }

    fn get_fee_for_token(&self, token: &TokenId) -> BigUint {
        require!(
            self.whitelisted_fee_tokens().contains(token),
            "invalid fee token provided"
        );
        let fee_mapper = self.fee(token);
        fee_mapper.get()
    }

    fn make_fund(&self, payment: ManagedVec<Payment>, address: ManagedAddress, valability: u64) {
        let deposit_mapper = self.deposit(&address);

        deposit_mapper.update(|deposit| {
            require!(deposit.funds.is_empty(), "key already used");
            let num_tokens = payment.len();
            deposit.fees.num_token_to_transfer += num_tokens;
            deposit.valability = valability;
            deposit.expiration_round = self.get_expiration_round(valability);
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
        address: &ManagedAddress,
        payment: Payment,
    ) {
        self.get_fee_for_token(&payment.token_identifier);
        let deposit_mapper = self.deposit(address);
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
            valability: 0,
            expiration_round: 0,
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
    fn calculate_fee_adjustments(
        &self,
        payments: &ManagedVec<Payment>,
        fee_per_token: &BigUint,
    ) -> (BigUint, BigUint, usize) {
        let num_payments = payments.len();
        let total_fee_with_first = fee_per_token * num_payments as u64;
        let total_fee_without_first = fee_per_token * (num_payments as u64 - 1);
        (total_fee_with_first, total_fee_without_first, num_payments)
    }
}
