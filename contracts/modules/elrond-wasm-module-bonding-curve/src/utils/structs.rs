use crate::function_selector::FunctionSelector;

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq, Clone)]
pub struct CurveArguments<BigUint: BigUintApi> {
    pub available_supply: BigUint,
    pub balance: BigUint,
}

impl<BigUint> CurveArguments<BigUint>
where
    for<'a, 'b> &'a BigUint: core::ops::Sub<&'b BigUint, Output = BigUint>,
    for<'b> BigUint: core::ops::SubAssign<&'b BigUint>,
    BigUint: BigUintApi,
{
    pub fn first_token_available(&self) -> BigUint {
        &self.available_supply - &self.balance
    }
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq, Clone)]
pub struct BondingCurve<BigUint: BigUintApi> {
    pub curve: FunctionSelector<BigUint>,
    pub arguments: CurveArguments<BigUint>,
    pub sell_availability: bool,
    pub payment_token: TokenIdentifier,
    pub payment_amount: BigUint,
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq, Clone)]
pub struct TokenOwnershipData {
    pub token_nonces: Vec<u64>,
    pub owner: Address,
}

impl TokenOwnershipData {
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
