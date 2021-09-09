use crate::function_selector::FunctionSelector;

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq, Clone)]
pub struct CurveArguments<M: ManagedTypeApi> {
    pub available_supply: BigUint<M>,
    pub balance: BigUint<M>,
}

impl<M: ManagedTypeApi> CurveArguments<M> {
    pub fn first_token_available(&self) -> BigUint<M> {
        &self.available_supply - &self.balance
    }
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq, Clone)]
pub struct BondingCurve<M: ManagedTypeApi> {
    pub curve: FunctionSelector<M>,
    pub arguments: CurveArguments<M>,
    pub sell_availability: bool,
    pub payment_token: TokenIdentifier<M>,
    pub payment_amount: BigUint<M>,
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq, Clone)]
pub struct TokenOwnershipData<M: ManagedTypeApi> {
    pub token_nonces: Vec<u64>,
    pub owner: ManagedAddress<M>,
}

impl<M: ManagedTypeApi> TokenOwnershipData<M> {
    pub fn add_nonce(&mut self, nonce: u64) {
        if !self.token_nonces.contains(&nonce) {
            self.token_nonces.push(nonce);
        }
    }
    pub fn remove_nonce(&mut self, nonce: u64) -> SCResult<()> {
        let index = self
            .token_nonces
            .iter()
            .position(|n| *n == nonce)
            .ok_or("Nonce requested is not available")?;
        self.token_nonces.remove(index);
        Ok(())
    }
}
