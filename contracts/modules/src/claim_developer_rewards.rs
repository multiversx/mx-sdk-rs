multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait ClaimDeveloperRewardsModule {
    #[endpoint(claimDeveloperRewards)]
    fn claim_developer_rewards(&self, child_sc_address: ManagedAddress) {
        self.tx()
            .to(&child_sc_address)
            .typed(system_proxy::UserBuiltinProxy)
            .claim_developer_rewards()
            .async_call_and_exit();
    }
}
