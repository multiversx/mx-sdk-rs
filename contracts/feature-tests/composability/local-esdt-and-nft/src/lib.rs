#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

// used as mock attributes for NFTs
#[derive(TopEncode, TopDecode, TypeAbi)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

#[multiversx_sc::contract]
pub trait LocalEsdtAndEsdtNft {
    #[init]
    fn init(&self) {}

    // Fungible Tokens

    #[payable("EGLD")]
    #[endpoint(issueFungibleToken)]
    fn issue_fungible_token(
        &self,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        initial_supply: BigUint,
    ) {
        let issue_cost = self.call_value().egld_value();
        let caller = self.blockchain().get_caller();

        self.send()
            .esdt_system_sc_proxy()
            .issue_fungible(
                issue_cost.clone_value(),
                &token_display_name,
                &token_ticker,
                &initial_supply,
                FungibleTokenProperties {
                    num_decimals: 0,
                    can_freeze: true,
                    can_wipe: true,
                    can_pause: true,
                    can_mint: true,
                    can_burn: true,
                    can_change_owner: true,
                    can_upgrade: true,
                    can_add_special_roles: true,
                },
            )
            .async_call()
            .with_callback(self.callbacks().esdt_issue_callback(&caller))
            .call_and_exit()
    }

    #[endpoint(localMint)]
    fn local_mint(&self, token_identifier: TokenIdentifier, amount: BigUint) {
        self.send().esdt_local_mint(&token_identifier, 0, &amount);
    }

    #[endpoint(localBurn)]
    fn local_burn(&self, token_identifier: TokenIdentifier, amount: BigUint) {
        self.send().esdt_local_burn(&token_identifier, 0, &amount);
    }

    // Non-Fungible Tokens

    #[payable("EGLD")]
    #[endpoint(nftIssue)]
    fn nft_issue(&self, token_display_name: ManagedBuffer, token_ticker: ManagedBuffer) {
        let issue_cost = self.call_value().egld_value();
        let caller = self.blockchain().get_caller();

        self.send()
            .esdt_system_sc_proxy()
            .issue_non_fungible(
                issue_cost.clone_value(),
                &token_display_name,
                &token_ticker,
                NonFungibleTokenProperties {
                    can_freeze: true,
                    can_wipe: true,
                    can_pause: true,
                    can_transfer_create_role: true,
                    can_change_owner: true,
                    can_upgrade: true,
                    can_add_special_roles: true,
                },
            )
            .async_call()
            .with_callback(self.callbacks().nft_issue_callback(&caller))
            .call_and_exit()
    }

    #[endpoint(nftCreate)]
    #[allow(clippy::too_many_arguments)]
    fn nft_create(
        &self,
        token_identifier: TokenIdentifier,
        amount: BigUint,
        name: ManagedBuffer,
        royalties: BigUint,
        hash: ManagedBuffer,
        color: Color,
        uri: ManagedBuffer,
    ) {
        let mut uris = ManagedVec::new();
        uris.push(uri);

        self.send().esdt_nft_create::<Color>(
            &token_identifier,
            &amount,
            &name,
            &royalties,
            &hash,
            &color,
            &uris,
        );
    }

    #[endpoint(nftAddQuantity)]
    fn nft_add_quantity(&self, token_identifier: TokenIdentifier, nonce: u64, amount: BigUint) {
        self.send()
            .esdt_local_mint(&token_identifier, nonce, &amount);
    }

    #[endpoint(nftBurn)]
    fn nft_burn(&self, token_identifier: TokenIdentifier, nonce: u64, amount: BigUint) {
        self.send()
            .esdt_local_burn(&token_identifier, nonce, &amount);
    }

    #[endpoint(transferNftViaAsyncCall)]
    fn transfer_nft_via_async_call(
        &self,
        to: ManagedAddress,
        token_identifier: TokenIdentifier,
        nonce: u64,
        amount: BigUint,
    ) {
        self.send()
            .transfer_esdt_via_async_call(to, token_identifier, nonce, amount);
    }

    #[endpoint]
    fn transfer_nft_and_execute(
        &self,
        to: ManagedAddress,
        token_identifier: TokenIdentifier,
        nonce: u64,
        amount: BigUint,
        function: ManagedBuffer,
        arguments: MultiValueEncoded<ManagedBuffer>,
    ) {
        let mut arg_buffer = ManagedArgBuffer::new();
        for arg in arguments.into_iter() {
            arg_buffer.push_arg_raw(arg);
        }

        let _ = self.send_raw().transfer_esdt_nft_execute(
            &to,
            &token_identifier,
            nonce,
            &amount,
            self.blockchain().get_gas_left(),
            &function,
            &arg_buffer,
        );
    }

    // Semi-Fungible

    #[payable("EGLD")]
    #[endpoint(sftIssue)]
    fn sft_issue(&self, token_display_name: ManagedBuffer, token_ticker: ManagedBuffer) {
        let issue_cost = self.call_value().egld_value();
        let caller = self.blockchain().get_caller();

        self.send()
            .esdt_system_sc_proxy()
            .issue_semi_fungible(
                issue_cost.clone_value(),
                &token_display_name,
                &token_ticker,
                SemiFungibleTokenProperties {
                    can_freeze: true,
                    can_wipe: true,
                    can_pause: true,
                    can_transfer_create_role: true,
                    can_change_owner: true,
                    can_upgrade: true,
                    can_add_special_roles: true,
                },
            )
            .async_call()
            .with_callback(self.callbacks().nft_issue_callback(&caller))
            .call_and_exit()
    }

    // common

    #[endpoint(setLocalRoles)]
    fn set_local_roles(
        &self,
        address: ManagedAddress,
        token_identifier: TokenIdentifier,
        roles: MultiValueEncoded<EsdtLocalRole>,
    ) {
        self.send()
            .esdt_system_sc_proxy()
            .set_special_roles(&address, &token_identifier, roles.into_iter())
            .async_call()
            .with_callback(self.callbacks().change_roles_callback())
            .call_and_exit()
    }

    #[endpoint(unsetLocalRoles)]
    fn unset_local_roles(
        &self,
        address: ManagedAddress,
        token_identifier: TokenIdentifier,
        roles: MultiValueEncoded<EsdtLocalRole>,
    ) {
        self.send()
            .esdt_system_sc_proxy()
            .unset_special_roles(&address, &token_identifier, roles.into_iter())
            .async_call()
            .with_callback(self.callbacks().change_roles_callback())
            .call_and_exit()
    }

    #[endpoint(controlChanges)]
    fn control_changes(&self, token: TokenIdentifier) {
        let property_arguments = TokenPropertyArguments {
            can_freeze: Some(true),
            can_burn: Some(true),
            ..Default::default()
        };

        self.send()
            .esdt_system_sc_proxy()
            .control_changes(&token, &property_arguments)
            .async_call()
            .call_and_exit();
    }

    // views

    #[view(getFungibleEsdtBalance)]
    fn get_fungible_esdt_balance(&self, token_identifier: &TokenIdentifier) -> BigUint {
        self.blockchain()
            .get_esdt_balance(&self.blockchain().get_sc_address(), token_identifier, 0)
    }

    #[view(getNftBalance)]
    fn get_nft_balance(&self, token_identifier: &TokenIdentifier, nonce: u64) -> BigUint {
        self.blockchain().get_esdt_balance(
            &self.blockchain().get_sc_address(),
            token_identifier,
            nonce,
        )
    }

    #[view(getCurrentNftNonce)]
    fn get_current_nft_nonce(&self, token_identifier: &TokenIdentifier) -> u64 {
        self.blockchain()
            .get_current_esdt_nft_nonce(&self.blockchain().get_sc_address(), token_identifier)
    }

    // callbacks

    #[callback]
    fn esdt_issue_callback(
        &self,
        caller: &ManagedAddress,
        #[call_result] result: ManagedAsyncCallResult<()>,
    ) {
        let (token_identifier, returned_tokens) = self.call_value().egld_or_single_fungible_esdt();
        // callback is called with ESDTTransfer of the newly issued token, with the amount requested,
        // so we can get the token identifier and amount from the call data
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                self.last_issued_token()
                    .set(&token_identifier.unwrap_esdt());
                self.last_error_message().clear();
            },
            ManagedAsyncCallResult::Err(message) => {
                // return issue cost to the caller
                if token_identifier.is_egld() && returned_tokens > 0 {
                    self.send().direct_egld(caller, &returned_tokens);
                }

                self.last_error_message().set(&message.err_msg);
            },
        }
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
                let (token_identifier, returned_tokens) =
                    self.call_value().egld_or_single_fungible_esdt();
                if token_identifier.is_egld() && returned_tokens > 0 {
                    self.send().direct_egld(caller, &returned_tokens);
                }

                self.last_error_message().set(&message.err_msg);
            },
        }
    }

    #[callback]
    fn change_roles_callback(&self, #[call_result] result: ManagedAsyncCallResult<()>) {
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                self.last_error_message().clear();
            },
            ManagedAsyncCallResult::Err(message) => {
                self.last_error_message().set(&message.err_msg);
            },
        }
    }

    // storage

    #[view(lastIssuedToken)]
    #[storage_mapper("lastIssuedToken")]
    fn last_issued_token(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(lastErrorMessage)]
    #[storage_mapper("lastErrorMessage")]
    fn last_error_message(&self) -> SingleValueMapper<ManagedBuffer>;
}
