multiversx_sc::imports!();

#[multiversx_sc::derive::proxy]
pub trait UserBuiltin {
    #[endpoint(SetUserName)]
    fn set_user_name(&self, name: &ManagedBuffer);

    #[endpoint(DeleteUserName)]
    fn delete_user_name(&self);
}
