use self::builtin_func_names::{
    CHANGE_OWNER_BUILTIN_FUNC_NAME, CLAIM_DEVELOPER_REWARDS_FUNC_NAME, DELETE_USERNAME_FUNC_NAME,
    ESDT_LOCAL_BURN_FUNC_NAME, ESDT_LOCAL_MINT_FUNC_NAME, ESDT_NFT_ADD_QUANTITY_FUNC_NAME,
    ESDT_NFT_ADD_URI_FUNC_NAME, ESDT_NFT_BURN_FUNC_NAME, ESDT_NFT_CREATE_FUNC_NAME,
    ESDT_NFT_UPDATE_ATTRIBUTES_FUNC_NAME, SET_USERNAME_FUNC_NAME,
};
use crate::proxy_imports::*;

/// Proxy describing the user builtin function signatures.
pub struct UserBuiltinProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for UserBuiltinProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = UserBuiltinProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        UserBuiltinProxyMethods { wrapped_tx: tx }
    }
}

pub struct UserBuiltinProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

impl<Env, From, To, Gas> UserBuiltinProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn set_user_name<Arg0: CodecInto<ManagedBuffer<Env::Api>>>(
        self,
        name: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call(SET_USERNAME_FUNC_NAME)
            .argument(&name)
            .original_result()
    }

    pub fn delete_user_name(self) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call(DELETE_USERNAME_FUNC_NAME)
            .original_result()
    }

    pub fn claim_developer_rewards(self) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call(CLAIM_DEVELOPER_REWARDS_FUNC_NAME)
            .original_result()
    }

    pub fn change_owner_address(
        self,
        new_owner: &ManagedAddress<Env::Api>,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call(CHANGE_OWNER_BUILTIN_FUNC_NAME)
            .argument(new_owner)
            .original_result()
    }

    pub fn esdt_local_burn(
        self,
        token: &TokenIdentifier<Env::Api>,
        nonce: u64,
        amount: &BigUint<Env::Api>,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        if nonce == 0 {
            return self
                .wrapped_tx
                .raw_call(ESDT_LOCAL_BURN_FUNC_NAME)
                .argument(token)
                .argument(amount)
                .original_result();
        }

        self.wrapped_tx
            .raw_call(ESDT_NFT_BURN_FUNC_NAME)
            .argument(token)
            .argument(&nonce)
            .argument(amount)
            .original_result()
    }

    pub fn esdt_local_mint(
        self,
        token: &TokenIdentifier<Env::Api>,
        nonce: u64,
        amount: &BigUint<Env::Api>,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        if nonce == 0 {
            return self
                .wrapped_tx
                .raw_call(ESDT_LOCAL_MINT_FUNC_NAME)
                .argument(token)
                .argument(amount)
                .original_result();
        }
        self.wrapped_tx
            .raw_call(ESDT_NFT_ADD_QUANTITY_FUNC_NAME)
            .argument(token)
            .argument(&nonce)
            .argument(amount)
            .original_result()
    }

    pub fn nft_add_multiple_uri(
        self,
        token_id: &TokenIdentifier<Env::Api>,
        nft_nonce: u64,
        new_uris: &ManagedVec<Env::Api, ManagedBuffer<Env::Api>>,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        let mut arg_buffer = ManagedArgBuffer::new();
        arg_buffer.push_arg(token_id);
        arg_buffer.push_arg(nft_nonce);

        for uri in new_uris {
            arg_buffer.push_arg(uri);
        }

        self.wrapped_tx
            .raw_call(ESDT_NFT_ADD_URI_FUNC_NAME)
            .arguments_raw(arg_buffer)
            .original_result()
    }
}
