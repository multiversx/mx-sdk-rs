use multiversx_sc::imports::*;

use crate::{digital_cash_err_msg::*, helpers, storage};

pub use multiversx_sc::api::ED25519_SIGNATURE_BYTE_LEN;

#[multiversx_sc::module]
pub trait SignatureOperationsModule: storage::StorageModule + helpers::HelpersModule {
    #[endpoint]
    fn withdraw(&self, address: ManagedAddress) {
        let deposit_mapper = self.deposit(&address);
        require!(!deposit_mapper.is_empty(), NON_EXISTENT_KEY_ERR_MSG);

        let deposit = deposit_mapper.take();
        let original_fee_payment = deposit.fees.value;

        require!(
            deposit.expiration < self.blockchain().get_block_timestamp_millis(),
            "cannot withdraw, deposit not expired yet"
        );

        let mut funds = deposit.funds;

        let fee_payment_to_return = Payment::new(
            original_fee_payment.token_identifier,
            0,
            original_fee_payment.amount,
        );
        funds.push(fee_payment_to_return);

        if !funds.is_empty() {
            self.tx()
                .to(&deposit.depositor_address)
                .payment(funds)
                .transfer();
        }
    }

    #[endpoint]
    fn claim(
        &self,
        address: ManagedAddress,
        signature: ManagedByteArray<Self::Api, ED25519_SIGNATURE_BYTE_LEN>,
    ) {
        let deposit_mapper = self.deposit(&address);
        require!(!deposit_mapper.is_empty(), NON_EXISTENT_KEY_ERR_MSG);

        let caller_address = self.blockchain().get_caller();
        self.require_signature(&address, &caller_address, signature);

        let deposit = deposit_mapper.take();
        let num_tokens_transferred = deposit.funds.len();
        let mut remaining_fee_payment = deposit.fees.value;

        let fee_token = remaining_fee_payment.token_identifier.clone();
        let fee = self.fee(&fee_token).get();
        require!(
            deposit.expiration >= self.blockchain().get_block_timestamp_millis(),
            "deposit expired"
        );

        let fee_cost = fee * num_tokens_transferred as u64;
        remaining_fee_payment.amount -= &fee_cost;

        self.deduct_and_collect_fees(&fee_token, fee_cost);

        if !deposit.funds.is_empty() {
            self.tx()
                .to(&caller_address)
                .payment(&deposit.funds)
                .transfer();
        }
        if remaining_fee_payment.amount > 0 {
            self.tx()
                .to(&deposit.depositor_address)
                .payment(&remaining_fee_payment)
                .transfer();
        }
    }

    #[endpoint]
    #[payable]
    fn forward(
        &self,
        address: ManagedAddress,
        forward_address: ManagedAddress,
        signature: ManagedByteArray<Self::Api, ED25519_SIGNATURE_BYTE_LEN>,
    ) {
        let additional_fee_payment = self.call_value().single_optional();
        let caller_address = self.blockchain().get_caller();
        self.require_signature(&address, &caller_address, signature);
        let new_deposit = self.deposit(&forward_address);

        let mut current_deposit = self.deposit(&address).take();
        let num_tokens = current_deposit.funds.len();

        let mut fee_token: Option<TokenId<Self::Api>> = None;
        let mut fee = BigUint::zero();

        require!(
            !new_deposit.is_empty(),
            "cannot deposit funds without covering the fee cost first"
        );
        new_deposit.update(|target_deposit| {
            fee_token = Some(if let Some(additional_fee_token) = additional_fee_payment {
                self.update_fees(
                    caller_address,
                    &forward_address,
                    additional_fee_token.clone(),
                );
                additional_fee_token.token_identifier.clone()
            } else {
                target_deposit.fees.value.token_identifier.clone()
            });

            fee = self.fee(fee_token.as_ref().unwrap()).get();

            require!(target_deposit.funds.is_empty(), "key already used");
            require!(
                &fee * num_tokens as u64 <= *target_deposit.fees.value.amount.as_big_uint(),
                "cannot deposit funds without covering the fee cost first"
            );

            target_deposit.fees.num_token_to_transfer += num_tokens;
            target_deposit.expiration = current_deposit.expiration;
            target_deposit.funds = current_deposit.funds;
        });

        let forward_fee = &fee * num_tokens as u64;
        current_deposit.fees.value.amount -= &forward_fee;

        self.deduct_and_collect_fees(&fee_token.unwrap(), forward_fee);

        if current_deposit.fees.value.amount > 0 {
            self.send_fee_to_address(
                &current_deposit.fees.value,
                &current_deposit.depositor_address,
            );
        }
    }

    fn require_signature(
        &self,
        address: &ManagedAddress,
        caller_address: &ManagedAddress,
        signature: ManagedByteArray<Self::Api, ED25519_SIGNATURE_BYTE_LEN>,
    ) {
        let address_buffer = address.as_managed_buffer();
        let caller_buffer = caller_address.as_managed_buffer();
        self.crypto()
            .verify_ed25519(address_buffer, caller_buffer, signature.as_managed_buffer());
    }
}
