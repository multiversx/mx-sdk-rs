use multiversx_sc_codec::{Empty, TopEncode};

use crate::types::{
    BigUint, ManagedAddress, ManagedBuffer, ManagedVec, NotPayable, ProxyArg, TokenIdentifier, Tx,
    TxEnv, TxFrom, TxGas, TxProxyTrait, TxTo, TxTypedCall,
};

use crate::vm_core::builtin_func_names::{
    CHANGE_OWNER_BUILTIN_FUNC_NAME, CLAIM_DEVELOPER_REWARDS_FUNC_NAME, DELETE_USERNAME_FUNC_NAME,
    ESDT_LOCAL_BURN_FUNC_NAME, ESDT_LOCAL_MINT_FUNC_NAME, ESDT_NFT_ADD_QUANTITY_FUNC_NAME,
    ESDT_NFT_ADD_URI_FUNC_NAME, ESDT_NFT_BURN_FUNC_NAME, ESDT_NFT_CREATE_FUNC_NAME,
    ESDT_NFT_UPDATE_ATTRIBUTES_FUNC_NAME, SET_USERNAME_FUNC_NAME,
};

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
    pub fn set_user_name<Arg0: ProxyArg<ManagedBuffer<Env::Api>>>(
        self,
        name: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call(SET_USERNAME_FUNC_NAME)
            .argument(&name)
            .original_result()
    }

    pub fn delete_user_name(self) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call(DELETE_USERNAME_FUNC_NAME)
            .original_result()
    }

    pub fn claim_developer_rewards(self) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call(CLAIM_DEVELOPER_REWARDS_FUNC_NAME)
            .original_result()
    }

    pub fn change_owner_address(
        self,
        new_owner: &ManagedAddress<Env::Api>,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call(CHANGE_OWNER_BUILTIN_FUNC_NAME)
            .argument(&new_owner)
            .original_result()
    }

    pub fn esdt_local_burn<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg1: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        token: Arg0,
        nonce: u64,
        amount: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        if nonce == 0 {
            return self
                .wrapped_tx
                .payment(NotPayable)
                .raw_call(ESDT_LOCAL_BURN_FUNC_NAME)
                .argument(&token)
                .argument(&amount)
                .original_result();
        }

        self.wrapped_tx
            .payment(NotPayable)
            .raw_call(ESDT_NFT_BURN_FUNC_NAME)
            .argument(&token)
            .argument(&nonce)
            .argument(&amount)
            .original_result()
    }

    pub fn esdt_local_mint<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg1: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        token: Arg0,
        nonce: u64,
        amount: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        if nonce == 0 {
            return self
                .wrapped_tx
                .payment(NotPayable)
                .raw_call(ESDT_LOCAL_MINT_FUNC_NAME)
                .argument(&token)
                .argument(&amount)
                .original_result();
        }
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call(ESDT_NFT_ADD_QUANTITY_FUNC_NAME)
            .argument(&token)
            .argument(&nonce)
            .argument(&amount)
            .original_result()
    }

    pub fn nft_add_multiple_uri<Arg0: ProxyArg<TokenIdentifier<Env::Api>>>(
        self,
        token_id: Arg0,
        nft_nonce: u64,
        new_uris: &ManagedVec<Env::Api, ManagedBuffer<Env::Api>>,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        let mut tx = self
            .wrapped_tx
            .payment(NotPayable)
            .raw_call(ESDT_NFT_ADD_URI_FUNC_NAME)
            .argument(&token_id)
            .argument(&nft_nonce);

        for uri in new_uris {
            tx = tx.argument(&uri);
        }

        tx.original_result()
    }

    pub fn nft_update_attributes<T: TopEncode, Arg0: ProxyArg<TokenIdentifier<Env::Api>>>(
        self,
        token_id: Arg0,
        nft_nonce: u64,
        new_attributes: &T,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call(ESDT_NFT_UPDATE_ATTRIBUTES_FUNC_NAME)
            .argument(&token_id)
            .argument(&nft_nonce)
            .argument(&new_attributes)
            .original_result()
    }

    #[allow(clippy::too_many_arguments)]
    pub fn esdt_nft_create<
        T: TopEncode,
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg1: ProxyArg<BigUint<Env::Api>>,
        Arg2: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg3: ProxyArg<BigUint<Env::Api>>,
        Arg4: ProxyArg<ManagedBuffer<Env::Api>>,
    >(
        self,
        token: Arg0,
        amount: Arg1,
        name: Arg2,
        royalties: Arg3,
        hash: Arg4,
        attributes: &T,
        uris: &ManagedVec<Env::Api, ManagedBuffer<Env::Api>>,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, u64> {
        let mut tx = self
            .wrapped_tx
            .payment(NotPayable)
            .raw_call(ESDT_NFT_CREATE_FUNC_NAME)
            .argument(&token)
            .argument(&amount)
            .argument(&name)
            .argument(&royalties)
            .argument(&hash)
            .argument(&attributes);

        if uris.is_empty() {
            // at least one URI is required, so we push an empty one
            tx = tx.argument(&Empty);
        } else {
            // The API function has the last argument as variadic,
            // so we top-encode each and send as separate argument
            for uri in uris {
                tx = tx.argument(&uri);
            }
        }

        tx.original_result()
    }
}
