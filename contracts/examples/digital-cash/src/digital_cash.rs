#![no_std]
#![allow(unused_attributes)]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

mod deposit_info;

use deposit_info::{DepositInfo, FundType, PaymentFunds};

pub const SECONDS_PER_ROUND: u64 = 6;
pub use multiversx_sc::api::{ED25519_KEY_BYTE_LEN, ED25519_SIGNATURE_BYTE_LEN};

#[multiversx_sc::contract]
pub trait DigitalCash {
    #[init]
    fn init(&self, fee: BigUint) {
        self.fee().set(fee);
    }

    //endpoints

    #[endpoint]
    #[payable("*")]
    fn fund(&self, address: ManagedAddress, valability: u64) {
        // egld or single esdt
        let payment = self.call_value().egld_or_single_esdt();

        let depositor_address = self.blockchain().get_caller();
        let fee = self.fee().get();

        require!(
            payment.amount > BigUint::zero(),
            "amount must be greater than 0"
        );

        self.payment(&depositor_address).update(|caller_fees| {
            require!(
                fee * caller_fees.num_token_transfer <= caller_fees.value,
                "cannot deposit funds without covering the fee cost first"
            );

            caller_fees.num_token_transfer += 1;
        });

        let fund_type = FundType {
            token: payment.token_identifier.clone(),
            nonce: payment.token_nonce,
        };

        let mut deposit = DepositInfo {
            depositor_address,
            payment,
            valability,
            expiration_round: self.get_expiration_round(valability),
        };

        if self.deposit(&address).contains_key(&fund_type) {
            self.deposit(&address).entry(fund_type).and_modify(|fund| {
                deposit.payment.amount += fund.payment.amount.clone();
                deposit.expiration_round = deposit.expiration_round.max(fund.expiration_round);
            });
        } else {
            self.deposit(&address).insert(fund_type, deposit);
        }
    }

    #[endpoint]
    fn withdraw(&self, address: ManagedAddress) {
        require!(!self.deposit(&address).is_empty(), "non-existent key");

        let mut withdrawed_tokens = ManagedVec::<Self::Api, FundType<Self::Api>>::new();
        let block_round = self.blockchain().get_block_round();
        let mut transfer_occured = false;
        let mut esdt_funds = ManagedVec::<Self::Api, EsdtTokenPayment<Self::Api>>::new();
        let mut egld_funds = BigUint::zero();
        for (key, deposit) in self.deposit(&address).iter() {
            if deposit.expiration_round < block_round {
                if deposit.payment.token_identifier.is_esdt() {
                    esdt_funds.push(deposit.payment.unwrap_esdt());
                } else {
                    egld_funds += deposit.payment.amount;
                }
                self.send().direct(
                    &deposit.depositor_address,
                    &deposit.payment.token_identifier,
                    deposit.payment.token_nonce,
                    &deposit.payment.amount,
                );
                transfer_occured = true;
                withdrawed_tokens.push(key);
            }
        }

        self.send()
            .direct_multi(&deposit.depositor_address, &esdt_funds);
        require!(transfer_occured, "withdrawal has not been available yet");

        for token in withdrawed_tokens.iter() {
            self.deposit(&address).remove(&token);
        }
    }

    #[endpoint]
    fn claim(
        &self,
        address: ManagedAddress,
        signature: ManagedByteArray<Self::Api, ED25519_SIGNATURE_BYTE_LEN>,
    ) {
        require!(!self.deposit(&address).is_empty(), "non-existent key");

        let caller_address = self.blockchain().get_caller();
        let fee = self.fee().get();
        self.require_signature(&address, &caller_address, signature);

        let mut withdrawed_tokens = ManagedVec::<Self::Api, FundType<Self::Api>>::new();
        let mut transfer_occured = false;
        let block_round = self.blockchain().get_block_round();

        for (key, deposit) in self.deposit(&address).iter() {
            if deposit.expiration_round >= block_round {
                self.send().direct(
                    &caller_address,
                    &deposit.payment.token_identifier,
                    deposit.payment.token_nonce,
                    &deposit.payment.amount,
                );
                transfer_occured = true;
                withdrawed_tokens.push(key);
            }
        }
        require!(transfer_occured, "deposit expired");

        self.payment(&caller_address).update(|caller_fees| {
            let num_tokens_transfered = withdrawed_tokens.len() as u64;
            let fee_cost = fee * num_tokens_transfered;

            caller_fees.num_token_transfer -= num_tokens_transfered;
            caller_fees.value -= fee_cost.clone();

            self.collected_fees()
                .update(|collected_fees| *collected_fees += fee_cost);
        });

        for token in withdrawed_tokens.iter() {
            self.deposit(&address).remove(&token);
        }
    }

    fn require_signature(
        &self,
        address: &ManagedAddress,
        caller_address: &ManagedAddress,
        signature: ManagedByteArray<Self::Api, ED25519_SIGNATURE_BYTE_LEN>,
    ) {
        let addr = address.as_managed_byte_array();
        let message = caller_address.as_managed_buffer();
        require!(
            self.crypto()
                .verify_ed25519_legacy_managed::<32>(addr, message, &signature),
            "invalid signature"
        );
    }

    #[endpoint]
    #[payable("EGLD")]
    fn payment_funds(&self) {
        let payment = self.call_value().egld_value();
        let caller_address = self.blockchain().get_caller();
        self.payment(&caller_address)
            .update(|payment_funds| payment_funds.value += payment);
    }

    #[endpoint]
    fn forward(
        &self,
        address: ManagedAddress,
        forward_address: ManagedAddress,
        signature: ManagedByteArray<Self::Api, ED25519_SIGNATURE_BYTE_LEN>,
    ) {
        let caller_address = self.blockchain().get_caller();
        let fee = self.fee().get();
        self.require_signature(&address, &caller_address, signature);

        let mut forwarded_tokens_number = 0u64;
        for (key, fund) in self.deposit(&address).iter() {
            let forwarded_fund = DepositInfo {
                depositor_address: fund.depositor_address,
                payment: fund.payment,
                valability: fund.valability,
                expiration_round: self.get_expiration_round(fund.valability),
            };
            self.deposit(&forward_address).insert(key, forwarded_fund);
            forwarded_tokens_number += 1;
        }

        self.payment(&caller_address).update(|caller_fees| {
            let fee_cost = &fee * forwarded_tokens_number;

            require!(
                &fee * caller_fees.num_token_transfer + &fee_cost <= caller_fees.value,
                "forward not permited due to uncovered fee costs by depositor"
            );

            caller_fees.value -= &fee_cost;

            self.collected_fees()
                .update(|collected_fees| *collected_fees += fee_cost);
        });

        self.deposit(&address).clear();
    }

    //views

    #[view(amount)]
    fn get_amount(
        &self,
        address: ManagedAddress,
        token: EgldOrEsdtTokenIdentifier,
        nonce: u64,
    ) -> BigUint {
        require!(!self.deposit(&address).is_empty(), "non-existent key");

        let data = self.deposit(&address).get(&FundType { token, nonce });
        let mut amount = BigUint::zero();
        if let Some(fund) = data {
            amount = fund.payment.amount;
        } else {
            require!(!self.deposit(&address).is_empty(), "non-existent key");
        }
        amount
    }

    //private functions

    fn get_expiration_round(&self, valability: u64) -> u64 {
        let valability_rounds = valability / SECONDS_PER_ROUND;
        self.blockchain().get_block_round() + valability_rounds
    }

    //storage

    #[view]
    #[storage_mapper("deposit")]
    fn deposit(
        &self,
        donor: &ManagedAddress,
    ) -> MapMapper<FundType<Self::Api>, DepositInfo<Self::Api>>;

    #[storage_mapper("payment")]
    fn payment(&self, donor: &ManagedAddress) -> SingleValueMapper<PaymentFunds<Self::Api>>;

    #[storage_mapper("fee")]
    fn fee(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("collected_fees")]
    fn collected_fees(&self) -> SingleValueMapper<BigUint>;
}
