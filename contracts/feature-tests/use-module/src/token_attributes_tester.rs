elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, TypeAbi)]
pub struct StructWithoutBigUint {
    internal_field_u64: u64,
    internal_field_u8: u8,
    internal_field_token_id: TokenIdentifier,
}

#[derive(TopEncode, TopDecode, TypeAbi)]
pub struct StructWithBigUint<BigUint: BigUintApi> {
    internal_field_biguint1: BigUint,
    internal_field_biguint2: BigUint,
}

#[elrond_wasm::module]
pub trait TokenAttributesTester:
    elrond_wasm_module_token_attributes::TokenAttributesModule
{
    #[endpoint(setTokenAttributesWithBigUint)]
    fn set_token_attributes_with_biguint(
        &self,
        token_id: TokenIdentifier,
        token_nonce: u64,
        attributes: StructWithBigUint<BigUint>,
    ) -> SCResult<()> {
        self.token_attributes_set(&token_id, token_nonce, &attributes)
    }

    #[endpoint(setTokenAttributesWithoutBigUint)]
    fn set_token_attributes_without_biguint(
        &self,
        token_id: TokenIdentifier,
        token_nonce: u64,
        attributes: StructWithoutBigUint,
    ) -> SCResult<()> {
        self.token_attributes_set(&token_id, token_nonce, &attributes)
    }

    #[endpoint(updateTokenAttributesWithBigUint)]
    fn update_token_attributes_with_biguint(
        &self,
        token_id: TokenIdentifier,
        token_nonce: u64,
        attributes: StructWithBigUint<BigUint>,
    ) -> SCResult<()> {
        self.token_attributes_update(&token_id, token_nonce, &attributes)
    }

    #[endpoint(updateTokenAttributesWithoutBigUint)]
    fn update_token_attributes_without_biguint(
        &self,
        token_id: TokenIdentifier,
        token_nonce: u64,
        attributes: StructWithoutBigUint,
    ) -> SCResult<()> {
        self.token_attributes_update(&token_id, token_nonce, &attributes)
    }

    #[endpoint(clearTokenAttributes)]
    fn clear_token_attributes(&self, token_id: TokenIdentifier, token_nonce: u64) -> SCResult<()> {
        self.token_attributes_clear(&token_id, token_nonce)
    }

    #[endpoint(isEmptyTokenAttributes)]
    fn is_empty_token_attributes(&self, token_id: TokenIdentifier, token_nonce: u64) -> bool {
        self.token_attributes_is_empty(&token_id, token_nonce)
    }

    #[endpoint(getTokenAttributesWithBigUint)]
    fn get_token_attributes_with_biguint(
        &self,
        token_id: TokenIdentifier,
        token_nonce: u64,
    ) -> SCResult<StructWithBigUint<BigUint>> {
        self.token_attributes_get::<StructWithBigUint<BigUint>>(&token_id, token_nonce)
    }

    #[endpoint(getTokenAttributesWithoutBigUint)]
    fn get_token_attributes_without_biguint(
        &self,
        token_id: TokenIdentifier,
        token_nonce: u64,
    ) -> SCResult<StructWithoutBigUint> {
        self.token_attributes_get::<StructWithoutBigUint>(&token_id, token_nonce)
    }
}
