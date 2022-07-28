elrond_wasm::imports!();

#[elrond_wasm::module]
pub trait ClaimDeveloperRewardsModule {
    #[proxy]
    fn vault_proxy(&self) -> vault::Proxy<Self::Api>;

    #[endpoint(claimDeveloperRewards)]
    fn claim_developer_rewards(&self, child_sc_address: ManagedAddress) {
        let () = self
            .send()
            .claim_developer_rewards(child_sc_address)
            .execute_on_dest_context();
    }
}
