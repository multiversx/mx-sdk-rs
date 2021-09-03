#![no_std]

elrond_wasm::imports!();

/// The module deals with temporarily pausing contract operations.
/// It provides a flag that contracts can use to check if owner decided to users the entire contract.
/// Use the features module for more granular on/off switches.
#[elrond_wasm::module]
pub trait UsersModule {
    /// Each user gets a user id. This is in order to be able to iterate over their data.
    /// This is a mapping from user address to user id.
    /// The key is the bytes "user_id" concatenated with their public key.
    /// The value is the user id.
    #[view(getUserId)]
    #[storage_get("user_id")]
    fn get_user_id(&self, address: &ManagedAddress) -> usize;

    #[storage_set("user_id")]
    fn set_user_id(&self, address: &ManagedAddress, user_id: usize);

    #[view(getUserAddress)]
    #[storage_get("user_address")]
    fn get_user_address(&self, user_id: usize) -> ManagedAddress;

    #[storage_set("user_address")]
    fn set_user_address(&self, user_id: usize, address: &ManagedAddress);

    /// Retrieves the number of delegtors, including the owner,
    /// even if they no longer have anything in the contract.
    #[view(getNumUsers)]
    #[storage_get("num_users")]
    fn get_num_users(&self) -> usize;

    /// Yields how accounts are registered in the contract.
    /// Note that not all of them must have stakes greater than zero.
    #[storage_set("num_users")]
    fn set_num_users(&self, num_users: usize);

    fn get_or_create_user(&self, address: &ManagedAddress) -> usize {
        let mut user_id = self.get_user_id(address);
        if user_id == 0 {
            let mut num_users = self.get_num_users();
            num_users += 1;
            self.set_num_users(num_users);
            user_id = num_users;
            self.set_user_id(address, user_id);
            self.set_user_address(user_id, address);
        }
        user_id
    }

    #[endpoint(updateUserAddress)]
    fn update_user_address(&self, #[var_args] addresses: VarArgs<ManagedAddress>) -> SCResult<()> {
        for address in addresses.into_vec() {
            let user_id = self.get_user_id(&address);
            require!(user_id > 0, "unknown address");
            self.set_user_address(user_id, &address);
        }
        Ok(())
    }
}
