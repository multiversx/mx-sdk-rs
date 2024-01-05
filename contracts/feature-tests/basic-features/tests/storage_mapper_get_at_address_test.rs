use basic_features::{empty_storage::ProxyTrait as _, ProxyTrait as _};
use multiversx_sc::types::{ManagedAddress, MultiValueEncoded};
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

    fn contains_at_address(
        &mut self,
        item: u32,
        other_contract_address: ManagedAddress<StaticApi>,
        expected: bool,
    ) -> &mut Self {
        let contains: bool = self.world.sc_call_get_result(
            ScCallStep::new().from(OWNER_ADDRESS_EXPR).call(
                self.basic_features_contract
                    .contains_at_address_endpoint(&item, other_contract_address),
            ),
        );
        assert_eq!(contains, expected);
        self
    }

    fn next_at_address(
        &mut self,
        item: u32,
        other_contract_address: ManagedAddress<StaticApi>,
        expected: u32,
    ) -> &mut Self {
        let next: u32 = self.world.sc_call_get_result(
            ScCallStep::new().from(OWNER_ADDRESS_EXPR).call(
                self.basic_features_contract
                    .next_at_address_endpoint(&item, other_contract_address),
            ),
        );
        assert_eq!(next, expected);
        self
    }

    fn previous_at_address(
        &mut self,
        item: u32,
        other_contract_address: ManagedAddress<StaticApi>,
        expected: u32,
    ) -> &mut Self {
        let previous: u32 = self.world.sc_call_get_result(
            ScCallStep::new().from(OWNER_ADDRESS_EXPR).call(
                self.basic_features_contract
                    .previous_at_address_endpoint(&item, other_contract_address),
            ),
        );
        assert_eq!(previous, expected);
        self
    }

    fn is_empty_at_address(
        &mut self,
        other_contract_address: ManagedAddress<StaticApi>,
        expected: bool,
    ) -> &mut Self {
        let is_empty: bool = self.world.sc_call_get_result(
            ScCallStep::new().from(OWNER_ADDRESS_EXPR).call(
                self.basic_features_contract
                    .is_empty_at_address_endpoint(other_contract_address),
            ),
        );
        assert_eq!(is_empty, expected);
        self
    }

    fn front_at_address(
        &mut self,
        other_contract_address: ManagedAddress<StaticApi>,
        expected: u32,
    ) -> &mut Self {
        let front: u32 = self.world.sc_call_get_result(
            ScCallStep::new().from(OWNER_ADDRESS_EXPR).call(
                self.basic_features_contract
                    .front_at_address_endpoint(other_contract_address),
            ),
        );
        assert_eq!(front, expected);
        self
    }

    fn back_at_address(
        &mut self,
        other_contract_address: ManagedAddress<StaticApi>,
        expected: u32,
    ) -> &mut Self {
        let back: u32 = self.world.sc_call_get_result(
            ScCallStep::new().from(OWNER_ADDRESS_EXPR).call(
                self.basic_features_contract
                    .back_at_address_endpoint(other_contract_address),
            ),
        );
        assert_eq!(back, expected);
        self
    }
    fn len_at_address(
        &mut self,
        other_contract_address: ManagedAddress<StaticApi>,
        expected: usize,
    ) -> &mut Self {
        let len: usize = self.world.sc_call_get_result(
            ScCallStep::new().from(OWNER_ADDRESS_EXPR).call(
                self.basic_features_contract
                    .len_at_address_endpoint(other_contract_address),
            ),
        );
        assert_eq!(len, expected);
        self
    }
    fn check_internal_consistency_at_address(
        &mut self,
        other_contract_address: ManagedAddress<StaticApi>,
        expected: bool,
    ) -> &mut Self {
        let consistency: bool = self.world.sc_call_get_result(
            ScCallStep::new().from(OWNER_ADDRESS_EXPR).call(
                self.basic_features_contract
                    .check_internal_consistency_at_address_endpoint(other_contract_address),
            ),
        );
        assert_eq!(consistency, expected);
        self
    }
}

#[test]
fn storage_mapper_get_at_address_test() {
    let mut state = StorageMapperGetState::new();

    //deploy
    state.deploy_basic_features_contract();
    state.deploy_value_storage_contract();

    //is empty
    state.is_empty_at_address(state.value_storage_contract.to_address().into(), true);

    //check internal consistency
    state.check_internal_consistency_at_address(
        state.value_storage_contract.to_address().into(),
        true,
    );

    //contains at address
    state.contains_at_address(
        11u32,
        state.value_storage_contract.to_address().into(),
        false,
    );

    //len at address 
    state.len_at_address(state.value_storage_contract.to_address().into(), 0usize);

    //fill
    // state.fill_set_mapper();

    //check
    // let mut check = MultiValueEncoded::new();
    // for item in 1u32..=10u32 {
    //     check.push(item)
    // }
    // state.check_storage_vec(check);
}
