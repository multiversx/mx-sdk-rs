#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[type_abi]
#[derive(
    ManagedVecItem, TopEncode, TopDecode, NestedEncode, NestedDecode, Clone, PartialEq, Debug,
)]
pub struct TokenAttributes<M: ManagedTypeApi> {
    pub amount: BigUint<M>,
}

impl<M: ManagedTypeApi> FixedSupplyToken<M> for TokenAttributes<M> {
    fn get_total_supply(&self) -> BigUint<M> {
        self.amount.clone()
    }

    fn into_part(self, payment_amount: &BigUint<M>) -> Self {
        let new_amount = self.rule_of_three_non_zero_result(payment_amount, &self.amount);
        TokenAttributes { amount: new_amount }
    }
}
impl<M: ManagedTypeApi> Mergeable<M> for TokenAttributes<M> {
    #[inline]
    fn can_merge_with(&self, _other: &Self) -> bool {
        true
    }

    fn merge_with(&mut self, other: Self) {
        self.error_if_not_mergeable(&other);

        self.amount += other.amount
    }
}

#[multiversx_sc::contract]
pub trait ExchangeFeatures {
    #[storage_mapper("supply")]
    fn supply(&self) -> SingleValueMapper<TokenAttributes<Self::Api>>;

    #[init]
    fn init(&self, initial_value: BigUint) {
        self.supply().set(TokenAttributes {
            amount: initial_value,
        });
    }

    #[upgrade]
    fn upgrade(&self, value: BigUint) {
        let token = self.supply().get();
        self.supply().set(token.into_part(&value));
    }

    #[endpoint]
    fn merge(&self, value: BigUint) {
        self.supply()
            .update(|token| token.merge_with(TokenAttributes { amount: value }));
    }

    #[endpoint]
    fn get_supply(&self) -> BigUint {
        self.supply().get().get_total_supply()
    }
}
