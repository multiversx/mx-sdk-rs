#![no_std]

imports!();

#[elrond_wasm_derive::contract(NonFungibleTokensImpl)]
pub trait NonFungibleTokens {

    #[init]
    fn init(&self, initial_minted: u64) {
        let owner = self.get_caller();

        self.set_owner(&owner);
        self.perform_mint(initial_minted, &owner);
    }

    // endpoints

    /// Creates new tokens and sets their ownership to the specified account.
    /// Only the contract owner may call this function.
    #[endpoint]
    fn mint(&self, count: u64, new_token_owner: &Address) -> SCResult<()> {
        require!(self.get_caller() == self.get_owner(), "Only owner can mint new tokens!");

        self.perform_mint(count, new_token_owner);

        Ok(())
    }

    /// Approves an account to transfer the token on behalf of its owner.<br>
    /// Only the owner of the token may call this function.
    #[endpoint]
    fn approve(&self, token_id: u64, approved_address: &Address) -> SCResult<()> {
        let caller = self.get_caller();
        let token_owner = self.get_token_owner(token_id);
        let total_minted = self.get_total_minted();

        require!(caller == token_owner, "Only the token owner can approve!");
        require!(token_id < total_minted, "Token does not exist!");

        self.set_approval(token_id, approved_address);

        Ok(())
    }

    /// Revokes approval for the token.<br>  
    /// Only the owner of the token may call this function.
    #[endpoint(revokeApproval)]
    fn revoke_approval(&self, token_id: u64) -> SCResult<()> {
        let caller = self.get_caller();
        let token_owner = self.get_token_owner(token_id);
        let total_minted = self.get_total_minted();

        require!(caller == token_owner, "Only the token owner can revoke approval!");
        require!(token_id < total_minted, "Token does not exist!");

        self.perform_revoke_approval(token_id);

        Ok(())
    }

    /// Transfer ownership of the token to a new account.
    #[endpoint]
    fn transfer(&self, token_id: u64, to: &Address) -> SCResult<()> {
        let caller = self.get_caller();
        let token_owner = self.get_token_owner(token_id);

        if caller == token_owner {
            self.perform_transfer(token_id, to);
        }
        else {
            let approved_address = self.get_approval(token_id);

            if caller == approved_address {
                self.perform_transfer(token_id, to);
            }
            else {
                return sc_error!("Only the owner or the approved account may transfer the token!");
            }
        }

        Ok(())
    }

    // private methods

    fn perform_mint(&self, count: u64, new_token_owner: &Address) {
        let new_owner_current_total = self.get_token_count(new_token_owner);
        let total_minted = self.get_total_minted();
        let first_new_id = total_minted;
        let last_new_id = total_minted + count;

        for id in first_new_id..last_new_id {
            self.set_token_owner(id, new_token_owner);
        }
        
        self.set_total_minted(total_minted + count);
        self.set_token_count(new_token_owner, new_owner_current_total + count);
    }

    fn perform_revoke_approval(&self, token_id: u64) {
        // clear at key "''approval|token_id"
        self.clear_storage_at_key(&["approval".as_bytes(), &token_id.to_be_bytes()]);
    }

    fn perform_transfer(&self, token_id: u64, to: &Address) {
        // new ownership revokes approvals by previous owner
        self.perform_revoke_approval(token_id);
        self.set_token_owner(token_id, to);
    }

    // Storage

    /// Constructs the final key from `key_parts` and clears the storage value addressed by it.  
    fn clear_storage_at_key(&self, key_parts: &[&[u8]]) {
        let mut final_key = Vec::new();

        for key in key_parts {
            final_key.extend_from_slice(key);
        }

        self.storage_store_slice_u8(&final_key, &Vec::new());
    }

    #[view(contractOwner)]
    #[storage_get("owner")]
    fn get_owner(&self) -> Address;

    #[storage_set("owner")]
    fn set_owner(&self, owner: &Address);

    #[view(totalMinted)]
    #[storage_get("totalMinted")]
    fn get_total_minted(&self) -> u64;

    #[storage_set("totalMinted")]
    fn set_total_minted(&self, total_minted: u64);

    #[view(tokenOwner)]
    #[storage_get("tokenOwner")]
    fn get_token_owner(&self, token_id: u64) -> Address;

    #[storage_set("tokenOwner")]
    fn set_token_owner(&self, token_id: u64, owner: &Address);

    #[view(tokenCount)]
    #[storage_get("tokenCount")]
    fn get_token_count(&self, owner: &Address) -> u64;

    #[storage_set("tokenCount")]
    fn set_token_count(&self, owner: &Address, token_count: u64);

    #[view(approval)]
    #[storage_get("approval")]
    fn get_approval(&self, token_id: u64) -> Address;

    #[storage_set("approval")]
    fn set_approval(&self, token_id: u64, approved_address: &Address);
}
