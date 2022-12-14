use elrond_wasm::types::ManagedBuffer;
use elrond_wasm_debug::{
    managed_buffer, rust_biguint, testing_framework::BlockchainStateWrapper, DebugApi,
};
use fractional_nfts::FractionalNfts;

const ISSUE_COST: u64 = 50000000000000000; // 0.05 EGLD

#[test]
fn test_split_and_merge() {
    let _ = DebugApi::dummy();

    let mut wrapper = BlockchainStateWrapper::new();

    let owner = wrapper.create_user_account(&rust_biguint!(ISSUE_COST));
    let alice = wrapper.create_user_account(&rust_biguint!(0));

    let nft_token_id = b"NFT-123456";
    let nft_nonce = 1;
    let nft_amount = rust_biguint!(1);
    let nft_attributes: ManagedBuffer<DebugApi> = managed_buffer!(b"NFT test attributes");
    let nft_creator = &owner;

    let royalties = 42u64;

    let fractional_token_ticker = managed_buffer!(b"FRACTIONAL");
    let fractional_token_display_name = managed_buffer!(b"My fractional token");
    let initial_fractional_amount = rust_biguint!(1_000u64);

    let fractionalized_token_id = b"FRACTIONAL-12345";
    let fractionalized_nft_name = managed_buffer!(b"My fractionalized NFT");
    let fractionalized_nft_attributes = managed_buffer!(b"My fractionalized NFT attributes");

    wrapper.set_nft_balance_all_properties(
        &alice,
        b"NFT-123456",
        nft_nonce,
        &nft_amount,
        &nft_attributes,
        royalties,
        Some(nft_creator),
        None,
        None,
        &[],
    );

    let fractional_nfts_sc = wrapper.create_sc_account(
        &rust_biguint!(0),
        Some(&owner),
        fractional_nfts::contract_obj,
        "fractional nfts",
    );

    // setup the mock contract
    wrapper
        .execute_tx(&owner, &fractional_nfts_sc, &rust_biguint!(0), |sc| {
            sc.init();
        })
        .assert_ok();

    // issue the fractional token
    wrapper
        .execute_tx(
            &owner,
            &fractional_nfts_sc,
            &rust_biguint!(ISSUE_COST),
            |sc| {
                sc.issue_and_set_all_roles(
                    fractional_token_display_name,
                    fractional_token_ticker,
                    0,
                );
            },
        )
        .assert_ok();

    // fractionalize the NFT
    wrapper
        .execute_esdt_transfer(
            &alice,
            &fractional_nfts_sc,
            nft_token_id,
            nft_nonce,
            &nft_amount,
            |sc| {
                sc.fractionalize_nft(
                    initial_fractional_amount.clone().into(),
                    fractionalized_nft_name,
                    fractionalized_nft_attributes,
                );
            },
        )
        .assert_ok();

    // // check the fractionalized amount has been received
    // wrapper.check_esdt_balance(&alice, fractionalized_token_id, &initial_fractional_amount);

    // // check that the royalties are the same as in the original NFT
    // // TODO

    // // check that the NFT is owned by the contract
    // wrapper.check_esdt_balance(&alice, fractionalized_token_id, &initial_fractional_amount);
}
