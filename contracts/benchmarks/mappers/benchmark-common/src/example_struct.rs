multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub type Nonce = u64;

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone)]
pub struct ExampleStruct<M>
where
    M: ManagedTypeApi,
{
    pub first_token_id: TokenIdentifier<M>,
    pub first_token_nonce: Nonce,
    pub first_token_amount: BigUint<M>,
    pub second_token_id: TokenIdentifier<M>,
    pub second_token_nonce: Nonce,
    pub second_token_amount: BigUint<M>,
}

impl<M> PartialEq for ExampleStruct<M>
where
    M: ManagedTypeApi,
{
    fn eq(&self, other: &Self) -> bool {
        self.first_token_id == other.first_token_id
            && self.first_token_nonce == other.first_token_nonce
            && self.first_token_amount == other.first_token_amount
            && self.second_token_id == other.second_token_id
            && self.second_token_nonce == other.second_token_nonce
            && self.second_token_amount == other.second_token_amount
    }
}
