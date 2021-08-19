elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, TypeAbi)]
pub struct TokenAttributesStruct<M: ManagedTypeApi> {
    field_biguint: BigUint<M>,
    field_u64: u64,
    field_vec_u32: Vec<u32>,
}

#[elrond_wasm::module]
pub trait TokenAttributesMapperFeatures {
    #[storage_mapper("TokenAttributes")]
    fn token_attributes(&self) -> TokenAttributesMapper<Self::Storage>;

    #[endpoint]
    fn token_attributes_set(
        &self,
        token_id: &TokenIdentifier,
        token_nonce: u64,
        attributes: &TokenAttributesStruct<Self::TypeManager>,
    ) {
        self.token_attributes()
            .set(token_id, token_nonce, attributes)
    }

    #[endpoint]
    fn token_attributes_update(
        &self,
        token_id: &TokenIdentifier,
        token_nonce: u64,
        attributes: &TokenAttributesStruct<Self::TypeManager>,
    ) {
        self.token_attributes()
            .update(token_id, token_nonce, attributes)
    }

    #[endpoint]
    fn token_attributes_get(
        &self,
        token_id: &TokenIdentifier,
        token_nonce: u64,
    ) -> TokenAttributesStruct<Self::TypeManager> {
        self.token_attributes().get(token_id, token_nonce)
    }

    #[endpoint]
    fn token_attributes_clear(&self, token_id: &TokenIdentifier, token_nonce: u64) {
        self.token_attributes().clear(token_id, token_nonce)
    }

    #[endpoint]
    fn token_attributes_is_empty(&self, token_id: &TokenIdentifier, token_nonce: u64) -> bool {
        self.token_attributes().is_empty(token_id, token_nonce)
    }
}
