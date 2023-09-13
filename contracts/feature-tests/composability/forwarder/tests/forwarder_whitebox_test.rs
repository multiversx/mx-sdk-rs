use forwarder::nft::{Color, ForwarderNftModule};
use multiversx_sc::{contract_base::ContractBase, types::Address};
use multiversx_sc_scenario::{
    managed_address, managed_biguint, managed_token_id, rust_biguint,
    scenario_model::{
        Account, AddressValue, CheckAccount, CheckStateStep, ScCallStep, SetStateStep,
    },
    ScenarioWorld, WhiteboxContract,
};

const USER_ADDRESS_EXPR: &str = "address:user";
const FORWARDER_ADDRESS_EXPR: &str = "sc:forwarder";
const FORWARDER_PATH_EXPR: &str = "file:output/forwarder.wasm";

const NFT_TOKEN_ID_EXPR: &str = "str:COOL-123456";
const NFT_TOKEN_ID: &[u8] = b"COOL-123456";

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/composability/forwarder");

    blockchain.register_contract(FORWARDER_PATH_EXPR, forwarder::ContractBuilder);
    blockchain
}

#[test]
fn test_nft_update_attributes_and_send() {
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

    let forwarder_whitebox = WhiteboxContract::new(FORWARDER_ADDRESS_EXPR, forwarder::contract_obj);

    let original_attributes = Color { r: 0, g: 0, b: 0 };

    world.whitebox_call(
        &forwarder_whitebox,
        ScCallStep::new().from(USER_ADDRESS_EXPR),
        |sc| {
            sc.nft_create_compact(
                managed_token_id!(NFT_TOKEN_ID),
                managed_biguint!(1),
                original_attributes,
            );

            sc.send().direct_esdt(
                &managed_address!(&address_expr_to_address(USER_ADDRESS_EXPR)),
                &managed_token_id!(NFT_TOKEN_ID),
                1,
                &managed_biguint!(1),
            );
        },
    );

    world.check_state_step(CheckStateStep::new().put_account(
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

    world.whitebox_call(
        &forwarder_whitebox,
        ScCallStep::new()
            .from(USER_ADDRESS_EXPR)
            .esdt_transfer(NFT_TOKEN_ID, 1, rust_biguint!(1)),
        |sc| {
            sc.nft_update_attributes(managed_token_id!(NFT_TOKEN_ID), 1, new_attributes);

            sc.send().direct_esdt(
                &managed_address!(&address_expr_to_address(USER_ADDRESS_EXPR)),
                &managed_token_id!(NFT_TOKEN_ID),
                1,
                &managed_biguint!(1),
            );
        },
    );

    world.check_state_step(CheckStateStep::new().put_account(
        USER_ADDRESS_EXPR,
        CheckAccount::new().esdt_nft_balance_and_attributes(
            NFT_TOKEN_ID_EXPR,
            1,
            "1",
            Some(new_attributes),
        ),
    ));
}

fn address_expr_to_address(address_expr: &str) -> Address {
    AddressValue::from(address_expr).to_address()
}
