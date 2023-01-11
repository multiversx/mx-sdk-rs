use crate::bonding_curve::curves::curve_function::CurveFunction;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq, Eq, Clone)]
pub struct CurveArguments<M: ManagedTypeApi> {
    pub available_supply: BigUint<M>,
    pub balance: BigUint<M>,
}

impl<M: ManagedTypeApi> CurveArguments<M> {
    pub fn first_token_available(&self) -> BigUint<M> {
        &self.available_supply - &self.balance
    }
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq, Eq, Clone)]
pub struct BondingCurve<
    M: ManagedTypeApi,
    T: CurveFunction<M> + TopEncode + TopDecode + NestedEncode + NestedDecode + TypeAbi,
> {
    pub curve: T,
    pub arguments: CurveArguments<M>,
    pub sell_availability: bool,
    pub payment: EgldOrEsdtTokenPayment<M>,
}

impl<
        M: ManagedTypeApi,
        T: CurveFunction<M> + TopEncode + TopDecode + NestedEncode + NestedDecode + TypeAbi,
    > BondingCurve<M, T>
{
    pub fn payment_token(&self) -> EgldOrEsdtTokenIdentifier<M> {
        self.payment.token_identifier.clone()
    }
    pub fn payment_is_egld(&self) -> bool {
        self.payment.token_identifier.is_egld()
    }
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq, Eq, Clone)]
pub struct TokenOwnershipData<M: ManagedTypeApi> {
    pub token_nonces: ManagedVec<M, u64>,
    pub owner: ManagedAddress<M>,
}

impl<M: ManagedTypeApi> TokenOwnershipData<M> {
    pub fn add_nonce(&mut self, nonce: u64) {
        if !self.token_nonces.contains(&nonce) {
            self.token_nonces.push(nonce);
        }
    }
    pub fn remove_nonce(&mut self, nonce: u64) {
        let index = self.token_nonces.iter().position(|n| n == nonce);

        match index {
            Some(value) => self.token_nonces.remove(value),
            None => M::error_api_impl().signal_error(b"Nonce requested is not available"),
        };
    }
}
