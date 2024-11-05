use multiversx_sc_scenario::imports::*;
use transfer_role_features::transfer_role_proxy;

const ACCEPT_FUNDS_FUNC_NAME: &[u8] = b"accept_funds";
const REJECT_FUNDS_FUNC_NAME: &[u8] = b"reject_funds";
const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
const TRANSFER_ROLE_FEATURES_ADDRESS: TestSCAddress = TestSCAddress::new("transfer-role-features");
const TRANSFER_ROLE_FEATURES_PATH: MxscPath =
    MxscPath::new("output/transfer-role-features.mxsc.json");
const TRANSFER_TOKEN: TestTokenIdentifier = TestTokenIdentifier::new("TRANSFER-123456");
const USER_ADDRESS: TestAddress = TestAddress::new("user");
const VAULT_ADDRESS: TestSCAddress = TestSCAddress::new("vault");
const VAULT_PATH: MxscPath = MxscPath::new("../vault/output/vault.mxsc.json");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.set_current_dir_from_workspace(
        "contracts/feature-tests/composability/transfer-role-features",
    );
    blockchain.register_contract(
        TRANSFER_ROLE_FEATURES_PATH,
        transfer_role_features::ContractBuilder,
    );
    blockchain.register_contract(VAULT_PATH, vault::ContractBuilder);
    blockchain
}

struct TransferRoleTestState {
    world: ScenarioWorld,
}

impl TransferRoleTestState {
    fn new() -> Self {
        let mut world = world();

        world.account(OWNER_ADDRESS).nonce(1).new_address(
            OWNER_ADDRESS,
            1,
            TRANSFER_ROLE_FEATURES_ADDRESS,
        );

        world.account(VAULT_ADDRESS).nonce(1).code(VAULT_PATH);
        world
            .account(USER_ADDRESS)
            .nonce(1)
            .esdt_balance(TRANSFER_TOKEN, 1000);

        Self { world }
    }

    fn deploy(&mut self) -> &mut Self {
        let whitelist = MultiValueVec::from(vec![
            AddressValue::from(OWNER_ADDRESS).to_address(),
            AddressValue::from(VAULT_ADDRESS).to_address(),
        ]);

        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .typed(transfer_role_proxy::TransferRoleFeaturesProxy)
            .init(whitelist)
            .code(TRANSFER_ROLE_FEATURES_PATH)
            .new_address(TRANSFER_ROLE_FEATURES_ADDRESS)
            .run();

        self
    }

    fn forward_payments(&mut self, dest: Address, endpoint_name: &[u8]) {
        self.world
            .tx()
            .from(USER_ADDRESS)
            .to(TRANSFER_ROLE_FEATURES_ADDRESS)
            .typed(transfer_role_proxy::TransferRoleFeaturesProxy)
            .forward_payments(dest, endpoint_name, MultiValueVec::<Vec<u8>>::new())
            .egld_or_single_esdt(
                &EgldOrEsdtTokenIdentifier::esdt(TRANSFER_TOKEN),
                0u64,
                &multiversx_sc::proxy_imports::BigUint::from(100u64),
            )
            .run();
    }

    fn check_user_and_vault_balance(&mut self) {
        self.world
            .check_account(USER_ADDRESS)
            .esdt_balance(TRANSFER_TOKEN, 800);
        self.world
            .check_account(VAULT_ADDRESS)
            .esdt_balance(TRANSFER_TOKEN, 100);
    }
}

#[test]
fn test_transfer_role() {
    let mut state = TransferRoleTestState::new();
    state.deploy();

    // transfer to user - ok
    state.forward_payments(Address::from(OWNER_ADDRESS.eval_to_array()), b"");
    state
        .world
        .check_account(USER_ADDRESS)
        .esdt_balance(TRANSFER_TOKEN, 900);

    state
        .world
        .check_account(OWNER_ADDRESS)
        .esdt_balance(TRANSFER_TOKEN, 100);

    // transfer to user - err, not whitelisted
    state
        .world
        .tx()
        .from(USER_ADDRESS)
        .to(TRANSFER_ROLE_FEATURES_ADDRESS)
        .typed(transfer_role_proxy::TransferRoleFeaturesProxy)
        .forward_payments(Address::zero(), "", MultiValueVec::<Vec<u8>>::new())
        .egld_or_single_esdt(
            &EgldOrEsdtTokenIdentifier::esdt(TRANSFER_TOKEN),
            0u64,
            &multiversx_sc::proxy_imports::BigUint::from(100u64),
        )
        .with_result(ExpectMessage("Destination address not whitelisted"))
        .run();

    // transfer to sc - ok
    state.forward_payments(VAULT_ADDRESS.to_address(), ACCEPT_FUNDS_FUNC_NAME);
    state.check_user_and_vault_balance();

    // transfer to sc - reject
    state.forward_payments(VAULT_ADDRESS.to_address(), REJECT_FUNDS_FUNC_NAME);
    state.check_user_and_vault_balance();
}
