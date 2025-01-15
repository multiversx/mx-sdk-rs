use forwarder::forwarder_proxy;

use multiversx_sc_scenario::imports::*;

const USER_ADDRESS: TestAddress = TestAddress::new("user");
const FORWARDER_ADDRESS: TestSCAddress = TestSCAddress::new("forwarder");
const FORWARDER_PATH: MxscPath = MxscPath::new("output/forwarder.mxsc.json");

const NFT_TOKEN_ID: TestTokenIdentifier = TestTokenIdentifier::new("COOL-123456");
const NFT_TOKEN: &[u8] = b"COOL-123456";

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.set_current_dir_from_workspace("contracts/feature-tests/composability");
    blockchain.register_contract(FORWARDER_PATH, forwarder::ContractBuilder);
    blockchain
}

struct ForwarderTestState {
    world: ScenarioWorld,
}

impl ForwarderTestState {
    fn new() -> Self {
        let mut world = world();

        let roles = vec![
            "ESDTRoleNFTCreate".to_string(),
            "ESDTRoleNFTUpdateAttributes".to_string(),
        ];

        world.account(USER_ADDRESS).nonce(1);
        world
            .account(FORWARDER_ADDRESS)
            .nonce(1)
            .code(FORWARDER_PATH)
            .esdt_roles(NFT_TOKEN_ID, roles);

        Self { world }
    }
}

#[test]
fn test_nft_update_attributes_and_send() {
    let mut state = ForwarderTestState::new();

    let original_attributes = forwarder_proxy::Color { r: 0, g: 0, b: 0 };

    state
        .world
        .tx()
        .from(USER_ADDRESS)
        .to(FORWARDER_ADDRESS)
        .typed(forwarder_proxy::ForwarderProxy)
        .nft_create_compact(NFT_TOKEN_ID, 1u64, original_attributes)
        .run();

    state.world.transfer_step(
        TransferStep::new()
            .from(FORWARDER_ADDRESS.eval_to_expr().as_str())
            .to(USER_ADDRESS.eval_to_expr().as_str())
            .esdt_transfer(NFT_TOKEN, 1, "1"),
    );

    state
        .world
        .check_account(USER_ADDRESS)
        .esdt_nft_balance_and_attributes(NFT_TOKEN_ID, 1, 1, original_attributes);

    let new_attributes = forwarder_proxy::Color {
        r: 255,
        g: 255,
        b: 255,
    };

    state.world.transfer_step(
        TransferStep::new()
            .from(USER_ADDRESS.eval_to_expr().as_str())
            .to(FORWARDER_ADDRESS.eval_to_expr().as_str())
            .esdt_transfer(NFT_TOKEN, 1, "1"),
    );

    state
        .world
        .tx()
        .from(USER_ADDRESS)
        .to(FORWARDER_ADDRESS)
        .typed(forwarder_proxy::ForwarderProxy)
        .nft_update_attributes(NFT_TOKEN_ID, 1u64, new_attributes)
        .run();

    state.world.transfer_step(
        TransferStep::new()
            .from(FORWARDER_ADDRESS.eval_to_expr().as_str())
            .to(USER_ADDRESS.eval_to_expr().as_str())
            .esdt_transfer(NFT_TOKEN, 1, "1"),
    );

    state
        .world
        .check_account(USER_ADDRESS)
        .esdt_nft_balance_and_attributes(NFT_TOKEN_ID, 1, 1, new_attributes);
}
