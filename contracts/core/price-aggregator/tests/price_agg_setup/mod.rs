use multiversx_price_aggregator_sc::{staking::StakingModule, PriceAggregator};
use multiversx_sc::types::{Address, EgldOrEsdtTokenIdentifier, MultiValueEncoded};
use multiversx_sc_modules::pause::PauseModule;
use multiversx_sc_scenario::{
    managed_address, managed_biguint, managed_buffer, rust_biguint,
    testing_framework::{BlockchainStateWrapper, ContractObjWrapper, TxResult},
    DebugApi,
};

pub const NR_ORACLES: usize = 4;
pub const SUBMISSION_COUNT: usize = 3;
pub const DECIMALS: u8 = 0;
pub static EGLD_TICKER: &[u8] = b"EGLD";
pub static USD_TICKER: &[u8] = b"USDC";

pub const STAKE_AMOUNT: u64 = 20;
pub const SLASH_AMOUNT: u64 = 10;
pub const SLASH_QUORUM: usize = 2;

pub struct PriceAggSetup<PriceAggObjBuilder>
where
    PriceAggObjBuilder:
        'static + Copy + Fn() -> multiversx_price_aggregator_sc::ContractObj<DebugApi>,
{
    pub b_mock: BlockchainStateWrapper,
    pub owner: Address,
    pub oracles: Vec<Address>,
    pub price_agg: ContractObjWrapper<
        multiversx_price_aggregator_sc::ContractObj<DebugApi>,
        PriceAggObjBuilder,
    >,
}

impl<PriceAggObjBuilder> PriceAggSetup<PriceAggObjBuilder>
where
    PriceAggObjBuilder:
        'static + Copy + Fn() -> multiversx_price_aggregator_sc::ContractObj<DebugApi>,
{
    pub fn new(builder: PriceAggObjBuilder) -> Self {
        let rust_zero = rust_biguint!(0);
        let mut b_mock = BlockchainStateWrapper::new();
        let owner = b_mock.create_user_account(&rust_zero);

        let mut oracles = Vec::new();
        for _ in 0..NR_ORACLES {
            let oracle = b_mock.create_user_account(&rust_biguint!(STAKE_AMOUNT));
            oracles.push(oracle);
        }

        let price_agg =
            b_mock.create_sc_account(&rust_zero, Some(&owner), builder, "price_agg_path");

        let current_timestamp = 100;
        b_mock.set_block_timestamp(current_timestamp);

        // init price aggregator
        b_mock
            .execute_tx(&owner, &price_agg, &rust_zero, |sc| {
                let mut oracle_args = MultiValueEncoded::new();
                for oracle in &oracles {
                    oracle_args.push(managed_address!(oracle));
                }

                sc.init(
                    EgldOrEsdtTokenIdentifier::egld(),
                    managed_biguint!(STAKE_AMOUNT),
                    managed_biguint!(SLASH_AMOUNT),
                    SLASH_QUORUM,
                    SUBMISSION_COUNT,
                    oracle_args,
                );
            })
            .assert_ok();

        for oracle in &oracles {
            b_mock
                .execute_tx(oracle, &price_agg, &rust_biguint!(STAKE_AMOUNT), |sc| {
                    sc.stake();
                })
                .assert_ok();
        }

        Self {
            b_mock,
            oracles,
            owner,
            price_agg,
        }
    }

    pub fn set_pair_decimals(&mut self, from: &[u8], to: &[u8], decimals: u8) {
        self.b_mock
            .execute_tx(&self.owner, &self.price_agg, &rust_biguint!(0), |sc| {
                sc.set_pair_decimals(managed_buffer!(from), managed_buffer!(to), decimals);
            })
            .assert_ok();
    }

    pub fn unpause(&mut self) {
        self.b_mock
            .execute_tx(&self.owner, &self.price_agg, &rust_biguint!(0), |sc| {
                sc.unpause_endpoint();
            })
            .assert_ok();
    }

    pub fn submit(&mut self, oracle: &Address, timestamp: u64, price: u64) -> TxResult {
        self.b_mock
            .execute_tx(oracle, &self.price_agg, &rust_biguint!(0), |sc| {
                sc.submit(
                    managed_buffer!(EGLD_TICKER),
                    managed_buffer!(USD_TICKER),
                    timestamp,
                    managed_biguint!(price),
                    DECIMALS,
                );
            })
    }
}
