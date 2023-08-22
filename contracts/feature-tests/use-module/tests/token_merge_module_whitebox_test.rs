#![allow(unused)]

use multiversx_sc::types::Address;
use multiversx_sc_scenario::{scenario_model::AddressValue, ScenarioWorld};

const OWNER_ADDRESS_EXPR: &str = "address:owner";

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
fn test_token_merge() {}

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
