#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

mod nft_module;

pub const MAX_PERCENTAGE: u64 = 100_000; // 100%

#[derive(ManagedVecItem, NestedEncode, NestedDecode, TypeAbi)]
pub struct Distribution<M: ManagedTypeApi> {
    pub address: ManagedAddress<M>,
    pub percentage: u64,
    pub endpoint: ManagedBuffer<M>,
    pub gas_limit: u64,
}

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct ExampleAttributes {
    pub creation_timestamp: u64,
}

#[elrond_wasm::contract]
pub trait SeedNftMinter: nft_module::NftModule {
    #[init]
    fn init(
        &self,
        marketplaces: ManagedVec<ManagedAddress>,
        distribution: ManagedVec<Distribution<Self::Api>>,
    ) {
        for marketplace in &marketplaces {
            self.marketplaces().insert(marketplace);
        }
        self.validate_distribution(&distribution);
        self.distribution_rules().set(distribution);
    }

    #[only_owner]
    #[endpoint(createNft)]
    fn create_nft(
        &self,
        name: ManagedBuffer,
        royalties: BigUint,
        uri: ManagedBuffer,
        selling_price: BigUint,
        opt_token_used_as_payment: OptionalValue<TokenIdentifier>,
        opt_token_used_as_payment_nonce: OptionalValue<u64>,
    ) {
        let token_used_as_payment = match opt_token_used_as_payment {
            OptionalValue::Some(token) => EgldOrEsdtTokenIdentifier::esdt(token),
            OptionalValue::None => EgldOrEsdtTokenIdentifier::egld(),
        };
        require!(
            token_used_as_payment.is_valid(),
            "Invalid token_used_as_payment arg, not a valid token ID"
        );

        let token_used_as_payment_nonce = if token_used_as_payment.is_egld() {
            0
        } else {
            match opt_token_used_as_payment_nonce {
                OptionalValue::Some(nonce) => nonce,
                OptionalValue::None => 0,
            }
        };

        let attributes = ExampleAttributes {
            creation_timestamp: self.blockchain().get_block_timestamp(),
        };
        let nft_nonce = self.create_nft_with_attributes(
            name,
            royalties,
            attributes,
            uri,
            selling_price,
            token_used_as_payment,
            token_used_as_payment_nonce,
        );

        self.nft_count().set(nft_nonce);
    }

    #[only_owner]
    #[endpoint(claimAndDistribute)]
    fn claim_and_distribute(&self) {
        let total_amount = self.claim_royalties(EgldOrEsdtTokenIdentifier::egld(), 0);
        self.distribute_royalties(total_amount);
    }

    fn claim_royalties(&self, token_id: EgldOrEsdtTokenIdentifier, token_nonce: u64) -> BigUint {
        let claim_destination = self.blockchain().get_sc_address();
        let mut total_amount = BigUint::zero();
        for address in self.marketplaces().iter() {
            let results: MultiValue2<BigUint, IgnoreValue> = self
                .marketplace_proxy(address)
                .claim_tokens(&claim_destination, &token_id, token_nonce)
                .execute_on_dest_context();

            let (claimed_amount, _) = results.into_tuple();
            total_amount += claimed_amount;
        }

        total_amount
    }

    fn distribute_royalties(&self, total_amount: BigUint) {
        for distribution in self.distribution_rules().get().iter() {
            let payment_amount = total_amount.clone() * distribution.percentage / MAX_PERCENTAGE;
            self.send()
                .contract_call::<IgnoreValue>(distribution.address, distribution.endpoint)
                .with_egld_transfer(payment_amount)
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
            index_total == MAX_PERCENTAGE,
            "Distribution percent total must be 100%"
        );
    }

    #[view(getMarketplaces)]
    #[storage_mapper("marketplaces")]
    fn marketplaces(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getNftCount)]
    #[storage_mapper("nftCount")]
    fn nft_count(&self) -> SingleValueMapper<u64>;

    #[view(getDistributionRules)]
    #[storage_mapper("distributionRules")]
    fn distribution_rules(&self) -> SingleValueMapper<ManagedVec<Distribution<Self::Api>>>;

    #[proxy]
    fn marketplace_proxy(
        &self,
        sc_address: ManagedAddress,
    ) -> nft_marketplace_proxy::Proxy<Self::Api>;
}

mod nft_marketplace_proxy {
    elrond_wasm::imports!();

    #[elrond_wasm::proxy]
    pub trait NftMarketplace {
        #[endpoint(claimTokens)]
        fn claim_tokens(
            &self,
            claim_destination: &ManagedAddress,
            token_id: &EgldOrEsdtTokenIdentifier,
            token_nonce: u64,
        ) -> MultiValue2<BigUint, IgnoreValue>;
    }
}
