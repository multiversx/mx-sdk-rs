#![no_std]

mod fee;
use fee::*;

elrond_wasm::imports!();
#[elrond_wasm::derive::contract]
pub trait EsdtTransferWithFee {
    #[init]
    fn init(&self) {}

    #[only_owner]
    #[endpoint]
    fn set_general_fee(&self, fee: Fee<Self::Api>) {
        self.general_fee().set(fee);
    }

    #[only_owner]
    #[endpoint]
    fn set_fee_for_token(&self, fee: Fee<Self::Api>, token: TokenIdentifier) {
        self.specific_fee(&token).set(fee);
    }

    #[payable("*")]
    #[endpoint]
    fn transfer(&self, address: ManagedAddress) {
        require!(!self.general_fee().is_empty(), "Fees are not set yet");
        let payments = self.call_value().all_esdt_transfers();
        if payments.is_empty() {
            return;
        }

        let mut fees = ManagedVec::<Self::Api, EsdtTokenPayment<Self::Api>>::new();
        for payment in &payments {
            fees.push(self.get_fee(payment));
        }

        for fee in &fees {
            for mut payment in &payments {
                if payment.token_identifier == fee.token_identifier {
                    require!(
                        payment.amount >= fee.amount,
                        "Insufficient payments for covering fees"
                    );
                    self.paid_fees(&payment.token_identifier)
                        .update(|value| *value += fee.amount.clone());
                    payment.amount -= fee.amount.clone();
                    break;
                }
            }
        }

        self.send().direct_multi(&address, &payments, &[]);
    }

    fn get_fee(&self, payment: EsdtTokenPayment<Self::Api>) -> EsdtTokenPayment<Self::Api> {
        if self.specific_fee(&payment.token_identifier).is_empty() {
            self.calculate_fee(self.general_fee().get(), payment)
        } else {
            self.calculate_fee(self.general_fee().get(), payment)
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

    #[view(getGeneralFee)]
    #[storage_mapper("general_fee")]
    fn general_fee(&self) -> SingleValueMapper<Fee<Self::Api>>;

    #[view(getSpecificFee)]
    #[storage_mapper("specific_fee")]
    fn specific_fee(&self, token: &TokenIdentifier) -> SingleValueMapper<Fee<Self::Api>>;

    #[view(getPaidFees)]
    #[storage_mapper("paid_fees")]
    fn paid_fees(&self, token: &TokenIdentifier) -> SingleValueMapper<BigUint>;
}
