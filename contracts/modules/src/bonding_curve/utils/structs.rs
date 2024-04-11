use crate::bonding_curve::curves::curve_function::CurveFunction;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq, Eq, Clone)]
pub struct CurveArguments<'a, M: ManagedTypeApi<'a>> {
    pub available_supply: BigUint<'a, M>,
    pub balance: BigUint<'a, M>,
}

impl<'a, M: ManagedTypeApi<'a>> CurveArguments<'a, M> {
    pub fn first_token_available(&self) -> BigUint<'a, M> {
        &self.available_supply - &self.balance
    }
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq, Eq, Clone)]
pub struct BondingCurve<
    M: ManagedTypeApi<'a>,
    T: CurveFunction<'a, M> + TopEncode + TopDecode + NestedEncode + NestedDecode + TypeAbi,
> {
    pub curve: T,
    pub arguments: CurveArguments<'a, M>,
    pub sell_availability: bool,
    pub payment: EgldOrEsdtTokenPayment<'a, M>,
}

impl<
        M: ManagedTypeApi<'a>,
        T: CurveFunction<'a, M> + TopEncode + TopDecode + NestedEncode + NestedDecode + TypeAbi,
    > BondingCurve<'a, M, T>
{
    pub fn payment_token(&self) -> EgldOrEsdtTokenIdentifier<'a, M> {
        self.payment.token_identifier.clone()
    }
    pub fn payment_is_egld(&self) -> bool {
        self.payment.token_identifier.is_egld()
    }
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq, Eq, Clone)]
pub struct TokenOwnershipData<'a, M: ManagedTypeApi<'a>> {
    pub token_nonces: ManagedVec<'a, M, u64>,
    pub owner: ManagedAddress<'a, M>,
}

impl<'a, M: ManagedTypeApi<'a>> TokenOwnershipData<'a, M> {
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
