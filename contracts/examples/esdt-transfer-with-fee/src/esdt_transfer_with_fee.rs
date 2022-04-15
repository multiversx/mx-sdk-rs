#![no_std]

mod fee;
use fee::*;

elrond_wasm::imports!();
#[elrond_wasm::contract]
pub trait EsdtTransferWithFee {
    #[init]
    fn init(&self) {}

    #[only_owner]
    #[endpoint(setExactValueFee)]
    fn set_exact_value_fee(&self, fee: EsdtTokenPayment<Self::Api>, token: TokenIdentifier) {
        self.token_fee(&token).set(Fee::ExactValue(fee));
    }

    #[only_owner]
    #[endpoint(setPercentageFee)]
    fn set_percentage_fee(&self, fee: u32, token: TokenIdentifier) {
        self.token_fee(&token).set(Fee::Percentage(fee));
    }

    #[only_owner]
    #[endpoint(claimFees)]
    fn claim_fees(&self) {
        let paid_fees = self.paid_fees();
        require!(!paid_fees.is_empty(), "There is nothing to claim");
        let mut fees = ManagedVec::new();
        for ((token, nonce), amount) in self.paid_fees().iter() {
            fees.push(EsdtTokenPayment::new(token, nonce, amount))
        }
        self.paid_fees().clear();

        let caller = self.blockchain().get_caller();
        self.send().direct_multi(&caller, &fees, &[]);
    }

    #[payable("*")]
    #[endpoint]
    fn transfer(&self, address: ManagedAddress) {
        let payments = self.call_value().all_esdt_transfers();
        let mut new_payments = ManagedVec::new();

        let mut payments_iter = payments.iter();
        while let Some(payment) = payments_iter.next() {
            let fee_type = self.get_fee_type(&payment);
            match &fee_type {
                Fee::ExactValue(fee) => {
                    let next_payment = payments_iter
                        .next()
                        .unwrap_or_else(|| sc_panic!("Fee payment missing"));
                    require!(
                        next_payment.token_identifier == fee.token_identifier
                            && next_payment.token_nonce == fee.token_nonce,
                        "Fee payment missing"
                    );
                    require!(
                        next_payment.amount == fee.amount,
                        "Insufficient payments for covering fees"
                    );

                    self.get_payment_after_fees(fee_type, &next_payment);
                    new_payments.push(payment);
                },
                Fee::Percentage(_) => {
                    self.get_payment_after_fees(fee_type, &payment);
                    new_payments.push(payment);
                },
                Fee::Unset => {
                    new_payments.push(payment);
                },
            }
        }
        self.send().direct_multi(&address, &new_payments, &[]);
    }

    fn get_payment_after_fees(
        &self,
        fee: Fee<Self::Api>,
        payment: &EsdtTokenPayment<Self::Api>,
    ) -> EsdtTokenPayment<Self::Api> {
        let mut new_payment = payment.clone();
        let fee_payment = self.calculate_fee(&fee, payment.clone());

        self.paid_fees()
            .entry((
                new_payment.token_identifier.clone(),
                new_payment.token_nonce,
            ))
            .or_insert(0u64.into())
            .update(|value| *value += &fee_payment.amount);

        new_payment.amount -= &fee_payment.amount;
        new_payment
    }

    fn get_fee_type(&self, payment: &EsdtTokenPayment<Self::Api>) -> Fee<Self::Api> {
        let fee_mapper = self.token_fee(&payment.token_identifier);
        if fee_mapper.is_empty() {
            Fee::Unset
        } else {
            fee_mapper.get()
        }
    }

    fn calculate_fee(
        &self,
        fee: &Fee<Self::Api>,
        mut provided: EsdtTokenPayment<Self::Api>,
    ) -> EsdtTokenPayment<Self::Api> {
        match fee {
            Fee::ExactValue(requested) => requested.clone(),
            Fee::Percentage(percentage) => {
                let calculated_fee_amount = &provided.amount * *percentage / PERCENTAGE_DIVISOR;
                provided.amount = calculated_fee_amount;
                provided
            },
            Fee::Unset => {
                provided.amount = BigUint::zero();
                provided
            },
        }
    }

    #[view(getTokenFee)]
    #[storage_mapper("token_fee")]
    fn token_fee(&self, token: &TokenIdentifier) -> SingleValueMapper<Fee<Self::Api>>;

    #[view(getPaidFees)]
    #[storage_mapper("paid_fees")]
    fn paid_fees(&self) -> MapMapper<(TokenIdentifier, u64), BigUint>;
}
