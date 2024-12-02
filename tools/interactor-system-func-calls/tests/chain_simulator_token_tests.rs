use multiversx_sc_snippets::imports::{EsdtTokenType, RustBigUint};
use system_sc_interact::{Config, NftDummyAttributes, SysFuncCallsInteract};

#[tokio::test]
#[ignore = "fixes needed"]
async fn cs_builtin_func_tokens_test() {
    let mut interact = SysFuncCallsInteract::init(Config::chain_simulator_config()).await;

    // issue dynamic NFT
    let dynamic_nft_token_id = interact
        .issue_token(
            RustBigUint::from(50000000000000000u64),
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
            Vec::new(),
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
        .set_new_uris(dynamic_nft_token_id.as_bytes(), nonce, uris)
        .await;

    println!("New uris set for {dynamic_nft_token_id:?} with nonce {nonce:?}");
}
