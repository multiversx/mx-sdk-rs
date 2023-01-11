#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

mod distribution_module;
mod nft_module;

use distribution_module::Distribution;
use multiversx_sc_modules::default_issue_callbacks;

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct ExampleAttributes {
    pub creation_timestamp: u64,
}

#[multiversx_sc::contract]
pub trait SeedNftMinter:
    distribution_module::DistributionModule
    + nft_module::NftModule
    + default_issue_callbacks::DefaultIssueCallbacksModule
{
    #[init]
    fn init(
        &self,
        marketplaces: ManagedVec<ManagedAddress>,
        distribution: ManagedVec<Distribution<Self::Api>>,
    ) {
        self.marketplaces().extend(&marketplaces);
        self.init_distribution(distribution);
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
    fn claim_and_distribute(&self, token_id: EgldOrEsdtTokenIdentifier, token_nonce: u64) {
        let total_amount = self.claim_royalties(&token_id, token_nonce);
        self.distribute_funds(&token_id, token_nonce, total_amount);
    }

    fn claim_royalties(&self, token_id: &EgldOrEsdtTokenIdentifier, token_nonce: u64) -> BigUint {
        let claim_destination = self.blockchain().get_sc_address();
        let mut total_amount = BigUint::zero();
        for address in self.marketplaces().iter() {
            let results: MultiValue2<BigUint, ManagedVec<EsdtTokenPayment>> = self
                .marketplace_proxy(address)
                .claim_tokens(&claim_destination, token_id, token_nonce)
                .execute_on_dest_context();

            let (egld_amount, esdt_payments) = results.into_tuple();
            let amount = if token_id.is_egld() {
                egld_amount
            } else {
                esdt_payments
                    .try_get(0)
                    .map(|esdt_payment| esdt_payment.amount)
                    .unwrap_or_default()
            };
            total_amount += amount;
        }

        total_amount
    }

    #[view(getMarketplaces)]
    #[storage_mapper("marketplaces")]
    fn marketplaces(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getNftCount)]
    #[storage_mapper("nftCount")]
    fn nft_count(&self) -> SingleValueMapper<u64>;

    #[proxy]
    fn marketplace_proxy(
        &self,
        sc_address: ManagedAddress,
    ) -> nft_marketplace_proxy::Proxy<Self::Api>;
}

mod nft_marketplace_proxy {
    multiversx_sc::imports!();

    #[multiversx_sc::proxy]
    pub trait NftMarketplace {
        #[endpoint(claimTokens)]
        fn claim_tokens(
            &self,
            claim_destination: &ManagedAddress,
            token_id: &EgldOrEsdtTokenIdentifier,
            token_nonce: u64,
        ) -> MultiValue2<BigUint, ManagedVec<EsdtTokenPayment>>;
    }
}
