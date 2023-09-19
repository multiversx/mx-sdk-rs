multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::{
    constants::*,
    deposit_info::{DepositInfo, Fee},
    helpers, storage,
};

#[multiversx_sc::module]
pub trait PayFeeAndFund: storage::StorageModule + helpers::HelpersModule {
    #[endpoint]
    #[payable("*")]
    fn fund(&self, address: ManagedAddress, valability: u64) {
        let deposit_mapper = self.deposit(&address);
        require!(!deposit_mapper.is_empty(), FEES_NOT_COVERED_ERR_MSG);
        let depositor = deposit_mapper.get().depositor_address;
        require!(
            self.blockchain().get_caller() == depositor,
            "invalid depositor"
        );

        let egld_payment = self.call_value().egld_value().clone_value();
        let esdt_payment = self.call_value().all_esdt_transfers().clone_value();
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

    #[endpoint(depositFees)]
    #[payable("EGLD")]
    fn deposit_fees(&self, address: ManagedAddress) {
        let payment = self.call_value().egld_or_single_esdt();
        let accepted_fee_token = self.fee_token().get();
        require!(
            payment.token_identifier == accepted_fee_token,
            "Invalid fee token provided"
        );
        let caller_address = self.blockchain().get_caller();
        let deposit_mapper = self.deposit(&address);
        if !deposit_mapper.is_empty() {
            deposit_mapper.update(|deposit| deposit.fees.value += payment.amount);

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
