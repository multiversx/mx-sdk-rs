multiversx_sc::imports!();

use crate::{constants::*, storage};
#[multiversx_sc::module]
pub trait HelpersModule: storage::StorageModule {
    fn send_fee_to_address(&self, fee_amount: &BigUint, address: &ManagedAddress) {
        let accepted_fee_token = self.fee_token().get();
        if accepted_fee_token == EgldOrEsdtTokenIdentifier::egld() {
            self.send().direct_egld(address, fee_amount);
        } else {
            let esdt_fee_token = accepted_fee_token.unwrap_esdt();
            self.send()
                .direct_esdt(address, &esdt_fee_token, 0, fee_amount);
        }
    }

    fn get_num_token_transfers(
        &self,
        egld_value: &BigUint,
        esdt_transfers: &ManagedVec<EsdtTokenPayment>,
    ) -> usize {
        let mut amount = esdt_transfers.len();
        if egld_value > &0 {
            amount += 1;
        }

        amount
    }

    fn get_expiration_round(&self, valability: u64) -> u64 {
        let valability_rounds = valability / SECONDS_PER_ROUND;
        self.blockchain().get_block_round() + valability_rounds
    }
}
