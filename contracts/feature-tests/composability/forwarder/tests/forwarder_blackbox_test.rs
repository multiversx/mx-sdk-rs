use forwarder::nft::{Color, ProxyTrait as _};

use multiversx_sc_scenario::{
    api::StaticApi,
    scenario_model::{
        Account, CheckAccount, CheckStateStep, ScCallStep, SetStateStep, TransferStep,
    },
    ContractInfo, ScenarioWorld,
};

const USER_ADDRESS_EXPR: &str = "address:user";
const FORWARDER_ADDRESS_EXPR: &str = "sc:forwarder";
const FORWARDER_PATH_EXPR: &str = "file:output/forwarder.wasm";

const NFT_TOKEN_ID_EXPR: &str = "str:COOL-123456";
const NFT_TOKEN_ID: &[u8] = b"COOL-123456";

type ForwarderContract = ContractInfo<forwarder::Proxy<StaticApi>>;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/composability/forwarder");

    blockchain.register_contract(FORWARDER_PATH_EXPR, forwarder::ContractBuilder);
    blockchain
}

struct ForwarderTestState {
    world: ScenarioWorld,
    forwarder_contract: ForwarderContract,
}

impl ForwarderTestState {
    fn new() -> Self {
        let mut world = world();

        let forwarder_code = world.code_expression(FORWARDER_PATH_EXPR);
        let roles = vec![
            "ESDTRoleNFTCreate".to_string(),
            "ESDTRoleNFTUpdateAttributes".to_string(),
        ];

        world.set_state_step(
            SetStateStep::new()
                .put_account(USER_ADDRESS_EXPR, Account::new().nonce(1))
                .put_account(
                    FORWARDER_ADDRESS_EXPR,
                    Account::new()
                        .nonce(1)
                        .code(forwarder_code.clone())
                        .esdt_roles(NFT_TOKEN_ID_EXPR, roles),
                ),
        );

        let forwarder_contract = ForwarderContract::new(FORWARDER_ADDRESS_EXPR);

        Self {
            world,
            forwarder_contract,
        }
    }
}

#[test]
fn test_nft_update_attributes_and_send() {
    let mut state = ForwarderTestState::new();

    let original_attributes = Color { r: 0, g: 0, b: 0 };

    state.world.sc_call(
        ScCallStep::new().from(USER_ADDRESS_EXPR).call(
            state
                .forwarder_contract
                .nft_create_compact(NFT_TOKEN_ID, 1u64, original_attributes),
        ),
    );

    state.world.transfer_step(
        TransferStep::new()
            .from(FORWARDER_ADDRESS_EXPR)
            .to(USER_ADDRESS_EXPR)
            .esdt_transfer(NFT_TOKEN_ID, 1, "1"),
    );

    state
        .world
        .check_state_step(CheckStateStep::new().put_account(
            USER_ADDRESS_EXPR,
            CheckAccount::new().esdt_nft_balance_and_attributes(
                NFT_TOKEN_ID_EXPR,
                1,
                "1",
                Some(original_attributes),
            ),
        ));

    let new_attributes = Color {
        r: 255,
        g: 255,
        b: 255,
    };

    state.world.transfer_step(
        TransferStep::new()
            .from(USER_ADDRESS_EXPR)
            .to(FORWARDER_ADDRESS_EXPR)
            .esdt_transfer(NFT_TOKEN_ID, 1, "1"),
    );

    state.world.sc_call(
        ScCallStep::new().from(USER_ADDRESS_EXPR).call(
            state
                .forwarder_contract
                .nft_update_attributes(NFT_TOKEN_ID, 1u64, new_attributes),
        ),
    );

    state.world.transfer_step(
        TransferStep::new()
            .from(FORWARDER_ADDRESS_EXPR)
            .to(USER_ADDRESS_EXPR)
            .esdt_transfer(NFT_TOKEN_ID, 1, "1"),
    );

    state
        .world
        .check_state_step(CheckStateStep::new().put_account(
            USER_ADDRESS_EXPR,
            CheckAccount::new().esdt_nft_balance_and_attributes(
                NFT_TOKEN_ID_EXPR,
                1,
                "1",
                Some(new_attributes),
            ),
        ));
}
