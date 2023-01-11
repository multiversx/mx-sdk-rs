multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi)]
pub struct TokenAttributesStruct<M: ManagedTypeApi> {
    field_biguint: BigUint<M>,
    field_u64: u64,
    field_vec_u32: ManagedVec<M, u32>,
}

#[multiversx_sc::module]
pub trait TokenAttributesMapperFeatures {
    #[storage_mapper("TokenAttributes")]
    fn token_attributes(&self) -> TokenAttributesMapper;

    #[endpoint]
    fn token_attributes_set(
        &self,
        token_id: &TokenIdentifier,
        token_nonce: u64,
        attributes: &TokenAttributesStruct<Self::Api>,
    ) {
        self.token_attributes()
            .set(token_id, token_nonce, attributes)
    }

    #[endpoint]
    fn token_attributes_update(
        &self,
        token_id: &TokenIdentifier,
        token_nonce: u64,
        attributes: &TokenAttributesStruct<Self::Api>,
    ) {
        self.token_attributes()
            .update(token_id, token_nonce, attributes)
    }

    #[endpoint]
    fn token_attributes_get_attributes(
        &self,
        token_id: &TokenIdentifier,
        token_nonce: u64,
    ) -> TokenAttributesStruct<Self::Api> {
        self.token_attributes()
            .get_attributes::<TokenAttributesStruct<Self::Api>, Self::Api>(token_id, token_nonce)
    }

    #[endpoint]
    fn token_attributes_get_nonce(
        &self,
        token_id: &TokenIdentifier,
        attributes: TokenAttributesStruct<Self::Api>,
    ) -> u64 {
        self.token_attributes()
            .get_nonce::<TokenAttributesStruct<Self::Api>, Self::Api>(token_id, &attributes)
    }

    #[endpoint]
    fn token_attributes_clear(&self, token_id: &TokenIdentifier, token_nonce: u64) {
        self.token_attributes()
            .clear::<TokenAttributesStruct<Self::Api>, Self::Api>(token_id, token_nonce)
    }

    #[endpoint]
    fn token_attributes_has_attributes(
        &self,
        token_id: &TokenIdentifier,
        token_nonce: u64,
    ) -> bool {
        self.token_attributes()
            .has_attributes(token_id, token_nonce)
    }
}
