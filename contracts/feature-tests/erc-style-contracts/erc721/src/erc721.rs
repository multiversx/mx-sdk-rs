#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait NonFungibleTokens {
    #[init]
    fn init(&self, initial_minted: u64) {
        let owner = self.blockchain().get_caller();
        self.perform_mint(initial_minted, &owner);
    }

    // endpoints

    /// Creates new tokens and sets their ownership to the specified account.
    /// Only the contract owner may call this function.
    #[only_owner]
    #[endpoint]
    fn mint(&self, count: u64, new_token_owner: ManagedAddress) {
        self.perform_mint(count, &new_token_owner);
    }

    /// Approves an account to transfer the token on behalf of its owner.<br>
    /// Only the owner of the token may call this function.
    #[endpoint]
    fn approve(&self, token_id: u64, approved_address: ManagedAddress) {
        require!(
            token_id < self.total_minted().get(),
            "Token does not exist!"
        );
        require!(
            self.blockchain().get_caller() == self.token_owner(token_id).get(),
            "Only the token owner can approve!"
        );

        self.approval(token_id).set(&approved_address);
    }

    /// Revokes approval for the token.<br>  
    /// Only the owner of the token may call this function.
    #[endpoint]
    fn revoke(&self, token_id: u64) {
        require!(
            token_id < self.total_minted().get(),
            "Token does not exist!"
        );
        require!(
            self.blockchain().get_caller() == self.token_owner(token_id).get(),
            "Only the token owner can revoke approval!"
        );

        self.approval(token_id).clear();
    }

    /// Transfer ownership of the token to a new account.
    #[endpoint]
    fn transfer(&self, token_id: u64, to: ManagedAddress) {
        require!(
            token_id < self.total_minted().get(),
            "Token does not exist!"
        );

        let caller = self.blockchain().get_caller();
        let token_owner = self.token_owner(token_id).get();

        if caller == token_owner {
            self.perform_transfer(token_id, &token_owner, &to);

            return;
        } else if !self.approval(token_id).is_empty() {
            let approved_address = self.approval(token_id).get();

            if caller == approved_address {
                self.perform_transfer(token_id, &token_owner, &to);

                return;
            }
        }

        sc_panic!("Only the owner or the approved account may transfer the token!")
    }

    // private methods

    fn perform_mint(&self, count: u64, new_token_owner: &ManagedAddress) {
        let new_owner_current_total = self.token_count(new_token_owner).get();
        let total_minted = self.total_minted().get();
        let first_new_id = total_minted;
        let last_new_id = total_minted + count;

        for id in first_new_id..last_new_id {
            self.token_owner(id).set(new_token_owner);
        }

        self.total_minted().set(total_minted + count);
        self.token_count(new_token_owner)
            .set(new_owner_current_total + count);
    }

    fn perform_transfer(&self, token_id: u64, from: &ManagedAddress, to: &ManagedAddress) {
        self.token_count(from).update(|count| *count -= 1);
        self.token_count(to).update(|count| *count += 1);
        self.token_owner(token_id).set(to);

        // new ownership revokes approvals by previous owner
        self.approval(token_id).clear();
    }

    // storage

    #[view(totalMinted)]
    #[storage_mapper("totalMinted")]
    fn total_minted(&self) -> SingleValueMapper<u64>;

    #[view(tokenOwner)]
    #[storage_mapper("tokenOwner")]
    fn token_owner(&self, token_id: u64) -> SingleValueMapper<ManagedAddress>;

    #[view(tokenCount)]
    #[storage_mapper("tokenCount")]
    fn token_count(&self, owner: &ManagedAddress) -> SingleValueMapper<u64>;

    #[view(approval)]
    #[storage_mapper("approval")]
    fn approval(&self, token_id: u64) -> SingleValueMapper<ManagedAddress>;
}
