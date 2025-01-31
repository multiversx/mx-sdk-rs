use multiversx_sc::imports::*;

use crate::{constants::*, helpers, storage};

pub use multiversx_sc::api::ED25519_SIGNATURE_BYTE_LEN;

#[multiversx_sc::module]
pub trait SignatureOperationsModule: storage::StorageModule + helpers::HelpersModule {
    #[endpoint]
    fn withdraw(&self, address: ManagedAddress) {
        let deposit_mapper = self.deposit(&address);
        require!(!deposit_mapper.is_empty(), NON_EXISTENT_KEY_ERR_MSG);

        let deposit = deposit_mapper.take();
        let paid_fee_token = deposit.fees.value;

        let block_round = self.blockchain().get_block_round();
        require!(
            deposit.expiration_round < block_round,
            "withdrawal has not been available yet"
        );

        let mut funds = deposit.funds;

        let fee =
            EgldOrEsdtTokenPayment::new(paid_fee_token.token_identifier, 0, paid_fee_token.amount);
        funds.push(fee);

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

        let block_round = self.blockchain().get_block_round();
        let deposit = deposit_mapper.take();
        let num_tokens_transfered = deposit.funds.len();
        let mut deposited_fee = deposit.fees.value;

        let fee_token = deposited_fee.token_identifier.clone();
        let fee = self.fee(&fee_token).get();
        require!(deposit.expiration_round >= block_round, "deposit expired");

        let fee_cost = fee * num_tokens_transfered as u64;
        deposited_fee.amount -= &fee_cost;

        self.collected_fees(&fee_token)
            .update(|collected_fees| *collected_fees += fee_cost);

        if !deposit.funds.is_empty() {
            self.tx()
                .to(&caller_address)
                .payment(&deposit.funds)
                .transfer();
        }
        if deposited_fee.amount > 0 {
            self.send_fee_to_address(&deposited_fee, &deposit.depositor_address);
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
        let paid_fee = self.call_value().egld_or_single_esdt();
        let caller_address = self.blockchain().get_caller();
        let fee_token = paid_fee.token_identifier.clone();
        self.require_signature(&address, &caller_address, signature);
        self.update_fees(caller_address, &forward_address, paid_fee);

        let new_deposit = self.deposit(&forward_address);
        let fee = self.fee(&fee_token).get();

        let mut current_deposit = self.deposit(&address).take();
        let num_tokens = current_deposit.funds.len();
        new_deposit.update(|fwd_deposit| {
            require!(fwd_deposit.funds.is_empty(), "key already used");
            require!(
                &fee * num_tokens as u64 <= fwd_deposit.fees.value.amount,
                "cannot deposit funds without covering the fee cost first"
            );

            fwd_deposit.fees.num_token_to_transfer += num_tokens;
            fwd_deposit.valability = current_deposit.valability;
            fwd_deposit.expiration_round = self.get_expiration_round(current_deposit.valability);
            fwd_deposit.funds = current_deposit.funds;
        });

        let forward_fee = &fee * num_tokens as u64;
        current_deposit.fees.value.amount -= &forward_fee;

        self.collected_fees(&fee_token)
            .update(|collected_fees| *collected_fees += forward_fee);

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
        let addr = address.as_managed_buffer();
        let message = caller_address.as_managed_buffer();
        self.crypto()
            .verify_ed25519(addr, message, signature.as_managed_buffer());
    }
}
