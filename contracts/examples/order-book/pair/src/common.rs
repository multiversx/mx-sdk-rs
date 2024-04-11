use multiversx_sc::{derive_imports::*, imports::*};

pub const MAX_ORDERS_PER_USER: usize = 100;
pub const PERCENT_BASE_POINTS: u64 = 100_000;
pub const FEE_PENALTY_INCREASE_EPOCHS: u64 = 5;
pub const FEE_PENALTY_INCREASE_PERCENT: u64 = 1_000;
pub const FREE_ORDER_FROM_STORAGE_MIN_PENALTIES: u64 = 6;

#[derive(
    ManagedVecItem, TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Eq, TypeAbi, Clone,
)]
pub enum OrderType {
    Buy,
    Sell,
}

#[derive(ManagedVecItem, Clone)]
pub struct Payment<'a, M: ManagedTypeApi<'a>> {
    pub token_id: TokenIdentifier<'a, M>,
    pub amount: BigUint<'a, M>,
}

#[derive(ManagedVecItem, Clone)]
pub struct Transfer<'a, M: ManagedTypeApi<'a>> {
    pub to: ManagedAddress<'a, M>,
    pub payment: Payment<'a, M>,
}

#[derive(ManagedVecItem, TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone)]
pub enum FeeConfigEnum {
    Fixed,
    Percent,
}

#[derive(ManagedVecItem, TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone)]
pub struct FeeConfig<'a, M: ManagedTypeApi<'a>> {
    pub fee_type: FeeConfigEnum,
    pub fixed_fee: BigUint<'a, M>,
    pub percent_fee: u64,
}

#[derive(
    ManagedVecItem, TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone, Default,
)]
pub struct DealConfig {
    pub match_provider_percent: u64,
}

#[derive(TopEncode, TopDecode, TypeAbi, Clone)]
pub struct OrderInputParams<'a, M: ManagedTypeApi<'a>> {
    pub amount: BigUint<'a, M>,
    pub match_provider: ManagedAddress<'a, M>,
    pub fee_config: FeeConfig<'a, M>,
    pub deal_config: DealConfig,
}

#[derive(ManagedVecItem, TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone)]
pub struct Order<'a, M: ManagedTypeApi<'a>> {
    pub id: u64,
    pub creator: ManagedAddress<'a, M>,
    pub match_provider: ManagedAddress<'a, M>,
    pub input_amount: BigUint<'a, M>,
    pub output_amount: BigUint<'a, M>,
    pub fee_config: FeeConfig<'a, M>,
    pub deal_config: DealConfig,
    pub create_epoch: u64,
    pub order_type: OrderType,
}

impl DealConfig {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Default::default()
    }
}

#[multiversx_sc::module]
pub trait CommonModule {
    fn new_order(
        &self,
        id: u64,
        payment: Payment<Self::Api>,
        params: OrderInputParams<Self::Api>,
        order_type: OrderType,
    ) -> Order<Self::Api> {
        Order {
            id,
            creator: self.blockchain().get_caller(),
            match_provider: params.match_provider,
            input_amount: payment.amount,
            output_amount: params.amount,
            fee_config: params.fee_config,
            deal_config: params.deal_config,
            create_epoch: self.blockchain().get_block_epoch(),
            order_type,
        }
    }

    fn rule_of_three(&self, part: &BigUint, total: &BigUint, value: &BigUint) -> BigUint {
        &(part * value) / total
    }

    fn calculate_fee_amount(&self, amount: &BigUint, fee_config: &FeeConfig<Self::Api>) -> BigUint {
        match fee_config.fee_type {
            FeeConfigEnum::Fixed => fee_config.fixed_fee.clone(),
            FeeConfigEnum::Percent => amount * fee_config.percent_fee / PERCENT_BASE_POINTS,
        }
    }

    fn calculate_amount_after_fee(
        &self,
        amount: &BigUint,
        fee_config: &FeeConfig<Self::Api>,
    ) -> BigUint {
        amount - &self.calculate_fee_amount(amount, fee_config)
    }

    #[view(getFirstTokenId)]
    #[storage_mapper("first_token_id")]
    fn first_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getSecondTokenId)]
    #[storage_mapper("second_token_id")]
    fn second_token_id(&self) -> SingleValueMapper<TokenIdentifier>;
}
