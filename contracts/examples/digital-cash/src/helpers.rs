use multiversx_sc::imports::*;

use crate::{
    constants::*,
    deposit_info::{DepositInfo, Fee},
    storage,
};
#[multiversx_sc::module]
pub trait HelpersModule: storage::StorageModule {
    fn send_fee_to_address(&self, fee: &EgldOrEsdtTokenPayment, address: &ManagedAddress) {
        if fee.token_identifier == EgldOrEsdtTokenIdentifier::egld() {
            self.tx().to(address).egld(&fee.amount).transfer();
        } else {
            let esdt_fee = fee.clone().unwrap_esdt();
            self.tx()
                .to(address)
                .single_esdt(&esdt_fee.token_identifier, 0, &esdt_fee.amount)
                .transfer();
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

    fn get_fee_for_token(&self, token: &EgldOrEsdtTokenIdentifier) -> BigUint {
        require!(
            self.whitelisted_fee_tokens().contains(token),
            "invalid fee toke provided"
        );
        let fee_token = self.fee(token);
        fee_token.get()
    }

    fn make_fund(
        &self,
        egld_payment: BigUint,
        esdt_payment: ManagedVec<EsdtTokenPayment>,
        address: ManagedAddress,
        valability: u64,
    ) {
        let deposit_mapper = self.deposit(&address);

        deposit_mapper.update(|deposit| {
            require!(
                deposit.egld_funds == 0 && deposit.esdt_funds.is_empty(),
                "key already used"
            );
            let num_tokens = self.get_num_token_transfers(&egld_payment, &esdt_payment);
            deposit.fees.num_token_to_transfer += num_tokens;
            deposit.valability = valability;
            deposit.expiration_round = self.get_expiration_round(valability);
            deposit.esdt_funds = esdt_payment;
            deposit.egld_funds = egld_payment;
        });
    }

    fn check_fees_cover_number_of_tokens(
        &self,
        num_tokens: usize,
        fee: BigUint,
        paid_fee: BigUint,
    ) {
        require!(num_tokens > 0, "amount must be greater than 0");
        require!(
            fee * num_tokens as u64 <= paid_fee,
            CANNOT_DEPOSIT_FUNDS_ERR_MSG
        );
    }

    fn update_fees(
        &self,
        caller_address: ManagedAddress,
        address: &ManagedAddress,
        payment: EgldOrEsdtTokenPayment,
    ) {
        self.get_fee_for_token(&payment.token_identifier);
        let deposit_mapper = self.deposit(address);
        if !deposit_mapper.is_empty() {
            deposit_mapper.update(|deposit| {
                require!(
                    deposit.depositor_address == caller_address,
                    "invalid depositor address"
                );
                require!(
                    deposit.fees.value.token_identifier == payment.token_identifier,
                    "can only have 1 type of token as fee"
                );
                deposit.fees.value.amount += payment.amount;
            });
            return;
        }

        let new_deposit = DepositInfo {
            depositor_address: caller_address,
            esdt_funds: ManagedVec::new(),
            egld_funds: BigUint::zero(),
            valability: 0,
            expiration_round: 0,
            fees: Fee {
                num_token_to_transfer: 0,
                value: payment,
            },
        };
        deposit_mapper.set(new_deposit);
    }
}
