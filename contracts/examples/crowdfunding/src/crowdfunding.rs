#![no_std]

use multiversx_sc::{chain_core::types::TimestampMillis, derive_imports::*, imports::*};
pub mod crowdfunding_proxy;

#[type_abi]
#[derive(TopEncode, TopDecode, PartialEq, Eq, Clone, Copy, Debug)]
pub enum Status {
    FundingPeriod,
    Successful,
    Failed,
}

#[multiversx_sc::contract]
pub trait Crowdfunding {
    #[init]
    fn init(&self, token_identifier: TokenId, target: BigUint, deadline: TimestampMillis) {
        require!(token_identifier.is_valid(), "Invalid token provided");
        self.cf_token_identifier().set(token_identifier);

        require!(target > 0, "Target must be more than 0");
        self.target().set(target);

        require!(
            deadline > self.get_current_time_ms(),
            "Deadline can't be in the past"
        );
        self.deadline().set(deadline);
    }

    #[endpoint]
    #[payable]
    fn fund(&self) {
        let payment = self.call_value().single();

        require!(
            payment.token_identifier == self.cf_token_identifier().get(),
            "wrong token"
        );
        require!(payment.is_fungible(), "only fungible tokens accepted");
        require!(
            self.status() == Status::FundingPeriod,
            "cannot fund after deadline"
        );

        let caller = self.blockchain().get_caller();
        self.deposit(&caller)
            .update(|deposit| *deposit += payment.amount.as_big_uint());
    }

    #[view]
    fn status(&self) -> Status {
        if self.get_current_time_ms() < self.deadline().get() {
            Status::FundingPeriod
        } else if self.get_current_funds() >= self.target().get() {
            Status::Successful
        } else {
            Status::Failed
        }
    }

    #[view(getCurrentFunds)]
    #[title("currentFunds")]
    fn get_current_funds(&self) -> BigUint {
        let token = self.cf_token_identifier().get();

        self.blockchain().get_sc_balance(&token, 0)
    }

    #[endpoint]
    fn claim(&self) {
        match self.status() {
            Status::FundingPeriod => sc_panic!("cannot claim before deadline"),
            Status::Successful => {
                let caller = self.blockchain().get_caller();
                require!(
                    caller == self.blockchain().get_owner_address(),
                    "only owner can claim successful funding"
                );

                let token_identifier = self.cf_token_identifier().get();
                let sc_balance = self.get_current_funds();

                if let Some(sc_balance_non_zero) = sc_balance.into_non_zero() {
                    self.tx()
                        .to(&caller)
                        .payment(Payment::new(token_identifier, 0, sc_balance_non_zero))
                        .transfer();
                }
            }
            Status::Failed => {
                let caller = self.blockchain().get_caller();
                let deposit = self.deposit(&caller).get();

                if deposit > 0u32 {
                    let token_identifier = self.cf_token_identifier().get();

                    self.deposit(&caller).clear();

                    if let Some(deposit_non_zero) = deposit.into_non_zero() {
                        self.tx()
                            .to(&caller)
                            .payment(Payment::new(token_identifier, 0, deposit_non_zero))
                            .transfer();
                    }
                }
            }
        }
    }

    // private

    fn get_current_time_ms(&self) -> TimestampMillis {
        self.blockchain().get_block_timestamp_millis()
    }

    // storage

    #[view(getTarget)]
    #[title("target")]
    #[storage_mapper("target")]
    fn target(&self) -> SingleValueMapper<BigUint>;

    #[view(getDeadline)]
    #[title("deadline")]
    #[storage_mapper("deadline")]
    fn deadline(&self) -> SingleValueMapper<TimestampMillis>;

    #[view(getDeposit)]
    #[title("deposit")]
    #[storage_mapper("deposit")]
    fn deposit(&self, donor: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[view(getCrowdfundingTokenIdentifier)]
    #[title("tokenIdentifier")]
    #[storage_mapper("tokenIdentifier")]
    fn cf_token_identifier(&self) -> SingleValueMapper<TokenId>;
}
