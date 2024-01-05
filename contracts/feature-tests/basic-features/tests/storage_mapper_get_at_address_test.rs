use basic_features::ProxyTrait as _;
use multiversx_sc::types::MultiValueEncoded;
use multiversx_sc_scenario::{
    api::StaticApi,
    scenario_model::{Account, ScCallStep, ScDeployStep, ScQueryStep, SetStateStep},
    ContractInfo, ScenarioWorld,
};
use value_storage::ProxyTrait as _;

const VALUE_STORAGE_PATH: &str = "file:value_storage/output/value_storage.wasm";
const BASIC_FEATURES_PATH: &str = "file:basic-features/output/basic-features.wasm";
const OWNER_ADDRESS_EXPR: &str = "address:owner";
const BASIC_FEATURES_ADDRESS_EXPR: &str = "sc:basic_features";
const VALUE_STORAGE_ADDRESS_EXPR: &str = "sc:value_storage";

type BasicFeatures = ContractInfo<basic_features::Proxy<StaticApi>>;
type ValueStorage = ContractInfo<value_storage::Proxy<StaticApi>>;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/feature-tests");
    blockchain.register_contract(BASIC_FEATURES_PATH, basic_features::ContractBuilder);
    blockchain.register_contract(VALUE_STORAGE_PATH, value_storage::ContractBuilder);

    blockchain
}

struct StorageMapperGetState {
    world: ScenarioWorld,
    basic_features_contract: BasicFeatures,
    value_storage_contract: ValueStorage,
}

impl StorageMapperGetState {
    fn new() -> Self {
        let mut world = world();
        world.set_state_step(
            SetStateStep::new()
                .put_account(OWNER_ADDRESS_EXPR, Account::new().nonce(1))
                .new_address(OWNER_ADDRESS_EXPR, 1, VALUE_STORAGE_ADDRESS_EXPR)
                .new_address(OWNER_ADDRESS_EXPR, 2, BASIC_FEATURES_ADDRESS_EXPR),
        );

        let basic_features_contract = BasicFeatures::new(BASIC_FEATURES_ADDRESS_EXPR);
        let value_storage_contract = ValueStorage::new(VALUE_STORAGE_ADDRESS_EXPR);

        Self {
            world,
            basic_features_contract,
            value_storage_contract,
        }
    }

    fn deploy_basic_features_contract(&mut self) -> &mut Self {
        let basic_features_code = self.world.code_expression(BASIC_FEATURES_PATH);
        self.world.sc_deploy(
            ScDeployStep::new()
                .from(OWNER_ADDRESS_EXPR)
                .code(basic_features_code)
                .call(self.basic_features_contract.init()),
        );

        self
    }

    fn deploy_value_storage_contract(&mut self) -> &mut Self {
        let value_storage_code = self.world.code_expression(BASIC_FEATURES_PATH);
        self.world.sc_deploy(
            ScDeployStep::new()
                .from(OWNER_ADDRESS_EXPR)
                .code(value_storage_code)
                .call(self.value_storage_contract.init()),
        );

        self
    }

    fn fill_set_mapper(&mut self) -> &mut Self {
        let mut storage = MultiValueEncoded::new();
        for item in 1u32..=10u32 {
            storage.push(item)
        }
        self.world.sc_call(
            ScCallStep::new()
                .from(OWNER_ADDRESS_EXPR)
                .call(self.value_storage_contract.fill_set_mapper(storage)),
        );

        self
    }

    fn check_storage_vec(&mut self, expected: MultiValueEncoded<StaticApi, u32>) -> &mut Self {
        self.world.sc_query(
            ScQueryStep::new()
                .call(self.value_storage_contract.value_set_mapper())
                .expect_value(expected),
        );

        self
    }
}

#[test]
fn storage_mapper_get_at_address_test() {
    let mut state = StorageMapperGetState::new();

    //deploy
    state.deploy_basic_features_contract();
    state.deploy_value_storage_contract();

    //fill
    state.fill_set_mapper();

    //check
    let mut check = MultiValueEncoded::new();
    for item in 1u32..=10u32 {
        check.push(item)
    }
    state.check_storage_vec(check);
}
