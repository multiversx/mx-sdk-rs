multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub const MAX_DISTRIBUTION_PERCENTAGE: u64 = 100_000; // 100%

#[derive(ManagedVecItem, NestedEncode, NestedDecode, TypeAbi)]
pub struct Distribution<M: ManagedTypeApi> {
    pub address: ManagedAddress<M>,
    pub percentage: u64,
    pub endpoint: ManagedBuffer<M>,
    pub gas_limit: u64,
}

#[multiversx_sc::module]
pub trait DistributionModule {
    fn init_distribution(&self, distribution: ManagedVec<Distribution<Self::Api>>) {
        self.validate_distribution(&distribution);
        self.distribution_rules().set(distribution);
    }

    fn distribute_funds(
        &self,
        token_id: &EgldOrEsdtTokenIdentifier,
        token_nonce: u64,
        total_amount: BigUint,
    ) {
        if total_amount == 0 {
            return;
        }
        for distribution in self.distribution_rules().get().iter() {
            let payment_amount =
                &total_amount * distribution.percentage / MAX_DISTRIBUTION_PERCENTAGE;
            if payment_amount == 0 {
                continue;
            }
            self.send()
                .contract_call::<IgnoreValue>(distribution.address, distribution.endpoint)
                .with_egld_or_single_esdt_transfer((token_id.clone(), token_nonce, payment_amount))
                .with_gas_limit(distribution.gas_limit)
                .transfer_execute();
        }
    }

    fn validate_distribution(&self, distribution: &ManagedVec<Distribution<Self::Api>>) {
        let index_total: u64 = distribution
            .iter()
            .map(|distribution| distribution.percentage)
            .sum();
        require!(
            index_total == MAX_DISTRIBUTION_PERCENTAGE,
            "Distribution percent total must be 100%"
        );
    }

    #[view(getDistributionRules)]
    #[storage_mapper("distributionRules")]
    fn distribution_rules(&self) -> SingleValueMapper<ManagedVec<Distribution<Self::Api>>>;
}
