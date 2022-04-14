#![no_std]

mod fee;
use fee::*;

elrond_wasm::imports!();
#[elrond_wasm::contract]
pub trait EsdtTransferWithFee {
    #[init]
    fn init(&self) {}

    #[only_owner]
    #[endpoint(setFeeForToken)]
    fn set_fee_for_token(&self, fee: Fee<Self::Api>, token: TokenIdentifier) {
        self.token_fee(&token).set(fee);
    }

    #[only_owner]
    #[endpoint(claimFees)]
    fn claim_fees(&self) {
        let mut fees = ManagedVec::new();
        for ((token, nonce), amount) in self.paid_fees().iter() {
            fees.push(EsdtTokenPayment::new(token, nonce, amount))
        }
        self.paid_fees().clear();

        let caller = self.blockchain().get_caller();
        self.send().direct_multi(&caller, &fees, &[]);
    }

    #[payable("*")]
    #[endpoint(transfer)]
    fn transfer(&self, address: ManagedAddress) {
        let payments = self.call_value().all_esdt_transfers();

        let mut fees = ManagedVec::<Self::Api, EsdtTokenPayment<Self::Api>>::new();
        for payment in &payments {
            let calculated_fees = self.get_fee(payment);
            if calculated_fees.amount > 0 {
                fees.push(calculated_fees);
            }
        }

        for fee in fees.iter() {
            let mut perceived_tax = false;
            for mut payment in &payments {
                if payment.token_identifier == fee.token_identifier {
                    require!(
                        payment.amount >= fee.amount,
                        "Insufficient payments for covering fees"
                    );

                    self.paid_fees()
                        .entry((payment.token_identifier.clone(), payment.token_nonce))
                        .or_insert(0u64.into())
                        .update(|value| *value += fee.amount.clone());

                    payment.amount -= fee.amount.clone();
                    perceived_tax = true;
                    break;
                }
            }

            require!(perceived_tax, "Fee payment missing");
        }

        self.send().direct_multi(&address, &payments, &[]);
    }

    fn get_fee(&self, mut payment: EsdtTokenPayment<Self::Api>) -> EsdtTokenPayment<Self::Api> {
        let fee_mapper = self.token_fee(&payment.token_identifier);
        if fee_mapper.is_empty() {
            payment.amount = 0u64.into();
            payment
        } else {
            self.calculate_fee(fee_mapper.get(), payment)
        }
    }

    fn calculate_fee(
        &self,
        fee: Fee<Self::Api>,
        mut provided: EsdtTokenPayment<Self::Api>,
    ) -> EsdtTokenPayment<Self::Api> {
        let calculated_fee;
        match fee {
            Fee::ExactValue(requested) => calculated_fee = requested,

            Fee::Percentage(percentage) => {
                let calculated_fee_amount =
                    provided.amount.clone() * percentage / PERCENTAGE_DIVISOR;
                provided.amount = calculated_fee_amount;
                calculated_fee = provided;
            },
        }
        calculated_fee
    }

    #[view(getTokenFee)]
    #[storage_mapper("token_fee")]
    fn token_fee(&self, token: &TokenIdentifier) -> SingleValueMapper<Fee<Self::Api>>;

    #[view(getPaidFees)]
    #[storage_mapper("paid_fees")]
    fn paid_fees(&self) -> MapMapper<(TokenIdentifier, u64), BigUint>;
}
