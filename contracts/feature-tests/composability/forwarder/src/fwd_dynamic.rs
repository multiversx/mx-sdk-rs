use crate::fwd_nft::{CallbackProxy, Color};

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait ForwarderDynamicModule:
    crate::fwd_nft::ForwarderNftModule + crate::fwd_storage::ForwarderStorageModule
{
    #[payable["EGLD"]]
    #[endpoint]
    fn issue_dynamic_token(
        &self,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        token_type: EsdtTokenType,
        num_decimals: usize,
    ) {
        let issue_cost = self.call_value().egld().clone();
        let caller = self.blockchain().get_caller();

        self.send()
            .esdt_system_sc_proxy()
            .issue_dynamic(
                issue_cost,
                token_display_name,
                token_ticker,
                token_type,
                num_decimals,
            )
            .callback(self.callbacks().nft_issue_callback(&caller))
            .async_call_and_exit();
    }

    #[payable["EGLD"]]
    #[endpoint]
    fn issue_token_all_roles(
        &self,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        token_type: EsdtTokenType,
        num_decimals: usize,
    ) {
        let issue_cost = self.call_value().egld().clone();
        let caller = self.blockchain().get_caller();

        self.send()
            .esdt_system_sc_proxy()
            .issue_and_set_all_roles(
                issue_cost,
                token_display_name,
                token_ticker,
                token_type,
                num_decimals,
            )
            .callback(self.callbacks().nft_issue_callback(&caller))
            .async_call_and_exit();
    }

    #[endpoint]
    fn change_to_dynamic(&self, token_id: TokenIdentifier) {
        self.send()
            .esdt_system_sc_proxy()
            .change_to_dynamic(token_id)
            .async_call_and_exit();
    }

    #[endpoint]
    fn update_token(&self, token_id: TokenIdentifier) {
        self.send()
            .esdt_system_sc_proxy()
            .update_token(token_id)
            .async_call_and_exit();
    }

    #[endpoint]
    fn modify_royalties(&self, token_id: TokenIdentifier, nonce: u64, new_royalty: u64) {
        self.send()
            .esdt_modify_royalties(&token_id, nonce, new_royalty);
    }

    #[endpoint]
    fn set_new_uris(
        &self,
        token_id: TokenIdentifier,
        nonce: u64,
        new_uris: MultiValueEncoded<ManagedBuffer>,
    ) {
        let new_uris = new_uris.to_vec();
        self.send()
            .esdt_nft_set_new_uris(&token_id, nonce, &new_uris);
    }

    #[endpoint]
    fn modify_creator(&self, token_id: TokenIdentifier, nonce: u64) {
        self.send().esdt_nft_modify_creator(&token_id, nonce);
    }

    #[endpoint]
    fn metadata_recreate(
        &self,
        token_id: TokenIdentifier,
        nonce: u64,
        name: ManagedBuffer,
        royalties: u64,
        hash: ManagedBuffer,
        new_attributes: Color,
        uris: MultiValueEncoded<ManagedBuffer>,
    ) {
        let uris = uris.to_vec();

        self.send().esdt_metadata_recreate(
            token_id,
            nonce,
            name,
            royalties,
            hash,
            &new_attributes,
            uris,
        );
    }

    #[endpoint]
    fn metadata_update(
        &self,
        token_id: TokenIdentifier,
        nonce: u64,
        name: ManagedBuffer,
        royalties: u64,
        hash: ManagedBuffer,
        new_attributes: Color,
        uris: MultiValueEncoded<ManagedBuffer>,
    ) {
        let uris = uris.to_vec();

        self.send().esdt_metadata_update(
            token_id,
            nonce,
            name,
            royalties,
            hash,
            &new_attributes,
            uris,
        );
    }
}
