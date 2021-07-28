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
pub struct Payment<BigUint: BigUintApi> {
    pub token_id: TokenIdentifier,
    pub amount: BigUint,
}

#[derive(Clone)]
pub struct Transfer<BigUint: BigUintApi> {
    pub to: Address,
    pub payment: Payment<BigUint>,
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone, Copy)]
pub enum FeeConfig<BigUint: BigUintApi> {
    Fixed(BigUint),
    Percent(u64),
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone, Copy)]
pub struct DealConfig {
    pub match_provider_percent: u64,
}

#[derive(TopEncode, TopDecode, TypeAbi, Clone)]
pub struct OrderInputParams<BigUint: BigUintApi> {
    pub amount: BigUint,
    pub match_provider: Option<Address>,
    pub fee_config: FeeConfig<BigUint>,
    pub deal_config: DealConfig,
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone)]
pub struct Order<BigUint: BigUintApi> {
    pub id: u64,
    pub creator: Address,
    pub match_provider: Option<Address>,
    pub input_amount: BigUint,
    pub output_amount: BigUint,
    pub fee_config: FeeConfig<BigUint>,
    pub deal_config: DealConfig,
    pub create_epoch: u64,
    pub order_type: OrderType,
}

impl<BigUint: BigUintApi> FeeConfig<BigUint> {
    pub fn is_fixed(&self) -> bool {
        matches!(*self, FeeConfig::Fixed(_))
    }

    pub fn is_percent(&self) -> bool {
        matches!(*self, FeeConfig::Percent(_))
    }
}

impl<BigUint: BigUintApi> Payment<BigUint> {
    pub fn new() -> Self {
        Default::default()
    }
}

impl<BigUint: BigUintApi> Default for Payment<BigUint> {
    fn default() -> Self {
        Payment {
            token_id: TokenIdentifier::egld(),
            amount: BigUint::zero(),
        }
    }
}

impl DealConfig {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Default for DealConfig {
    fn default() -> Self {
        DealConfig {
            match_provider_percent: 0,
        }
    }
}

impl<BigUint: BigUintApi> Order<BigUint> {
    pub fn new() -> Self {
        Default::default()
    }
}

impl<BigUint: BigUintApi> Default for Order<BigUint> {
    fn default() -> Self {
        Order {
            id: 0,
            creator: Address::zero(),
            match_provider: Option::None,
            input_amount: BigUint::zero(),
            output_amount: BigUint::zero(),
            fee_config: FeeConfig::Percent(0),
            deal_config: DealConfig::new(),
            create_epoch: 0,
            order_type: OrderType::Buy,
        }
    }
}

#[elrond_wasm::module]
pub trait CommonModule {
    fn new_order(
        &self,
        id: u64,
        payment: Payment<Self::BigUint>,
        params: OrderInputParams<Self::BigUint>,
        order_type: OrderType,
    ) -> Order<Self::BigUint> {
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

    fn rule_of_three(
        &self,
        part: &Self::BigUint,
        total: &Self::BigUint,
        value: &Self::BigUint,
    ) -> Self::BigUint {
        &(part * value) / total
    }

    fn calculate_fee_amount(
        &self,
        amount: &Self::BigUint,
        fee_config: &FeeConfig<Self::BigUint>,
    ) -> Self::BigUint {
        match fee_config.clone() {
            FeeConfig::Fixed(fee_amount) => fee_amount,
            FeeConfig::Percent(fee_percent) => {
                amount * &fee_percent.into() / PERCENT_BASE_POINTS.into()
            },
        }
    }

    fn calculate_amount_after_fee(
        &self,
        amount: &Self::BigUint,
        fee_config: &FeeConfig<Self::BigUint>,
    ) -> Self::BigUint {
        amount - &self.calculate_fee_amount(amount, fee_config)
    }

    #[view(getFirstTokenId)]
    #[storage_mapper("first_token_id")]
    fn first_token_id(&self) -> SingleValueMapper<Self::Storage, TokenIdentifier>;

    #[view(getSecondTokenId)]
    #[storage_mapper("second_token_id")]
    fn second_token_id(&self) -> SingleValueMapper<Self::Storage, TokenIdentifier>;
}
