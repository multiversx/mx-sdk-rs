use multiversx_sc::{codec::multi_types::MultiValueVec, types::Address};
use multiversx_sc_scenario::{
    api::StaticApi,
    scenario_model::{
        Account, AddressValue, CheckAccount, CheckStateStep, ScCallStep, ScDeployStep,
        SetStateStep, TxExpect,
    },
    ContractInfo, ScenarioWorld,
};
use transfer_role_features::ProxyTrait as _;

const ACCEPT_FUNDS_FUNC_NAME: &[u8] = b"accept_funds";
const OWNER_ADDRESS_EXPR: &str = "address:owner";
const REJECT_FUNDS_FUNC_NAME: &[u8] = b"reject_funds";
const TRANSFER_ROLE_FEATURES_ADDRESS_EXPR: &str = "sc:transfer-role-features";
const TRANSFER_ROLE_FEATURES_PATH_EXPR: &str = "file:output/transfer-role-features.wasm";
const TRANSFER_TOKEN_ID: &[u8] = b"TRANSFER-123456";
const TRANSFER_TOKEN_ID_EXPR: &str = "str:TRANSFER-123456";
const USER_ADDRESS_EXPR: &str = "address:user";
const VAULT_ADDRESS_EXPR: &str = "sc:vault";
const VAULT_PATH_EXPR: &str = "file:../vault/output/vault.wasm";

type TransferRoleFeaturesContract = ContractInfo<transfer_role_features::Proxy<StaticApi>>;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace(
        "contracts/feature-tests/composability/transfer-role-features",
    );

    blockchain.register_contract(
        TRANSFER_ROLE_FEATURES_PATH_EXPR,
        transfer_role_features::ContractBuilder,
    );
    blockchain.register_contract(VAULT_PATH_EXPR, vault::ContractBuilder);
    blockchain
}

struct TransferRoleTestState {
    world: ScenarioWorld,
    owner_address: Address,
    vault_address: Address,
    transfer_role_features_contract: TransferRoleFeaturesContract,
}

impl TransferRoleTestState {
    fn new() -> Self {
        let mut world = world();
        let vault_code = world.code_expression(VAULT_PATH_EXPR);

        world.set_state_step(
            SetStateStep::new()
                .put_account(OWNER_ADDRESS_EXPR, Account::new().nonce(1))
                .new_address(OWNER_ADDRESS_EXPR, 1, TRANSFER_ROLE_FEATURES_ADDRESS_EXPR)
                .put_account(
                    VAULT_ADDRESS_EXPR,
                    Account::new().nonce(1).code(vault_code.clone()),
                )
                .put_account(
                    USER_ADDRESS_EXPR,
                    Account::new()
                        .nonce(1)
                        .esdt_balance(TRANSFER_TOKEN_ID_EXPR, 1_000u64),
                ),
        );

        let owner_address = AddressValue::from(OWNER_ADDRESS_EXPR).to_address();
        let vault_address = AddressValue::from(VAULT_ADDRESS_EXPR).to_address();
        let transfer_role_features_contract =
            TransferRoleFeaturesContract::new(TRANSFER_ROLE_FEATURES_ADDRESS_EXPR);

        Self {
            world,
            owner_address,
            vault_address,
            transfer_role_features_contract,
        }
    }

    fn deploy(&mut self) -> &mut Self {
        let transfer_role_features_code =
            self.world.code_expression(TRANSFER_ROLE_FEATURES_PATH_EXPR);

        let whitelist = MultiValueVec::from(vec![
            AddressValue::from(OWNER_ADDRESS_EXPR).to_address(),
            AddressValue::from(VAULT_ADDRESS_EXPR).to_address(),
        ]);

        self.world.sc_deploy(
            ScDeployStep::new()
                .from(OWNER_ADDRESS_EXPR)
                .code(transfer_role_features_code)
                .call(self.transfer_role_features_contract.init(whitelist)),
        );

        self
    }

    fn forward_payments(&mut self, dest: Address, endpoint_name: &[u8]) {
        self.world.sc_call(
            ScCallStep::new()
                .from(USER_ADDRESS_EXPR)
                .esdt_transfer(TRANSFER_TOKEN_ID, 0, 100u64)
                .call(self.transfer_role_features_contract.forward_payments(
                    dest,
                    endpoint_name,
                    MultiValueVec::<Vec<u8>>::new(),
                )),
        );
    }

    fn check_user_and_vault_balance(&mut self) {
        self.world
            .check_state_step(CheckStateStep::new().put_account(
                USER_ADDRESS_EXPR,
                CheckAccount::new().esdt_balance(TRANSFER_TOKEN_ID_EXPR, "800"),
            ));
        self.world
            .check_state_step(CheckStateStep::new().put_account(
                VAULT_ADDRESS_EXPR,
                CheckAccount::new().esdt_balance(TRANSFER_TOKEN_ID_EXPR, "100"),
            ));
    }
}

#[test]
fn test_transfer_role() {
    let mut state = TransferRoleTestState::new();
    state.deploy();

    // transfer to user - ok
    state.forward_payments(state.owner_address.clone(), b"");
    state
        .world
        .check_state_step(CheckStateStep::new().put_account(
            USER_ADDRESS_EXPR,
            CheckAccount::new().esdt_balance(TRANSFER_TOKEN_ID_EXPR, "900"),
        ));
    state
        .world
        .check_state_step(CheckStateStep::new().put_account(
            OWNER_ADDRESS_EXPR,
            CheckAccount::new().esdt_balance(TRANSFER_TOKEN_ID_EXPR, "100"),
        ));

    // transfer to user - err, not whitelisted
    state.world.sc_call(
        ScCallStep::new()
            .from(USER_ADDRESS_EXPR)
            .esdt_transfer(TRANSFER_TOKEN_ID, 0, 100u64)
            .call(state.transfer_role_features_contract.forward_payments(
                Address::zero(),
                "",
                MultiValueVec::<Vec<u8>>::new(),
            ))
            .expect(TxExpect::user_error(
                "str:Destination address not whitelisted",
            )),
    );

    // transfer to sc - ok
    state.forward_payments(state.vault_address.clone(), ACCEPT_FUNDS_FUNC_NAME);
    state.check_user_and_vault_balance();

    // transfer to sc - reject
    state.forward_payments(state.vault_address.clone(), REJECT_FUNDS_FUNC_NAME);
    state.check_user_and_vault_balance();
}
