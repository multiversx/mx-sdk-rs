multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::{constants::*, helpers, storage};

pub use multiversx_sc::api::{ED25519_KEY_BYTE_LEN, ED25519_SIGNATURE_BYTE_LEN};

#[multiversx_sc::module]
pub trait SignatureOperationsModule: storage::StorageModule + helpers::HelpersModule {
    #[endpoint]
    fn withdraw(&self, address: ManagedAddress) {
        let deposit_mapper = self.deposit(&address);
        let accepted_fee_token = self.fee_token().get();
        require!(!deposit_mapper.is_empty(), NON_EXISTENT_KEY_ERR_MSG);

        let block_round = self.blockchain().get_block_round();
        let deposit = deposit_mapper.take();
        require!(
            deposit.expiration_round < block_round,
            "withdrawal has not been available yet"
        );

        let mut egld_funds = deposit.egld_funds;
        let mut esdt_funds = deposit.esdt_funds;

        if accepted_fee_token == EgldOrEsdtTokenIdentifier::egld() {
            egld_funds += deposit.fees.value;
        } else {
            let esdt_fee_token = accepted_fee_token.unwrap_esdt();
            let esdt_fee = EsdtTokenPayment::new(esdt_fee_token, 0, deposit.fees.value);
            esdt_funds.push(esdt_fee);
        }

        if egld_funds > 0 {
            self.send()
                .direct_egld(&deposit.depositor_address, &egld_funds);
        }

        if !esdt_funds.is_empty() {
            self.send()
                .direct_multi(&deposit.depositor_address, &esdt_funds);
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
        let fee = self.fee().get();
        let mut deposit = deposit_mapper.take();
        require!(deposit.expiration_round >= block_round, "deposit expired");

        let num_tokens_transfered = deposit.get_num_tokens();
        let fee_cost = fee * num_tokens_transfered as u64;
        deposit.fees.value -= &fee_cost;

        self.collected_fees()
            .update(|collected_fees| *collected_fees += fee_cost);

        if deposit.egld_funds > 0 {
            self.send()
                .direct_egld(&caller_address, &deposit.egld_funds);
        }
        if !deposit.esdt_funds.is_empty() {
            self.send()
                .direct_multi(&caller_address, &deposit.esdt_funds);
        }
        if deposit.fees.value > 0 {
            self.send_fee_to_address(&deposit.fees.value, &deposit.depositor_address);
        }
    }

    #[endpoint]
    #[payable("*")]
    fn forward(
        &self,
        address: ManagedAddress,
        forward_address: ManagedAddress,
        signature: ManagedByteArray<Self::Api, ED25519_SIGNATURE_BYTE_LEN>,
    ) {
        let fee = self.call_value().egld_or_single_esdt();
        let caller_address = self.blockchain().get_caller();
        self.require_signature(&address, &caller_address, signature);
        self.update_fees(caller_address, &forward_address, fee);

        let new_deposit = self.deposit(&forward_address);
        let fee = self.fee().get();

        let mut current_deposit = self.deposit(&address).take();
        let num_tokens = current_deposit.get_num_tokens();
        new_deposit.update(|fwd_deposit| {
            require!(
                fwd_deposit.egld_funds == BigUint::zero() && fwd_deposit.esdt_funds.is_empty(),
                "key already used"
            );
            require!(
                &fee * num_tokens as u64 <= fwd_deposit.fees.value,
                "cannot deposit funds without covering the fee cost first"
            );

            fwd_deposit.fees.num_token_to_transfer += num_tokens;
            fwd_deposit.valability = current_deposit.valability;
            fwd_deposit.expiration_round = self.get_expiration_round(current_deposit.valability);
            fwd_deposit.esdt_funds = current_deposit.esdt_funds;
            fwd_deposit.egld_funds = current_deposit.egld_funds;
        });

        let forward_fee = &fee * num_tokens as u64;
        current_deposit.fees.value -= &forward_fee;

        self.collected_fees()
            .update(|collected_fees| *collected_fees += forward_fee);

        if current_deposit.fees.value > 0 {
            self.send_fee_to_address(
                &current_deposit.fees.value,
                &current_deposit.depositor_address,
            );
        }
    }

    fn make_forward(&self) {}

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
