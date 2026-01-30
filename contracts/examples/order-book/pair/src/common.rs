use multiversx_sc::{derive_imports::*, imports::*};

pub const MAX_ORDERS_PER_USER: usize = 100;
pub const PERCENT_BASE_POINTS: u64 = 100_000;
pub const FEE_PENALTY_INCREASE_EPOCHS: u64 = 5;
pub const FEE_PENALTY_INCREASE_PERCENT: u64 = 1_000;
pub const FREE_ORDER_FROM_STORAGE_MIN_PENALTIES: u64 = 6;

#[type_abi]
#[derive(
    ManagedVecItem, TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Eq, Clone, Copy,
)]
pub enum OrderType {
    Buy,
    Sell,
}

#[derive(ManagedVecItem, Clone)]
pub struct Transfer<M: ManagedTypeApi> {
    pub to: ManagedAddress<M>,
    pub payment: FungiblePayment<M>,
}

#[type_abi]
#[derive(ManagedVecItem, TopEncode, TopDecode, NestedEncode, NestedDecode, Clone)]
pub enum FeeConfigEnum {
    Fixed,
    Percent,
}

#[type_abi]
#[derive(ManagedVecItem, TopEncode, TopDecode, NestedEncode, NestedDecode, Clone)]
pub struct FeeConfig<M: ManagedTypeApi> {
    pub fee_type: FeeConfigEnum,
    pub fixed_fee: BigUint<M>,
    pub percent_fee: u64,
}

#[type_abi]
#[derive(ManagedVecItem, TopEncode, TopDecode, NestedEncode, NestedDecode, Clone, Default)]
pub struct DealConfig {
    pub match_provider_percent: u64,
}

#[type_abi]
#[derive(TopEncode, TopDecode, Clone)]
pub struct OrderInputParams<M: ManagedTypeApi> {
    pub amount: NonZeroBigUint<M>,
    pub match_provider: ManagedAddress<M>,
    pub fee_config: FeeConfig<M>,
    pub deal_config: DealConfig,
}

#[type_abi]
#[derive(ManagedVecItem, TopEncode, TopDecode, NestedEncode, NestedDecode, Clone)]
pub struct Order<M: ManagedTypeApi> {
    pub id: u64,
    pub creator: ManagedAddress<M>,
    pub match_provider: ManagedAddress<M>,
    pub input_amount: NonZeroBigUint<M>,
    pub output_amount: NonZeroBigUint<M>,
    pub fee_config: FeeConfig<M>,
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
        payment: FungiblePayment<Self::Api>,
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

    fn rule_of_three(&self, part: u64, total: u64, value: &BigUint) -> BigUint {
        &(BigUint::from(part) * value) / BigUint::from(total)
    }

    fn calculate_fee_amount(
        &self,
        amount: &NonZeroBigUint,
        fee_config: &FeeConfig<Self::Api>,
    ) -> BigUint {
        match fee_config.fee_type {
            FeeConfigEnum::Fixed => fee_config.fixed_fee.clone(),
            FeeConfigEnum::Percent => amount
                .as_big_uint()
                .proportion(fee_config.percent_fee, PERCENT_BASE_POINTS),
        }
    }

    fn calculate_amount_after_fee(
        &self,
        amount: &NonZeroBigUint,
        fee_config: &FeeConfig<Self::Api>,
    ) -> BigUint {
        amount.as_big_uint() - &self.calculate_fee_amount(amount, fee_config)
    }

    #[view(getFirstTokenId)]
    #[storage_mapper("first_token_id")]
    fn first_token_id(&self) -> SingleValueMapper<TokenId>;

    #[view(getSecondTokenId)]
    #[storage_mapper("second_token_id")]
    fn second_token_id(&self) -> SingleValueMapper<TokenId>;
}
