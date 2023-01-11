#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait SimpleErc20Token {
    // STORAGE

    /// Total number of tokens in existence.
    #[view(totalSupply)]
    #[storage_mapper("totalSupply")]
    fn total_supply(&self) -> SingleValueMapper<BigUint>;

    /// Gets the balance of the specified address.
    ///
    /// Arguments:
    ///
    /// * `address` The address to query the the balance of
    ///
    #[view(balanceOf)]
    #[storage_mapper("balance")]
    fn token_balance(&self, address: &ManagedAddress) -> SingleValueMapper<BigUint>;

    /// The amount of tokens that an owner allowed to a spender.
    ///
    /// Arguments:
    ///
    /// * `owner` The address that owns the funds.
    /// * `spender` The address that will spend the funds.
    ///
    #[view(allowance)]
    #[storage_mapper("allowance")]
    fn allowance(
        &self,
        owner: &ManagedAddress,
        spender: &ManagedAddress,
    ) -> SingleValueMapper<BigUint>;

    // FUNCTIONALITY

    /// Constructor, is called immediately after the contract is created
    /// Will set the fixed global token supply and give all the supply to the creator.
    #[init]
    fn init(&self, total_supply: &BigUint) {
        let creator = self.blockchain().get_caller();

        // save total supply
        self.total_supply().set(total_supply);

        // deployer initially receives the total supply
        self.token_balance(&creator)
            .update(|balance| *balance += total_supply);
    }

    /// This method is private, deduplicates logic from transfer and transferFrom.
    fn perform_transfer(&self, sender: ManagedAddress, recipient: ManagedAddress, amount: BigUint) {
        // check if enough funds & decrease sender balance
        self.token_balance(&sender).update(|balance| {
            require!(amount <= *balance, "insufficient funds");

            *balance -= &amount;
        });

        // increase recipient balance
        self.token_balance(&recipient)
            .update(|balance| *balance += &amount);

        // log operation
        self.transfer_event(&sender, &recipient, &amount);
    }

    /// Transfer token to a specified address from sender.
    ///
    /// Arguments:
    ///
    /// * `to` The address to transfer to.
    ///
    #[endpoint]
    fn transfer(&self, to: ManagedAddress, amount: BigUint) {
        // the sender is the caller
        let sender = self.blockchain().get_caller();
        self.perform_transfer(sender, to, amount)
    }

    /// Use allowance to transfer funds between two accounts.
    ///
    /// Arguments:
    ///
    /// * `sender` The address to transfer from.
    /// * `recipient` The address to transfer to.
    /// * `amount` the amount of tokens to be transferred.
    ///
    #[endpoint(transferFrom)]
    fn transfer_from(&self, sender: ManagedAddress, recipient: ManagedAddress, amount: BigUint) {
        // get caller
        let caller = self.blockchain().get_caller();

        self.allowance(&sender, &caller).update(|allowance| {
            require!(amount <= *allowance, "allowance exceeded");

            *allowance -= &amount;
        });

        // transfer
        self.perform_transfer(sender, recipient, amount)
    }

    /// Approve the given address to spend the specified amount of tokens on behalf of the sender.
    /// It overwrites any previously existing allowance from sender to beneficiary.
    ///
    /// Arguments:
    ///
    /// * `spender` The address that will spend the funds.
    /// * `amount` The amount of tokens to be spent.
    ///
    #[endpoint]
    fn approve(&self, spender: ManagedAddress, amount: BigUint) {
        // sender is the caller
        let caller = self.blockchain().get_caller();

        // store allowance
        self.allowance(&caller, &spender).set(&amount);

        // log operation
        self.approve_event(&caller, &spender, &amount);
    }

    // EVENTS

    #[event("transfer")]
    fn transfer_event(
        &self,
        #[indexed] sender: &ManagedAddress,
        #[indexed] recipient: &ManagedAddress,
        amount: &BigUint,
    );

    #[event("approve")]
    fn approve_event(
        &self,
        #[indexed] sender: &ManagedAddress,
        #[indexed] recipient: &ManagedAddress,
        amount: &BigUint,
    );
}
