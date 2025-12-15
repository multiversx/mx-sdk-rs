use multiversx_sc::imports::*;

use crate::{constants::*, helpers, storage};

#[multiversx_sc::module]
pub trait PayFeeAndFund: storage::StorageModule + helpers::HelpersModule {
    #[endpoint(payFeeAndFund)]
    #[payable]
    fn pay_fee_and_fund(&self, address: ManagedAddress, valability: u64) {
        let mut payments = self.call_value().all().clone_value();
        require!(!payments.is_empty(), "no payment was provided");

        let mut fee_token = payments.get(0).clone();
        let provided_fee_token = payments.get(0).clone();

        fee_token.amount = self.get_fee_for_token(&fee_token.token_identifier);
        let nr_of_payments = payments.len();

        let fee_with_first_token = fee_token.amount.clone() * nr_of_payments as u32;
        let fee_without_first_token = fee_token.amount.clone() * (nr_of_payments as u32 - 1);

        require!(
            (provided_fee_token.amount == fee_without_first_token || // case when the first token is the exact fee amount
                provided_fee_token.amount > fee_with_first_token), // case when the first token also covers part of the funds
            "payment not covering fees"
        );

        if provided_fee_token.amount > fee_without_first_token {
            fee_token.amount = fee_with_first_token;
            let extracted_fee = Payment::new(
                provided_fee_token.token_identifier,
                provided_fee_token.token_nonce,
                provided_fee_token.amount - &fee_token.amount,
            );
            let _ = payments.set(0, extracted_fee);
        } else {
            payments.remove(0);
            fee_token.amount = fee_without_first_token;
        }

        let caller_address = self.blockchain().get_caller();
        self.update_fees(caller_address, &address, fee_token);

        self.make_fund(payments, address, valability)
    }

    #[endpoint]
    #[payable]
    fn fund(&self, address: ManagedAddress, valability: u64) {
        require!(!self.deposit(&address).is_empty(), FEES_NOT_COVERED_ERR_MSG);
        let deposit_mapper = self.deposit(&address).get();
        let depositor = deposit_mapper.depositor_address;
        require!(
            self.blockchain().get_caller() == depositor,
            "invalid depositor"
        );
        let deposited_fee_token = deposit_mapper.fees.value;
        let fee_amount = self.fee(&deposited_fee_token.token_identifier).get();

        let payment = self.call_value().all().clone_value();

        let num_tokens = payment.len();
        self.check_fees_cover_number_of_tokens(num_tokens, fee_amount, deposited_fee_token.amount);

        self.make_fund(payment, address, valability);
    }

    #[endpoint(depositFees)]
    #[payable("EGLD")]
    fn deposit_fees(&self, address: &ManagedAddress) {
        let payment = self.call_value().single().clone();
        let caller_address = self.blockchain().get_caller();
        self.update_fees(caller_address, address, payment);
    }
}
