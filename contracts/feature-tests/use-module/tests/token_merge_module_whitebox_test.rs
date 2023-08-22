#![allow(unused)]

use multiversx_sc::{storage::mappers::StorageTokenWrapper, types::Address};
use multiversx_sc_modules::token_merge::merged_token_setup::MergedTokenSetupModule;
use multiversx_sc_scenario::{
    managed_token_id, rust_biguint,
    scenario_model::{Account, AddressValue, ScCallStep, SetStateStep},
    testing_framework::TxTokenTransfer,
    ScenarioWorld, WhiteboxContract,
};

const OWNER_ADDRESS_EXPR: &str = "address:owner";
const USER_ADDRESS_EXPR: &str = "address:user";

const USE_MODULE_ADDRESS_EXPR: &str = "sc:use-module";
const USE_MODULE_PATH_EXPR: &str = "file:output/use-module.wasm";

const MERGED_TOKEN_ID_EXPR: &str = "str:MERGED-123456";
const MERGED_TOKEN_ID: &[u8] = b"MERGED-123456";
const NFT_TOKEN_ID_EXPR: &str = "str:NFT-123456";
const NFT_TOKEN_ID: &[u8] = b"NFT-123456";
const FUNGIBLE_TOKEN_ID_EXPR: &str = "str:FUN-123456";
const FUNGIBLE_TOKEN_ID: &[u8] = b"FUN-123456";

const NFT_AMOUNT: u64 = 1;
const FUNGIBLE_AMOUNT: u64 = 100;

const FIRST_NFT_NONCE: u64 = 5;
const FIRST_ATTRIBUTES: &[u8] = b"FirstAttributes";
const FIRST_ROYALTIES: u64 = 1_000;
const FIRST_URIS: &[&[u8]] = &[b"FirstUri", b"SecondUri"];

const SECOND_NFT_NONCE: u64 = 7;
const SECOND_ATTRIBUTES: &[u8] = b"SecondAttributes";
const SECOND_ROYALTIES: u64 = 5_000;
const SECOND_URIS: &[&[u8]] = &[b"cool.com/safe_file.exe"];

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/features-tests/use-module");

    blockchain.register_contract(USE_MODULE_PATH_EXPR, use_module::ContractBuilder);
    blockchain
}

#[test]
fn test_token_merge() {
    let mut world = world();

    let use_module_whitebox =
        WhiteboxContract::new(USE_MODULE_ADDRESS_EXPR, use_module::contract_obj);
    let use_modue_code = world.code_expression(USE_MODULE_PATH_EXPR);

    let roles = vec![
        "ESDTRoleNFTCreate".to_string(),
        "ESDTRoleNFTBurn".to_string(),
    ];

    world.set_state_step(
        SetStateStep::new()
            .put_account(OWNER_ADDRESS_EXPR, Account::new().nonce(1))
            .put_account(
                USER_ADDRESS_EXPR,
                Account::new()
                    .nonce(1)
                    .esdt_balance(FUNGIBLE_TOKEN_ID_EXPR, rust_biguint!(FUNGIBLE_AMOUNT)),
            )
            .put_account(
                USE_MODULE_ADDRESS_EXPR,
                Account::new()
                    .nonce(1)
                    .code(use_modue_code)
                    .owner(OWNER_ADDRESS_EXPR)
                    .esdt_roles(MERGED_TOKEN_ID_EXPR, roles),
            ),
    );

    world.whitebox_call(
        &use_module_whitebox,
        ScCallStep::new().from(OWNER_ADDRESS_EXPR),
        |sc| {
            sc.merged_token()
                .set_token_id(managed_token_id!(MERGED_TOKEN_ID));
            let _ = sc
                .mergeable_tokens_whitelist()
                .insert(managed_token_id!(NFT_TOKEN_ID));
            let _ = sc
                .mergeable_tokens_whitelist()
                .insert(managed_token_id!(FUNGIBLE_TOKEN_ID));
        },
    );

    // TODO: implement esdt_nft_balance_all_properties
    //     b_mock.set_nft_balance_all_properties(
    //     &user,
    //     NFT_TOKEN_ID,
    //     FIRST_NFT_NONCE,
    //     &rust_biguint!(NFT_AMOUNT),
    //     &FIRST_ATTRIBUTES.to_vec(),
    //     FIRST_ROYALTIES,
    //     None,
    //     None,
    //     None,
    //     &uris_to_vec(FIRST_URIS),
    // );
    // b_mock.set_nft_balance_all_properties(
    //     &user,
    //     NFT_TOKEN_ID,
    //     SECOND_NFT_NONCE,
    //     &rust_biguint!(NFT_AMOUNT),
    //     &SECOND_ATTRIBUTES.to_vec(),
    //     SECOND_ROYALTIES,
    //     None,
    //     None,
    //     None,
    //     &uris_to_vec(SECOND_URIS),
    // );

    // merge two NFTs
    let nft_transfers = vec![
        TxTokenTransfer {
            token_identifier: NFT_TOKEN_ID.to_vec(),
            nonce: FIRST_NFT_NONCE,
            value: rust_biguint!(NFT_AMOUNT),
        },
        TxTokenTransfer {
            token_identifier: NFT_TOKEN_ID.to_vec(),
            nonce: SECOND_NFT_NONCE,
            value: rust_biguint!(NFT_AMOUNT),
        },
    ];
}

#[test]
fn test_partial_split() {}

#[test]
fn test_custom_attributes() {}

fn address_expr_to_address(address_expr: &str) -> Address {
    AddressValue::from(address_expr).to_address()
}

fn uris_to_vec(uris: &[&[u8]]) -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    for uri in uris {
        out.push((*uri).to_vec());
    }

    out
}
