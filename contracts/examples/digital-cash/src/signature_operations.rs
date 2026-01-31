use multiversx_sc::imports::*;

use crate::{
    digital_cash_deposit,
    storage::{self, DepositInfo, DepositKey},
};

pub use multiversx_sc::api::ED25519_SIGNATURE_BYTE_LEN;

#[multiversx_sc::module]
pub trait SignatureOperationsModule:
    storage::StorageModule + digital_cash_deposit::DepositModule
{
    /// Withdraws an expired deposit back to the depositor.
    ///
    /// # Preconditions
    /// - Deposit must exist for the given deposit_key
    /// - Current block timestamp must be greater than the deposit's expiration timestamp
    ///
    /// # Requirements
    /// - Can be called by anyone (not restricted to depositor)
    /// - No signature required
    /// - No payment required
    ///
    /// # Outcomes
    /// - Deposit is removed from storage
    /// - All fees (if any) are transferred to the original depositor
    /// - All funds are transferred to the original depositor
    ///
    /// # Panics
    /// - "non-existent key" if deposit doesn't exist
    /// - "cannot withdraw, deposit not expired yet" if expiration timestamp hasn't passed
    #[endpoint(withdrawExpired)]
    fn withdraw_expired(&self, deposit_key: DepositKey<Self::Api>) {
        let deposit_mapper = self.deposit(&deposit_key);
        self.require_deposit_exists(&deposit_mapper);

        let deposit = deposit_mapper.take();

        require!(
            deposit.expiration < self.blockchain().get_block_timestamp_millis(),
            "cannot withdraw, deposit not expired yet"
        );

        if let Some(fees) = deposit.fees {
            self.tx()
                .to(&deposit.depositor_address)
                .payment(fees.into_payment())
                .transfer();
        }
        if !deposit.funds.is_empty() {
            self.tx()
                .to(&deposit.depositor_address)
                .payment(deposit.funds)
                .transfer();
        }
    }

    /// Claims a deposit by providing a valid ED25519 signature.
    ///
    /// # Preconditions
    /// - Deposit must exist for the given deposit_key
    /// - Current block timestamp must be less than or equal to the deposit's expiration
    /// - Signature must be valid: signed with the private key of deposit_key, over caller's address
    /// - If fees are enabled, deposit must have sufficient fees (base_fee × number_of_funds)
    ///
    /// # Requirements
    /// - Valid ED25519 signature proving ownership of deposit_key's private key
    /// - No payment required
    ///
    /// # Outcomes
    /// - Deposit is removed from storage
    /// - All funds are transferred to the caller
    /// - Required fees are collected by the contract
    /// - Excess fees (if any) are returned to the original depositor
    ///
    /// # Panics
    /// - "non-existent key" if deposit doesn't exist
    /// - ED25519 signature verification fails if signature is invalid
    /// - "deposit expired" if current timestamp is greater than expiration
    /// - "insufficient fees provided" if fees don't cover all funds
    #[endpoint]
    fn claim(
        &self,
        deposit_key: DepositKey<Self::Api>,
        signature: ManagedByteArray<Self::Api, ED25519_SIGNATURE_BYTE_LEN>,
    ) {
        let caller_address = self.blockchain().get_caller();
        let deposit = self.process_claim(&deposit_key, &signature, &caller_address);

        // the funds are transferred to the caller
        if !deposit.funds.is_empty() {
            self.tx()
                .to(&caller_address)
                .payment(deposit.funds)
                .transfer();
        }
    }

    /// Processes a claim by validating the signature and expiration, then collecting fees.
    ///
    /// This helper function is used by both `claim` and `forward` endpoints.
    /// It verifies the ED25519 signature, checks expiration, collects required fees,
    /// and returns any leftover fees to the depositor.
    ///
    /// Returns the deposit with funds still intact (not transferred).
    fn process_claim(
        &self,
        deposit_key: &DepositKey<Self::Api>,
        signature: &ManagedByteArray<Self::Api, ED25519_SIGNATURE_BYTE_LEN>,
        caller_address: &ManagedAddress,
    ) -> DepositInfo<Self::Api> {
        let deposit_mapper = self.deposit(deposit_key);
        self.require_deposit_exists(&deposit_mapper);

        self.check_signature(deposit_key, caller_address, signature.clone());

        let mut deposit = deposit_mapper.take();
        require!(
            deposit.expiration >= self.blockchain().get_block_timestamp_millis(),
            "deposit expired"
        );

        let (opt_fees_to_collect, opt_leftover) =
            self.take_fees_to_collect_and_leftover(&mut deposit);

        if let Some(fees_to_collect) = opt_fees_to_collect {
            self.add_collected_fee(&fees_to_collect);
        }

        // any leftover fees are returned to the depositor
        if let Some(leftover) = opt_leftover {
            self.tx()
                .to(&deposit.depositor_address)
                .payment(leftover.into_payment())
                .transfer();
        }

        deposit
    }

    /// Forwards funds from one deposit to another existing deposit.
    ///
    /// # Preconditions
    /// - Source deposit must exist for deposit_key
    /// - Destination deposit must already exist for forward_deposit_key with fees paid
    /// - Current block timestamp must be less than or equal to source deposit's expiration
    /// - Signature must be valid: signed with private key of deposit_key, over caller's address
    /// - If fees enabled, source deposit must have sufficient fees for its funds
    /// - Destination deposit must have sufficient fees for combined funds (existing + forwarded)
    /// - Caller must be the depositor of the destination deposit
    ///
    /// # Requirements
    /// - Valid ED25519 signature for source deposit
    /// - You may send an additional single fungible payment as fee, which will be added to the destination deposit's fees
    ///
    /// # Outcomes
    /// - Source deposit is removed from storage
    /// - Source deposit's funds are appended to destination deposit
    /// - Destination deposit's expiration is updated to source deposit's expiration
    /// - Required fees from source are collected by the contract
    /// - Excess fees from source (if any) are returned to source's original depositor
    /// - Any additional fee sent with the forward call is added to the destination deposit's fees
    ///
    /// # Panics
    /// - "non-existent key" if source deposit doesn't exist
    /// - "forward deposit needs to exist in advance, with fees paid" if destination doesn't exist
    /// - ED25519 signature verification fails if signature is invalid
    /// - "deposit expired" if source deposit has expired
    /// - "invalid depositor" if caller is not the depositor of destination
    /// - "insufficient fees provided" if combined fees don't cover all funds
    #[endpoint]
    #[payable]
    fn forward(
        &self,
        deposit_key: DepositKey<Self::Api>,
        forward_deposit_key: DepositKey<Self::Api>,
        signature: ManagedByteArray<Self::Api, ED25519_SIGNATURE_BYTE_LEN>,
    ) {
        let opt_additional_fee = self.call_value().single_optional();
        let caller_address = self.blockchain().get_caller();
        let old_deposit = self.process_claim(&deposit_key, &signature, &caller_address);
        require!(
            old_deposit.fees.is_none(),
            "fees should be empty at this point 1"
        );

        let forward_deposit_mapper = self.deposit(&forward_deposit_key);
        require!(
            !forward_deposit_mapper.is_empty(),
            "forward deposit needs to exist in advance, with fees paid"
        );

        forward_deposit_mapper.update(|deposit| {
            self.perform_append_funds(
                deposit,
                &caller_address,
                old_deposit.expiration,
                old_deposit.funds,
            );
            if let Some(additional_fee) = opt_additional_fee {
                self.add_deposit_fee(deposit, additional_fee.clone().fungible_or_panic());
            }
        });
    }

    /// Verifies an ED25519 signature over the deposit key.
    ///
    /// The signature must be generated using the private key corresponding to the deposit key,
    /// signing the caller's address. This proves the caller has authorization to claim the deposit.
    fn check_signature(
        &self,
        deposit_key: &DepositKey<Self::Api>,
        caller_address: &ManagedAddress,
        signature: ManagedByteArray<Self::Api, ED25519_SIGNATURE_BYTE_LEN>,
    ) {
        let caller_buffer = caller_address.as_managed_buffer();
        self.crypto().verify_ed25519(
            deposit_key.as_managed_buffer(),
            caller_buffer,
            signature.as_managed_buffer(),
        );
    }
}
