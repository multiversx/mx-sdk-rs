elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use super::storage;

// used as mock attributes for NFTs
#[derive(TopEncode, TopDecode, TypeAbi)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(TopEncode, TopDecode, TypeAbi, PartialEq, Clone)]
pub struct ComplexAttributes<M: ManagedTypeApi> {
    pub biguint: BigUint<M>,
    pub vec_u8: ManagedBuffer<M>,
    pub token_id: TokenIdentifier<M>,
    pub boolean: bool,
    pub boxed_bytes: ManagedBuffer<M>,
}

#[elrond_wasm::module]
pub trait ForwarderNftModule: storage::ForwarderStorageModule {
    #[view]
    fn get_nft_balance(&self, token_identifier: &TokenIdentifier, nonce: u64) -> BigUint {
        self.blockchain().get_esdt_balance(
            &self.blockchain().get_sc_address(),
            token_identifier,
            nonce,
        )
    }

    #[payable("*")]
    #[endpoint]
    fn buy_nft(
        &self,
        //#[payment_token] payment_token: TokenIdentifier,
        //#[payment_nonce] payment_nonce: u64,
        //#[payment_amount] payment_amount: BigUint,
        nft_id: TokenIdentifier,
        nft_nonce: u64,
        nft_amount: BigUint,
    ) -> BigUint {
        let (payment_amount, payment_token) = self.call_value().payment_token_pair();
        let payment_nonce = self.call_value().esdt_token_nonce();

        self.send().sell_nft(
            &nft_id,
            nft_nonce,
            &nft_amount,
            &self.blockchain().get_caller(),
            &payment_token,
            payment_nonce,
            &payment_amount,
        )
    }

    #[payable("EGLD")]
    #[endpoint]
    fn nft_issue(
        &self,
        #[payment] issue_cost: BigUint,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
    ) -> AsyncCall {
        let caller = self.blockchain().get_caller();

        self.send()
            .esdt_system_sc_proxy()
            .issue_non_fungible(
                issue_cost,
                &token_display_name,
                &token_ticker,
                NonFungibleTokenProperties {
                    can_freeze: true,
                    can_wipe: true,
                    can_pause: true,
                    can_change_owner: true,
                    can_upgrade: true,
                    can_add_special_roles: true,
                },
            )
            .async_call()
            .with_callback(self.callbacks().nft_issue_callback(&caller))
    }

    #[callback]
    fn nft_issue_callback(
        &self,
        caller: &ManagedAddress,
        #[call_result] result: ManagedAsyncCallResult<TokenIdentifier>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(token_identifier) => {
                self.last_issued_token().set(&token_identifier);
                self.last_error_message().clear();
            },
            ManagedAsyncCallResult::Err(message) => {
                // return issue cost to the caller
                let (returned_tokens, token_identifier) = self.call_value().payment_token_pair();
                if token_identifier.is_egld() && returned_tokens > 0 {
                    self.send().direct_egld(caller, &returned_tokens, &[]);
                }

                self.last_error_message().set(&message.err_msg);
            },
        }
    }

    #[endpoint]
    fn nft_create(
        &self,
        token_identifier: TokenIdentifier,
        amount: BigUint,
        name: ManagedBuffer,
        royalties: BigUint,
        hash: ManagedBuffer,
        color: Color,
        uri: ManagedBuffer,
    ) -> u64 {
        let mut uris = ManagedVec::new();
        uris.push(uri);
        let token_nonce = self.send().esdt_nft_create::<Color>(
            &token_identifier,
            &amount,
            &name,
            &royalties,
            &hash,
            &color,
            &uris,
        );

        self.create_event(&token_identifier, token_nonce, &amount);

        token_nonce
    }

    #[endpoint]
    fn nft_create_on_caller_behalf(
        &self,
        token_identifier: TokenIdentifier,
        amount: BigUint,
        name: ManagedBuffer,
        royalties: BigUint,
        hash: ManagedBuffer,
        color: Color,
        uri: ManagedBuffer,
    ) -> u64 {
        let mut uris = ManagedVec::new();
        uris.push(uri);

        self.send().esdt_nft_create_as_caller::<Color>(
            &token_identifier,
            &amount,
            &name,
            &royalties,
            &hash,
            &color,
            &uris,
        )
    }

    #[endpoint]
    fn nft_decode_complex_attributes(
        &self,
        token_identifier: TokenIdentifier,
        amount: BigUint,
        name: ManagedBuffer,
        royalties: BigUint,
        hash: ManagedBuffer,
        uri: ManagedBuffer,
        #[var_args] attrs_arg: MultiArg5<
            BigUint,
            ManagedBuffer,
            TokenIdentifier,
            bool,
            ManagedBuffer,
        >,
    ) -> SCResult<()> {
        let attrs_pieces = attrs_arg.into_tuple();
        let orig_attr = ComplexAttributes {
            biguint: attrs_pieces.0,
            vec_u8: attrs_pieces.1,
            token_id: attrs_pieces.2,
            boolean: attrs_pieces.3,
            boxed_bytes: attrs_pieces.4,
        };

        let mut uris = ManagedVec::new();
        uris.push(uri);
        let token_nonce = self.send().esdt_nft_create::<ComplexAttributes<Self::Api>>(
            &token_identifier,
            &amount,
            &name,
            &royalties,
            &hash,
            &orig_attr,
            &uris,
        );

        let token_info = self.blockchain().get_esdt_token_data(
            &self.blockchain().get_sc_address(),
            &token_identifier,
            token_nonce,
        );

        let decoded_attr = token_info.decode_attributes::<ComplexAttributes<Self::Api>>()?;

        require!(
            orig_attr.biguint == decoded_attr.biguint
                && orig_attr.vec_u8 == decoded_attr.vec_u8
                && orig_attr.token_id == decoded_attr.token_id
                && orig_attr.boolean == decoded_attr.boolean
                && orig_attr.boxed_bytes == decoded_attr.boxed_bytes,
            "orig_attr != decoded_attr"
        );
        Ok(())
    }

    #[endpoint]
    fn nft_add_quantity(&self, token_identifier: TokenIdentifier, nonce: u64, amount: BigUint) {
        self.send()
            .esdt_local_mint(&token_identifier, nonce, &amount);
    }

    #[endpoint]
    fn nft_burn(&self, token_identifier: TokenIdentifier, nonce: u64, amount: BigUint) {
        self.send()
            .esdt_local_burn(&token_identifier, nonce, &amount);
    }

    #[endpoint]
    fn transfer_nft_via_async_call(
        &self,
        to: ManagedAddress,
        token_identifier: TokenIdentifier,
        nonce: u64,
        amount: BigUint,
        data: ManagedBuffer,
    ) {
        self.send()
            .transfer_esdt_via_async_call(&to, &token_identifier, nonce, &amount, data);
    }

    #[endpoint]
    fn transfer_nft_and_execute(
        &self,
        to: ManagedAddress,
        token_identifier: TokenIdentifier,
        nonce: u64,
        amount: BigUint,
        function: ManagedBuffer,
        #[var_args] arguments: ManagedVarArgs<ManagedBuffer>,
    ) {
        let _ = self.raw_vm_api().direct_esdt_nft_execute(
            &to,
            &token_identifier,
            nonce,
            &amount,
            self.blockchain().get_gas_left(),
            &function,
            &arguments.to_arg_buffer(),
        );
    }

    #[endpoint]
    fn create_and_send(
        &self,
        to: ManagedAddress,
        token_identifier: TokenIdentifier,
        amount: BigUint,
        name: ManagedBuffer,
        royalties: BigUint,
        hash: ManagedBuffer,
        color: Color,
        uri: ManagedBuffer,
    ) {
        let token_nonce = self.nft_create(
            token_identifier.clone(),
            amount.clone(),
            name,
            royalties,
            hash,
            color,
            uri,
        );

        self.send().direct(
            &to,
            &token_identifier,
            token_nonce,
            &amount,
            b"NFT transfer",
        );

        self.send_event(&to, &token_identifier, token_nonce, &amount);
    }

    #[event("create")]
    fn create_event(
        &self,
        #[indexed] token_id: &TokenIdentifier,
        #[indexed] token_nonce: u64,
        #[indexed] amount: &BigUint,
    );

    #[event("send")]
    fn send_event(
        &self,
        #[indexed] to: &ManagedAddress,
        #[indexed] token_id: &TokenIdentifier,
        #[indexed] token_nonce: u64,
        #[indexed] amount: &BigUint,
    );
}
