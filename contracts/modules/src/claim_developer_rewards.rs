multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait ClaimDeveloperRewardsModule {
    #[endpoint(claimDeveloperRewards)]
    fn claim_developer_rewards(&self, child_sc_address: ManagedAddress) {
        self.send()
            .claim_developer_rewards(child_sc_address)
            .sync_call();
    }
}
