#![no_std]

elrond_wasm::imports!();

#[elrond_wasm::contract]
pub trait NftStoragePrepay {
    #[init]
    fn init(&self, cost_per_byte: BigUint) {
        self.cost_per_byte().set(&cost_per_byte);
    }

    // endpoints - owner-only

    #[endpoint(setCostPerByte)]
    fn set_cost_per_byte(&self, cost_per_byte: BigUint) -> SCResult<()> {
        only_owner!(self, "Only owner may call this function");

        self.cost_per_byte().set(&cost_per_byte);

        Ok(())
    }

    #[endpoint(reserveFunds)]
    fn reserve_funds(&self, address: ManagedAddress, file_size: BigUint) -> SCResult<()> {
        only_owner!(self, "Only owner may call this function");

        let storage_cost = self.get_cost_for_size(file_size);
        let mut user_deposit = self.deposit(&address).get();
        require!(
            user_deposit >= storage_cost,
            "User does not have enough deposit"
        );

        user_deposit -= &storage_cost;
        self.deposit(&address).set(&user_deposit);
        self.total_reserved()
            .update(|reserved| *reserved += storage_cost);

        Ok(())
    }

    #[endpoint]
    fn claim(&self) -> SCResult<()> {
        only_owner!(self, "Only owner may call this function");

        let total_reserved = self.total_reserved().get();
        require!(total_reserved > 0, "Nothing to claim");

        self.total_reserved().clear();

        let owner = self.blockchain().get_caller();
        self.send().direct_egld(&owner, &total_reserved, &[]);

        Ok(())
    }

    // endpoints

    #[payable("EGLD")]
    #[endpoint(depositPaymentForStorage)]
    fn deposit_payment_for_storage(&self, #[payment] payment: BigUint) {
        let caller = self.blockchain().get_caller();
        self.deposit(&caller).update(|deposit| *deposit += payment);
    }

    /// defaults to max amount
    #[endpoint(withdraw)]
    fn withdraw(&self, #[var_args] opt_amount: OptionalArg<BigUint>) -> SCResult<()> {
        let caller = self.blockchain().get_caller();
        let mut user_deposit = self.deposit(&caller).get();
        let amount = match opt_amount {
            OptionalArg::Some(amt) => amt,
            OptionalArg::None => user_deposit.clone(),
        };

        require!(user_deposit >= amount, "Can't withdraw more than deposit");

        user_deposit -= &amount;
        self.deposit(&caller).set(&user_deposit);

        self.send().direct_egld(&caller, &amount, &[]);

        Ok(())
    }

    // views

    #[view(getCostForSize)]
    fn get_cost_for_size(&self, file_size: BigUint) -> BigUint {
        let cost_per_byte = self.cost_per_byte().get();

        file_size * cost_per_byte
    }

    #[view(getDepositAmount)]
    fn get_deposit_amount(&self) -> BigUint {
        let caller = self.blockchain().get_caller();

        self.deposit(&caller).get()
    }

    // storage

    #[view(getCostPerByte)]
    #[storage_mapper("costPerByte")]
    fn cost_per_byte(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("deposit")]
    fn deposit(&self, address: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[storage_mapper("totalReserved")]
    fn total_reserved(&self) -> SingleValueMapper<BigUint>;
}
