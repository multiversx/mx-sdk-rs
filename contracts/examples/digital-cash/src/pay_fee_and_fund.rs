use multiversx_sc::imports::*;

use crate::{DepositKey, digital_cash_err_msg::*, helpers, storage};

#[multiversx_sc::module]
pub trait PayFeeAndFund: storage::StorageModule + helpers::HelpersModule {
    /// Pays the required fee and funds a deposit for the given address with specified expiration time.
    #[endpoint(payFeeAndFund)]
    #[payable]
    fn pay_fee_and_fund(&self, deposit_key: DepositKey<Self::Api>, expiration: TimestampMillis) {
        let mut payments = self.call_value().all().clone_value();
        require!(!payments.is_empty(), "no payment was provided");

        let payment_containing_fee = payments.get(0).clone();
        let fee_per_token = self.get_fee_for_token(&payment_containing_fee.token_identifier);
        let (total_fee_with_first, total_fee_without_first) =
            self.calculate_fee_adjustments(payments.len(), &fee_per_token);

        require!(
            payment_containing_fee.amount.as_big_uint() == &total_fee_without_first
                || payment_containing_fee.amount.as_big_uint() > &total_fee_with_first,
            "payment not covering fees"
        );

        // Adjust payments
        let mut fee_payment_for_deposit = payment_containing_fee.clone();
        if payment_containing_fee.amount.as_big_uint() > &total_fee_without_first {
            fee_payment_for_deposit.amount =
                NonZeroBigUint::new_or_panic(total_fee_with_first.clone());
            if payment_containing_fee.amount.as_big_uint() > &total_fee_with_first {
                let fund_from_fee_payment = Payment::new(
                    payment_containing_fee.token_identifier,
                    payment_containing_fee.token_nonce,
                    payment_containing_fee.amount - &fee_payment_for_deposit.amount,
                );
                let _ = payments.set(0, fund_from_fee_payment);
            } else {
                payments.remove(0);
            }
        } else {
            payments.remove(0);
            fee_payment_for_deposit.amount = NonZeroBigUint::new_or_panic(total_fee_without_first);
        }

        let caller_address = self.blockchain().get_caller();
        self.update_fees(caller_address, &deposit_key, fee_payment_for_deposit);
        self.make_fund(payments, deposit_key, expiration)
    }

    #[endpoint]
    #[payable]
    fn fund(&self, deposit_key: DepositKey<Self::Api>, expiration: TimestampMillis) {
        require!(
            !self.deposit(&deposit_key).is_empty(),
            FEES_NOT_COVERED_ERR_MSG
        );
        let deposit_mapper = self.deposit(&deposit_key).get();
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

        self.make_fund(payment, deposit_key, expiration);
    }

    #[endpoint(depositFees)]
    #[payable("EGLD")]
    fn deposit_fees(&self, deposit_key: &DepositKey<Self::Api>) {
        let payment = self.call_value().single().clone();
        let caller_address = self.blockchain().get_caller();
        self.update_fees(caller_address, deposit_key, payment);
    }
}
