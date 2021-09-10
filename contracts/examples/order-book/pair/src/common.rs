elrond_wasm::imports!();
elrond_wasm::derive_imports!();

pub const MAX_ORDERS_PER_USER: usize = 100;
pub const PERCENT_BASE_POINTS: u64 = 100_000;
pub const FEE_PENALTY_INCREASE_EPOCHS: u64 = 5;
pub const FEE_PENALTY_INCREASE_PERCENT: u64 = 1_000;
pub const FREE_ORDER_FROM_STORAGE_MIN_PENALTIES: u64 = 6;

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, TypeAbi, Clone)]
pub enum OrderType {
    Buy,
    Sell,
}

#[derive(Clone)]
pub struct Payment<M: ManagedTypeApi> {
    pub token_id: TokenIdentifier<M>,
    pub amount: BigUint<M>,
}

#[derive(Clone)]
pub struct Transfer<M: ManagedTypeApi> {
    pub to: ManagedAddress<M>,
    pub payment: Payment<M>,
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone)]
pub enum FeeConfig<M: ManagedTypeApi> {
    Fixed(BigUint<M>),
    Percent(u64),
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone, Default)]
pub struct DealConfig {
    pub match_provider_percent: u64,
}

#[derive(TopEncode, TopDecode, TypeAbi, Clone)]
pub struct OrderInputParams<M: ManagedTypeApi> {
    pub amount: BigUint<M>,
    pub match_provider: Option<ManagedAddress<M>>,
    pub fee_config: FeeConfig<M>,
    pub deal_config: DealConfig,
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone)]
pub struct Order<M: ManagedTypeApi> {
    pub id: u64,
    pub creator: ManagedAddress<M>,
    pub match_provider: Option<ManagedAddress<M>>,
    pub input_amount: BigUint<M>,
    pub output_amount: BigUint<M>,
    pub fee_config: FeeConfig<M>,
    pub deal_config: DealConfig,
    pub create_epoch: u64,
    pub order_type: OrderType,
}

impl<M: ManagedTypeApi> FeeConfig<M> {
    pub fn is_fixed(&self) -> bool {
        matches!(*self, FeeConfig::Fixed(_))
    }

    pub fn is_percent(&self) -> bool {
        matches!(*self, FeeConfig::Percent(_))
    }
}

impl DealConfig {
    pub fn new() -> Self {
        Default::default()
    }
}

#[elrond_wasm::module]
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
        match fee_config.clone() {
            FeeConfig::Fixed(fee_amount) => fee_amount,
            FeeConfig::Percent(fee_percent) => amount * fee_percent / PERCENT_BASE_POINTS,
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
