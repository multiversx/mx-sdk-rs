use multiversx_sc_snippets::imports::{EsdtLocalRole, EsdtTokenType, RustBigUint};
use system_sc_interact::{Config, NftDummyAttributes, SysFuncCallsInteract};

const ISSUE_COST: u64 = 50000000000000000u64;

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn cs_builtin_run_tests() {
    cs_builtin_func_tokens_test().await;
    change_to_dynamic_test().await;
    update_token_test().await;
    modify_creator().await;
    transfer_role().await;
}

async fn cs_builtin_func_tokens_test() {
    let mut interact = SysFuncCallsInteract::init(Config::chain_simulator_config()).await;

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
    let _ = interact
        .issue_token_all_roles(
            RustBigUint::from(ISSUE_COST),
            b"TESTNFT",
            b"TEST",
            0usize,
            EsdtTokenType::DynamicMeta,
        )
        .await;

    // issue dynamic SFT with all roles
    let _ = interact
        .issue_token_all_roles(
            RustBigUint::from(ISSUE_COST),
            b"TESTNFT",
            b"TEST",
            0usize,
            EsdtTokenType::DynamicSFT,
        )
        .await;

    // issue dynamic NFT with all roles
    let dynamic_nft_token_id = interact
        .issue_token_all_roles(
            RustBigUint::from(ISSUE_COST),
            b"TESTNFT",
            b"TEST",
            0usize,
            EsdtTokenType::DynamicNFT,
        )
        .await;

    println!("Dynamic NFT token id issued: {dynamic_nft_token_id:?}");

    // mint NFT
    let nonce = interact
        .mint_nft(
            dynamic_nft_token_id.as_bytes(),
            RustBigUint::from(1u64),
            b"myNFT",
            30u64,
            b"",
            &NftDummyAttributes {
                creation_epoch: 2u64,
                cool_factor: 3u8,
            },
            &Vec::new(),
        )
        .await;

    println!("Dynamic NFT minted at nonce {nonce:?}");

    // modify royalties
    interact
        .modify_royalties(dynamic_nft_token_id.as_bytes(), nonce, 20u64)
        .await;

    println!("Royalties changed for {dynamic_nft_token_id:?} with nonce {nonce:?}");

    // set new uris
    let uris = vec!["thisianuri.com".to_string()];
    interact
        .set_new_uris(dynamic_nft_token_id.as_bytes(), nonce, &uris)
        .await;
    interact
        .check_nft_uris(&dynamic_nft_token_id, nonce, &uris)
        .await;

    println!("New uris set for {dynamic_nft_token_id:?} with nonce {nonce:?}");

    let uris = vec!["uri1_update1".to_string(), "uri2_update2".to_string()];
    // metadata update
    interact
        .metadata_update(
            dynamic_nft_token_id.as_bytes(),
            nonce,
            b"TESTNFT",
            30u64,
            b"new_hash",
            &NftDummyAttributes {
                creation_epoch: 3u64,
                cool_factor: 5u8,
            },
            &uris,
        )
        .await;
    interact
        .check_nft_uris(&dynamic_nft_token_id, nonce, &uris)
        .await;
    println!("Metadata updated for {dynamic_nft_token_id:?} with nonce {nonce:?}");

    let uris = vec!["uri1_recreate1".to_string(), "uri2_recreate2".to_string()];
    // metadata recreate
    interact
        .metadata_recreate(
            dynamic_nft_token_id.as_bytes(),
            nonce,
            b"TESTNFT",
            30u64,
            b"new_hash_recreated",
            &NftDummyAttributes {
                creation_epoch: 100u64,
                cool_factor: 1u8,
            },
            &uris,
        )
        .await;
    interact
        .check_nft_uris(&dynamic_nft_token_id, nonce, &uris)
        .await;
    println!("Metadata recreated for {dynamic_nft_token_id:?} with nonce {nonce:?}. A new token has been created.");

    // metadata recreate with empty uris
    interact
        .metadata_recreate(
            dynamic_nft_token_id.as_bytes(),
            nonce,
            b"TESTNFT",
            30u64,
            b"new_hash_recreated",
            &NftDummyAttributes {
                creation_epoch: 100u64,
                cool_factor: 1u8,
            },
            &vec![],
        )
        .await;
    interact
        .check_nft_uris(&dynamic_nft_token_id, nonce, &vec![])
        .await;
    println!("Metadata recreated for {dynamic_nft_token_id:?} with nonce {nonce:?}. A new token has been created.");
}

async fn change_to_dynamic_test() {
    let mut interact = SysFuncCallsInteract::init(Config::chain_simulator_config()).await;

    // issue NFT with all roles
    let _ = interact
        .issue_token_all_roles(
            RustBigUint::from(ISSUE_COST),
            b"TESTNFT",
            b"TEST",
            0usize,
            EsdtTokenType::NonFungible,
        )
        .await;

    // issue META token with all roles
    let meta_token_id = interact
        .issue_token_all_roles(
            RustBigUint::from(ISSUE_COST),
            b"TESTNFT",
            b"TEST",
            18usize,
            EsdtTokenType::Meta,
        )
        .await;

    // change META to dynamic
    interact.change_to_dynamic(meta_token_id.as_bytes()).await;

    // issue SFT token with all roles
    let sft_token_id = interact
        .issue_token_all_roles(
            RustBigUint::from(ISSUE_COST),
            b"TESTNFT",
            b"TEST",
            18usize,
            EsdtTokenType::SemiFungible,
        )
        .await;

    // change SFT to dynamic
    interact.change_to_dynamic(sft_token_id.as_bytes()).await;
}

async fn update_token_test() {
    let mut interact = SysFuncCallsInteract::init(Config::chain_simulator_config()).await;

    // issue NFT with all roles
    let nft_token_id = interact
        .issue_token_all_roles(
            RustBigUint::from(ISSUE_COST),
            b"TESTNFT",
            b"TEST",
            0usize,
            EsdtTokenType::NonFungible,
        )
        .await;

    // update NFT
    interact.update_token(nft_token_id.as_bytes()).await;
}

async fn modify_creator() {
    let mut interact = SysFuncCallsInteract::init(Config::chain_simulator_config()).await;

    // issue dynamic NFT
    let dynamic_nft_token_id = interact
        .issue_dynamic_token(
            RustBigUint::from(ISSUE_COST),
            b"TESTNFT",
            b"TEST",
            EsdtTokenType::DynamicNFT,
            0usize,
        )
        .await;

    // set roles
    interact
        .set_roles(
            dynamic_nft_token_id.as_bytes(),
            vec![EsdtLocalRole::NftCreate],
        )
        .await;

    // mint NFT
    let nonce = interact
        .mint_nft(
            dynamic_nft_token_id.as_bytes(),
            RustBigUint::from(1u64),
            b"myNFT",
            30u64,
            b"",
            &NftDummyAttributes {
                creation_epoch: 2u64,
                cool_factor: 3u8,
            },
            &Vec::new(),
        )
        .await;

    println!("Dynamic NFT minted at nonce {nonce:?}");

    // set roles for other_address
    interact
        .set_roles_for_other(
            dynamic_nft_token_id.as_bytes(),
            vec![EsdtLocalRole::ModifyCreator],
        )
        .await;

    // send to other_address
    interact
        .send_esdt(dynamic_nft_token_id.as_bytes(), 1u64, 1u64.into())
        .await;

    // modify creator
    interact
        .modify_creator(dynamic_nft_token_id.as_bytes(), nonce)
        .await;
}

async fn transfer_role() {
    let mut interact = SysFuncCallsInteract::init(Config::chain_simulator_config()).await;

    // issue dynamic NFT
    let dynamic_nft_token_id = interact
        .issue_dynamic_token(
            RustBigUint::from(ISSUE_COST),
            b"TESTNFT",
            b"TEST",
            EsdtTokenType::DynamicNFT,
            0usize,
        )
        .await;

    // set roles
    interact
        .set_roles(
            dynamic_nft_token_id.as_bytes(),
            vec![EsdtLocalRole::Transfer],
        )
        .await;

    // get roles
    interact.get_roles(dynamic_nft_token_id.as_bytes()).await;
}
