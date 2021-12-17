#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, TypeAbi, PartialEq, Clone, Copy, Debug)]
pub enum Status {
    FundingPeriod,
    Successful,
    Failed,
}

#[elrond_wasm::contract]
pub trait Crowdfunding {
    #[init]
    fn init(
        &self,
        target: BigUint,
        deadline: u64,
        token_identifier: TokenIdentifier,
    ) -> SCResult<()> {
        require!(target > 0, "Target must be more than 0");
        self.target().set(&target);

        require!(
            deadline > self.get_current_time(),
            "Deadline can't be in the past"
        );
        self.deadline().set(&deadline);

        require!(
            token_identifier.is_egld() || token_identifier.is_valid_esdt_identifier(),
            "Invalid token provided"
        );
        self.cf_token_identifier().set(&token_identifier);

        Ok(())
    }

    #[endpoint]
    #[payable("*")]
    fn fund(
        &self,
        #[payment_token] token: TokenIdentifier,
        #[payment] payment: BigUint,
    ) -> SCResult<()> {
        require!(
            self.status() == Status::FundingPeriod,
            "cannot fund after deadline"
        );
        require!(token == self.cf_token_identifier().get(), "wrong token");

        let caller = self.blockchain().get_caller();
        self.deposit(&caller).update(|deposit| *deposit += payment);

        Ok(())
    }

    #[view]
    fn status(&self) -> Status {
        if self.get_current_time() < self.deadline().get() {
            Status::FundingPeriod
        } else if self.get_current_funds() >= self.target().get() {
            Status::Successful
        } else {
            Status::Failed
        }
    }

    #[view(getCurrentFunds)]
    fn get_current_funds(&self) -> BigUint {
        let token = self.cf_token_identifier().get();

        self.blockchain().get_sc_balance(&token, 0)
    }

    #[endpoint]
    fn claim(&self) -> SCResult<()> {
        match self.status() {
            Status::FundingPeriod => sc_error!("cannot claim before deadline"),
            Status::Successful => {
                let caller = self.blockchain().get_caller();
                require!(
                    caller == self.blockchain().get_owner_address(),
                    "only owner can claim successful funding"
                );

                let token_identifier = self.cf_token_identifier().get();
                let sc_balance = self.get_current_funds();

                self.send()
                    .direct(&caller, &token_identifier, 0, &sc_balance, &[]);

                Ok(())
            },
            Status::Failed => {
                let caller = self.blockchain().get_caller();
                let deposit = self.deposit(&caller).get();

                if deposit > 0 {
                    let token_identifier = self.cf_token_identifier().get();

                    self.deposit(&caller).clear();
                    self.send()
                        .direct(&caller, &token_identifier, 0, &deposit, &[]);
                }

                Ok(())
            },
        }
    }

    // private

    fn get_current_time(&self) -> u64 {
        self.blockchain().get_block_timestamp()
    }

    // storage

    #[view(getTarget)]
    #[storage_mapper("target")]
    fn target(&self) -> SingleValueMapper<BigUint>;

    #[view(getDeadline)]
    #[storage_mapper("deadline")]
    fn deadline(&self) -> SingleValueMapper<u64>;

    #[view(getDeposit)]
    #[storage_mapper("deposit")]
    fn deposit(&self, donor: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[view(getCrowdfundingTokenIdentifier)]
    #[storage_mapper("tokenIdentifier")]
    fn cf_token_identifier(&self) -> SingleValueMapper<TokenIdentifier>;
}
