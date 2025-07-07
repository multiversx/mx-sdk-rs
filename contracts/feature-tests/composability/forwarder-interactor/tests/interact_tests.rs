use forwarder_interact::{Color, Config, ContractInteract};
use multiversx_sc_snippets::imports::*;

const ISSUE_COST: u64 = 50000000000000000u64;

// Simple deploy test that runs on the real blockchain configuration.
// In order for this test to work, make sure that the `config.toml` file contains the real blockchain config (or choose it manually)
// Can be run with `sc-meta test`.
#[tokio::test]
#[ignore = "run on demand, relies on real blockchain state"]
async fn deploy_test_forwarder() {
    let mut interactor = ContractInteract::new(Config::new(), None).await;

    interactor.deploy().await;
}

#[tokio::test]
#[ignore = "run on demand"]
async fn builtin_func_tokens_test() {
    let mut interact = ContractInteract::new(Config::new(), None).await;

    // deploy forwarder
    interact.deploy().await;

    // issue dynamic NFT
    interact
        .issue_dynamic_token(
            RustBigUint::from(ISSUE_COST),
            b"TESTNFT",
            b"TEST",
            EsdtTokenType::DynamicNFT,
            0usize,
        )
        .await;

    // issue dynamic SFT
    interact
        .issue_dynamic_token(
            RustBigUint::from(ISSUE_COST),
            b"TESTNFT",
            b"TEST",
            EsdtTokenType::DynamicSFT,
            0usize,
        )
        .await;

    // issue dynamic META
    interact
        .issue_dynamic_token(
            RustBigUint::from(ISSUE_COST),
            b"TESTNFT",
            b"TEST",
            EsdtTokenType::DynamicMeta,
            18usize,
        )
        .await;

    // issue dynamic META with all roles
    interact
        .issue_token_all_roles(
            RustBigUint::from(ISSUE_COST),
            b"TESTNFT",
            b"TEST",
            0usize,
            EsdtTokenType::DynamicMeta,
        )
        .await;

    // issue dynamic SFT with all roles
    interact
        .issue_token_all_roles(
            RustBigUint::from(ISSUE_COST),
            b"TESTNFT",
            b"TEST",
            0usize,
            EsdtTokenType::DynamicSFT,
        )
        .await;

    // issue dynamic NFT with all roles
    interact
        .issue_token_all_roles(
            RustBigUint::from(ISSUE_COST),
            b"TESTNFT",
            b"TEST",
            0usize,
            EsdtTokenType::DynamicNFT,
        )
        .await;

    let dynamic_nft_token_id = interact.last_issued_token().await;

    println!("Dynamic NFT token id issued: {dynamic_nft_token_id:?}");

    // mint NFT
    interact
        .nft_create(
            dynamic_nft_token_id.as_bytes(),
            RustBigUint::from(1u64),
            b"myNFT",
            30u64,
            b"",
            &Color {
                r: 1u8,
                g: 2u8,
                b: 5u8,
            },
            b"sample_uri",
        )
        .await;

    let nonce = 1u64;

    println!("Dynamic NFT minted at nonce {nonce:?}");

    // modify royalties
    interact
        .modify_royalties(dynamic_nft_token_id.as_bytes(), nonce, 20u64)
        .await;

    println!("Royalties changed for {dynamic_nft_token_id:?} with nonce {nonce:?}");

    // set new uris
    let uris = vec!["thisianuri.com".to_string()];
    interact
        .set_new_uris(dynamic_nft_token_id.as_bytes(), nonce, uris)
        .await;

    println!("New uris set for {dynamic_nft_token_id:?} with nonce {nonce:?}");

    // metadata update
    interact
        .metadata_update(
            dynamic_nft_token_id.as_bytes(),
            nonce,
            b"TESTNFT",
            30u64,
            b"new_hash",
            &Color {
                r: 6u8,
                g: 7u8,
                b: 8u8,
            },
            Vec::new(),
        )
        .await;

    println!("Metadata updated for {dynamic_nft_token_id:?} with nonce {nonce:?}");

    // metadata recreate
    interact
        .metadata_recreate(
            dynamic_nft_token_id.as_bytes(),
            nonce,
            b"TESTNFT",
            30u64,
            b"new_hash_recreated",
            &Color {
                r: 8u8,
                g: 8u8,
                b: 8u8,
            },
            Vec::new(),
        )
        .await;

    println!("Metadata recreated for {dynamic_nft_token_id:?} with nonce {nonce:?}. A new token has been created.");
}

#[tokio::test]
#[ignore = "run on demand"]
async fn change_to_dynamic_test() {
    let mut interact = ContractInteract::new(Config::new(), None).await;

    // deploy forwarder
    interact.deploy().await;

    // issue NFT with all roles
    interact
        .issue_token_all_roles(
            RustBigUint::from(ISSUE_COST),
            b"TESTNFT",
            b"TEST",
            0usize,
            EsdtTokenType::NonFungible,
        )
        .await;

    // issue META token with all roles
    interact
        .issue_token_all_roles(
            RustBigUint::from(ISSUE_COST),
            b"TESTNFT",
            b"TEST",
            18usize,
            EsdtTokenType::MetaFungible,
        )
        .await;

    // get token id from the contract
    let meta_token_id = interact.last_issued_token().await;

    // change META to dynamic
    interact.change_to_dynamic(meta_token_id.as_bytes()).await;

    // issue SFT token with all roles
    interact
        .issue_token_all_roles(
            RustBigUint::from(ISSUE_COST),
            b"TESTNFT",
            b"TEST",
            18usize,
            EsdtTokenType::SemiFungible,
        )
        .await;

    // get token id from the contract
    let sft_token_id = interact.last_issued_token().await;

    // change SFT to dynamic
    interact.change_to_dynamic(sft_token_id.as_bytes()).await;
}

#[tokio::test]
#[ignore = "run on demand"]
async fn update_token_test() {
    let mut interact = ContractInteract::new(Config::new(), None).await;

    // deploy forwarder
    interact.deploy().await;

    // issue NFT with all roles
    interact
        .issue_token_all_roles(
            RustBigUint::from(ISSUE_COST),
            b"TESTNFT",
            b"TEST",
            0usize,
            EsdtTokenType::NonFungible,
        )
        .await;

    // get token id from the contract
    let nft_token_id = interact.last_issued_token().await;

    // update NFT
    interact.update_token(nft_token_id.as_bytes()).await;
}

#[tokio::test]
#[ignore = "run on demand"]
async fn modify_creator() {
    let mut interact = ContractInteract::new(Config::new(), None).await;

    // deploy forwarder
    interact.deploy().await;

    let wallet_address = interact.wallet_address.clone();
    let sc_address = interact.state.current_address().clone();

    // issue dynamic NFT
    let dynamic_nft_token_id = interact
        .issue_dynamic_token_from_wallet(
            RustBigUint::from(ISSUE_COST),
            b"TESTNFT",
            b"TEST",
            EsdtTokenType::DynamicNFT,
            0usize,
        )
        .await;

    // set roles for self to mint
    interact
        .set_roles_from_wallet(
            &wallet_address,
            dynamic_nft_token_id.as_bytes(),
            vec![EsdtLocalRole::NftCreate],
        )
        .await;

    // mint NFT
    let nonce = interact
        .mint_nft_from_wallet(
            dynamic_nft_token_id.as_bytes(),
            RustBigUint::from(1u64),
            b"myNFT",
            30u64,
            b"",
            &Color {
                r: 1u8,
                g: 2u8,
                b: 3u8,
            },
            Vec::new(),
        )
        .await;

    println!("Dynamic NFT minted at nonce {nonce:?}");

    // set modify creator role for the contract
    interact
        .set_roles_from_wallet(
            &sc_address.to_address(),
            dynamic_nft_token_id.as_bytes(),
            vec![EsdtLocalRole::ModifyCreator],
        )
        .await;

    // send nft to the contract
    interact
        .send_esdt_from_wallet(
            &sc_address.to_address(),
            dynamic_nft_token_id.as_bytes(),
            1u64,
            1u64.into(),
        )
        .await;

    // modify creator into the contract (from wallet to SC through a SC call)
    interact
        .modify_creator(dynamic_nft_token_id.as_bytes(), nonce)
        .await;
}
