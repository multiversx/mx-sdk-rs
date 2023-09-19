multiversx_sc::imports!();

use crate::{
    constants::*,
    deposit_info::{DepositInfo, Fee},
    storage,
};
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

    fn check_token_is_accepted_as_fee(&self, token: &EgldOrEsdtTokenIdentifier) {
        let accepted_fee_token = self.fee_token().get();
        require!(token == &accepted_fee_token, "Invalid fee token provided");
    }

    fn make_fund(
        &self,
        egld_payment: BigUint,
        esdt_payment: ManagedVec<EsdtTokenPayment>,
        address: ManagedAddress,
        valability: u64,
    ) {
        let deposit_mapper = self.deposit(&address);
        let num_tokens = self.get_num_token_transfers(&egld_payment, &esdt_payment);
        require!(num_tokens > 0, "amount must be greater than 0");

        let fee = self.fee().get();
        deposit_mapper.update(|deposit| {
            require!(
                deposit.egld_funds == 0 && deposit.esdt_funds.is_empty(),
                "key already used"
            );
            require!(
                fee * num_tokens as u64 <= deposit.fees.value,
                CANNOT_DEPOSIT_FUNDS_ERR_MSG
            );

            deposit.fees.num_token_to_transfer += num_tokens;
            deposit.valability = valability;
            deposit.expiration_round = self.get_expiration_round(valability);
            deposit.esdt_funds = esdt_payment;
            deposit.egld_funds = egld_payment;
        });
    }

    fn update_fees(
        &self,
        caller_address: ManagedAddress,
        address: &ManagedAddress,
        payment: EgldOrEsdtTokenPayment,
    ) {
        self.check_token_is_accepted_as_fee(&payment.token_identifier);
        let deposit_mapper = self.deposit(address);
        if !deposit_mapper.is_empty() {
            deposit_mapper.update(|deposit| {
                require!(
                    deposit.depositor_address == caller_address,
                    "invalid depositor address"
                );
                deposit.fees.value += payment.amount;
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
                value: payment.amount,
            },
        };
        deposit_mapper.set(new_deposit);
    }
}
