use multiversx_sc::imports::*;

use crate::{constants::*, helpers, storage};

#[multiversx_sc::module]
pub trait PayFeeAndFund: storage::StorageModule + helpers::HelpersModule {
    /// Pays the required fee and funds a deposit for the given address with specified valability.
    #[endpoint(payFeeAndFund)]
    #[payable]
    fn pay_fee_and_fund(&self, address: ManagedAddress, valability: u64) {
        let mut payments = self.call_value().all().clone_value();
        require!(!payments.is_empty(), "no payment was provided");

        let input_fee_token = payments.get(0).clone();
        let fee_per_token = self.get_fee_for_token(&input_fee_token.token_identifier);
        let (total_fee_with_first, total_fee_without_first, _num_payments) =
            self.calculate_fee_adjustments(&payments, &fee_per_token);
        require!(
            input_fee_token.amount.as_big_uint() == &total_fee_without_first
                || input_fee_token.amount.as_big_uint() > &total_fee_with_first,
            "payment not covering fees"
        );

        // Adjust payments
        let mut fee_payment_for_deposit = input_fee_token.clone();
        if input_fee_token.amount.as_big_uint() > &total_fee_without_first {
            fee_payment_for_deposit.amount =
                NonZeroBigUint::new(total_fee_with_first.clone()).unwrap();
            if input_fee_token.amount.as_big_uint() > &total_fee_with_first {
                let fund_from_fee_payment = Payment::new(
                    input_fee_token.token_identifier,
                    input_fee_token.token_nonce,
                    input_fee_token.amount - &fee_payment_for_deposit.amount,
                );
                let _ = payments.set(0, fund_from_fee_payment);
            } else {
                payments.remove(0);
            }
        } else {
            payments.remove(0);
            fee_payment_for_deposit.amount = NonZeroBigUint::new(total_fee_without_first).unwrap();
        }

        let caller_address = self.blockchain().get_caller();
        self.update_fees(caller_address, &address, fee_payment_for_deposit);
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
        self.check_fees_cover_number_of_tokens(
            num_tokens,
            &fee_amount,
            deposited_fee_token.amount.as_big_uint(),
        );

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
