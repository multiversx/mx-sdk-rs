#![allow(deprecated)] // TODO: migrate tests

use multiversx_sc::{
    arrayvec::ArrayVec,
    codec::Empty,
    contract_base::ContractBase,
    storage::mappers::StorageTokenWrapper,
    types::{EsdtLocalRole, EsdtTokenPayment, ManagedVec},
};
use multiversx_sc_modules::token_merge::{
    merged_token_instances::MergedTokenInstances, merged_token_setup::MergedTokenSetupModule,
};
use multiversx_sc_scenario::{
    managed_address, managed_biguint, managed_token_id, rust_biguint,
    testing_framework::{BlockchainStateWrapper, TxTokenTransfer},
};
use use_module::token_merge_mod_impl::{CustomAttributes, TokenMergeModImpl};

static MERGED_TOKEN_ID: &[u8] = b"MERGED-123456";
static NFT_TOKEN_ID: &[u8] = b"NFT-123456";
static FUNGIBLE_TOKEN_ID: &[u8] = b"FUN-123456";

const NFT_AMOUNT: u64 = 1;
const FUNGIBLE_AMOUNT: u64 = 100;

const FIRST_NFT_NONCE: u64 = 5;
static FIRST_ATTRIBUTES: &[u8] = b"FirstAttributes";
const FIRST_ROYALTIES: u64 = 1_000;
static FIRST_URIS: &[&[u8]] = &[b"FirstUri", b"SecondUri"];

const SECOND_NFT_NONCE: u64 = 7;
static SECOND_ATTRIBUTES: &[u8] = b"SecondAttributes";
const SECOND_ROYALTIES: u64 = 5_000;
static SECOND_URIS: &[&[u8]] = &[b"cool.com/safe_file.exe"];

#[test]
fn test_token_merge() {
    let rust_zero = rust_biguint!(0);
    let mut b_mock = BlockchainStateWrapper::new();
    let owner = b_mock.create_user_account(&rust_zero);
    let user = b_mock.create_user_account(&rust_zero);
    let merging_sc = b_mock.create_sc_account(
        &rust_zero,
        Some(&owner),
        use_module::contract_obj,
        "wasm path",
    );

    b_mock
        .execute_tx(&owner, &merging_sc, &rust_zero, |sc| {
            sc.merged_token()
                .set_token_id(managed_token_id!(MERGED_TOKEN_ID));
            let _ = sc
                .mergeable_tokens_whitelist()
                .insert(managed_token_id!(NFT_TOKEN_ID));
            let _ = sc
                .mergeable_tokens_whitelist()
                .insert(managed_token_id!(FUNGIBLE_TOKEN_ID));
        })
        .assert_ok();
    b_mock.set_esdt_local_roles(
        merging_sc.address_ref(),
        MERGED_TOKEN_ID,
        &[EsdtLocalRole::NftCreate, EsdtLocalRole::NftBurn],
    );

    b_mock.set_esdt_balance(&user, FUNGIBLE_TOKEN_ID, &rust_biguint!(FUNGIBLE_AMOUNT));
    b_mock.set_nft_balance_all_properties(
        &user,
        NFT_TOKEN_ID,
        FIRST_NFT_NONCE,
        &rust_biguint!(NFT_AMOUNT),
        &FIRST_ATTRIBUTES.to_vec(),
        FIRST_ROYALTIES,
        None,
        None,
        None,
        &uris_to_vec(FIRST_URIS),
    );
    b_mock.set_nft_balance_all_properties(
        &user,
        NFT_TOKEN_ID,
        SECOND_NFT_NONCE,
        &rust_biguint!(NFT_AMOUNT),
        &SECOND_ATTRIBUTES.to_vec(),
        SECOND_ROYALTIES,
        None,
        None,
        None,
        &uris_to_vec(SECOND_URIS),
    );

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
    b_mock
        .execute_esdt_multi_transfer(&user, &merging_sc, &nft_transfers, |sc| {
            let merged_token = sc.merge_tokens_endpoint();
            assert_eq!(
                merged_token.token_identifier,
                managed_token_id!(MERGED_TOKEN_ID)
            );
            assert_eq!(merged_token.token_nonce, 1);
            assert_eq!(merged_token.amount, managed_biguint!(NFT_AMOUNT));

            let merged_token_data = sc.blockchain().get_esdt_token_data(
                &managed_address!(&user),
                &managed_token_id!(MERGED_TOKEN_ID),
                1,
            );
            let mut expected_uri = ArrayVec::new();
            expected_uri.push(EsdtTokenPayment::new(
                managed_token_id!(NFT_TOKEN_ID),
                FIRST_NFT_NONCE,
                managed_biguint!(NFT_AMOUNT),
            ));
            expected_uri.push(EsdtTokenPayment::new(
                managed_token_id!(NFT_TOKEN_ID),
                SECOND_NFT_NONCE,
                managed_biguint!(NFT_AMOUNT),
            ));

            let actual_uri = MergedTokenInstances::decode_from_first_uri(&merged_token_data.uris);
            assert_eq!(expected_uri, actual_uri.into_instances());

            assert_eq!(
                merged_token_data.royalties,
                managed_biguint!(SECOND_ROYALTIES)
            );
        })
        .assert_ok();

    b_mock.check_nft_balance(
        &user,
        MERGED_TOKEN_ID,
        1,
        &rust_biguint!(NFT_AMOUNT),
        Option::<&Empty>::None,
    );
    b_mock.check_nft_balance(
        merging_sc.address_ref(),
        NFT_TOKEN_ID,
        FIRST_NFT_NONCE,
        &rust_biguint!(NFT_AMOUNT),
        Option::<&Empty>::None,
    );
    b_mock.check_nft_balance(
        merging_sc.address_ref(),
        NFT_TOKEN_ID,
        SECOND_NFT_NONCE,
        &rust_biguint!(NFT_AMOUNT),
        Option::<&Empty>::None,
    );

    // split nfts
    b_mock
        .execute_esdt_transfer(
            &user,
            &merging_sc,
            MERGED_TOKEN_ID,
            1,
            &rust_biguint!(NFT_AMOUNT),
            |sc| {
                let output_tokens = sc.split_tokens_endpoint();
                let mut expected_output_tokens = ManagedVec::new();
                expected_output_tokens.push(EsdtTokenPayment::new(
                    managed_token_id!(NFT_TOKEN_ID),
                    FIRST_NFT_NONCE,
                    managed_biguint!(NFT_AMOUNT),
                ));
                expected_output_tokens.push(EsdtTokenPayment::new(
                    managed_token_id!(NFT_TOKEN_ID),
                    SECOND_NFT_NONCE,
                    managed_biguint!(NFT_AMOUNT),
                ));
                assert_eq!(output_tokens, expected_output_tokens);
            },
        )
        .assert_ok();

    b_mock.check_nft_balance(
        &user,
        NFT_TOKEN_ID,
        FIRST_NFT_NONCE,
        &rust_biguint!(NFT_AMOUNT),
        Option::<&Empty>::None,
    );
    b_mock.check_nft_balance(
        &user,
        NFT_TOKEN_ID,
        SECOND_NFT_NONCE,
        &rust_biguint!(NFT_AMOUNT),
        Option::<&Empty>::None,
    );

    // merge the NFT with fungible
    let esdt_transfers = vec![
        TxTokenTransfer {
            token_identifier: NFT_TOKEN_ID.to_vec(),
            nonce: FIRST_NFT_NONCE,
            value: rust_biguint!(NFT_AMOUNT),
        },
        TxTokenTransfer {
            token_identifier: FUNGIBLE_TOKEN_ID.to_vec(),
            nonce: 0,
            value: rust_biguint!(FUNGIBLE_AMOUNT),
        },
    ];
    b_mock
        .execute_esdt_multi_transfer(&user, &merging_sc, &esdt_transfers, |sc| {
            let merged_token = sc.merge_tokens_endpoint();
            assert_eq!(
                merged_token.token_identifier,
                managed_token_id!(MERGED_TOKEN_ID)
            );
            assert_eq!(merged_token.token_nonce, 2);
            assert_eq!(merged_token.amount, managed_biguint!(NFT_AMOUNT));

            let merged_token_data = sc.blockchain().get_esdt_token_data(
                &managed_address!(&user),
                &managed_token_id!(MERGED_TOKEN_ID),
                2,
            );
            let mut expected_uri = ArrayVec::new();
            expected_uri.push(EsdtTokenPayment::new(
                managed_token_id!(NFT_TOKEN_ID),
                FIRST_NFT_NONCE,
                managed_biguint!(NFT_AMOUNT),
            ));
            expected_uri.push(EsdtTokenPayment::new(
                managed_token_id!(FUNGIBLE_TOKEN_ID),
                0,
                managed_biguint!(FUNGIBLE_AMOUNT),
            ));

            let actual_uri = MergedTokenInstances::decode_from_first_uri(&merged_token_data.uris);
            assert_eq!(expected_uri, actual_uri.into_instances());

            assert_eq!(
                merged_token_data.royalties,
                managed_biguint!(FIRST_ROYALTIES)
            );
        })
        .assert_ok();

    b_mock.check_nft_balance(
        &user,
        MERGED_TOKEN_ID,
        2,
        &rust_biguint!(NFT_AMOUNT),
        Option::<&Empty>::None,
    );

    // merge NFT with an already merged token
    let combined_transfers = vec![
        TxTokenTransfer {
            token_identifier: NFT_TOKEN_ID.to_vec(),
            nonce: SECOND_NFT_NONCE,
            value: rust_biguint!(NFT_AMOUNT),
        },
        TxTokenTransfer {
            token_identifier: MERGED_TOKEN_ID.to_vec(),
            nonce: 2,
            value: rust_biguint!(NFT_AMOUNT),
        },
    ];
    b_mock
        .execute_esdt_multi_transfer(&user, &merging_sc, &combined_transfers, |sc| {
            let merged_token = sc.merge_tokens_endpoint();
            assert_eq!(
                merged_token.token_identifier,
                managed_token_id!(MERGED_TOKEN_ID)
            );
            assert_eq!(merged_token.token_nonce, 3);
            assert_eq!(merged_token.amount, managed_biguint!(NFT_AMOUNT));

            let merged_token_data = sc.blockchain().get_esdt_token_data(
                &managed_address!(&user),
                &managed_token_id!(MERGED_TOKEN_ID),
                3,
            );
            let mut expected_uri = ArrayVec::new();
            expected_uri.push(EsdtTokenPayment::new(
                managed_token_id!(NFT_TOKEN_ID),
                FIRST_NFT_NONCE,
                managed_biguint!(NFT_AMOUNT),
            ));
            expected_uri.push(EsdtTokenPayment::new(
                managed_token_id!(FUNGIBLE_TOKEN_ID),
                0,
                managed_biguint!(FUNGIBLE_AMOUNT),
            ));
            expected_uri.push(EsdtTokenPayment::new(
                managed_token_id!(NFT_TOKEN_ID),
                SECOND_NFT_NONCE,
                managed_biguint!(NFT_AMOUNT),
            ));

            let actual_uri = MergedTokenInstances::decode_from_first_uri(&merged_token_data.uris);
            assert_eq!(expected_uri, actual_uri.into_instances());

            assert_eq!(
                merged_token_data.royalties,
                managed_biguint!(SECOND_ROYALTIES)
            );
        })
        .assert_ok();

    b_mock.check_nft_balance(
        &user,
        MERGED_TOKEN_ID,
        3,
        &rust_biguint!(NFT_AMOUNT),
        Option::<&Empty>::None,
    );

    // split the 3 merged tokens
    b_mock
        .execute_esdt_transfer(
            &user,
            &merging_sc,
            MERGED_TOKEN_ID,
            3,
            &rust_biguint!(NFT_AMOUNT),
            |sc| {
                let output_tokens = sc.split_tokens_endpoint();
                let mut expected_output_tokens = ManagedVec::new();
                expected_output_tokens.push(EsdtTokenPayment::new(
                    managed_token_id!(NFT_TOKEN_ID),
                    FIRST_NFT_NONCE,
                    managed_biguint!(NFT_AMOUNT),
                ));
                expected_output_tokens.push(EsdtTokenPayment::new(
                    managed_token_id!(FUNGIBLE_TOKEN_ID),
                    0,
                    managed_biguint!(FUNGIBLE_AMOUNT),
                ));
                expected_output_tokens.push(EsdtTokenPayment::new(
                    managed_token_id!(NFT_TOKEN_ID),
                    SECOND_NFT_NONCE,
                    managed_biguint!(NFT_AMOUNT),
                ));

                assert_eq!(output_tokens, expected_output_tokens);
            },
        )
        .assert_ok();

    b_mock.check_nft_balance(
        &user,
        NFT_TOKEN_ID,
        FIRST_NFT_NONCE,
        &rust_biguint!(NFT_AMOUNT),
        Option::<&Empty>::None,
    );
    b_mock.check_nft_balance(
        &user,
        NFT_TOKEN_ID,
        SECOND_NFT_NONCE,
        &rust_biguint!(NFT_AMOUNT),
        Option::<&Empty>::None,
    );
    b_mock.check_esdt_balance(&user, FUNGIBLE_TOKEN_ID, &rust_biguint!(FUNGIBLE_AMOUNT));
}

#[test]
fn partial_split_test() {
    let rust_zero = rust_biguint!(0);
    let mut b_mock = BlockchainStateWrapper::new();
    let owner = b_mock.create_user_account(&rust_zero);
    let user = b_mock.create_user_account(&rust_zero);
    let merging_sc = b_mock.create_sc_account(
        &rust_zero,
        Some(&owner),
        use_module::contract_obj,
        "wasm path",
    );

    b_mock
        .execute_tx(&owner, &merging_sc, &rust_zero, |sc| {
            sc.merged_token()
                .set_token_id(managed_token_id!(MERGED_TOKEN_ID));
            let _ = sc
                .mergeable_tokens_whitelist()
                .insert(managed_token_id!(NFT_TOKEN_ID));
            let _ = sc
                .mergeable_tokens_whitelist()
                .insert(managed_token_id!(FUNGIBLE_TOKEN_ID));
        })
        .assert_ok();
    b_mock.set_esdt_local_roles(
        merging_sc.address_ref(),
        MERGED_TOKEN_ID,
        &[EsdtLocalRole::NftCreate, EsdtLocalRole::NftBurn],
    );

    b_mock.set_esdt_balance(&user, FUNGIBLE_TOKEN_ID, &rust_biguint!(FUNGIBLE_AMOUNT));
    b_mock.set_nft_balance_all_properties(
        &user,
        NFT_TOKEN_ID,
        FIRST_NFT_NONCE,
        &rust_biguint!(NFT_AMOUNT),
        &FIRST_ATTRIBUTES.to_vec(),
        FIRST_ROYALTIES,
        None,
        None,
        None,
        &uris_to_vec(FIRST_URIS),
    );
    b_mock.set_nft_balance_all_properties(
        &user,
        NFT_TOKEN_ID,
        SECOND_NFT_NONCE,
        &rust_biguint!(NFT_AMOUNT),
        &SECOND_ATTRIBUTES.to_vec(),
        SECOND_ROYALTIES,
        None,
        None,
        None,
        &uris_to_vec(SECOND_URIS),
    );

    // merge 2 NFTs and a fungible token
    let esdt_transfers = vec![
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
        TxTokenTransfer {
            token_identifier: FUNGIBLE_TOKEN_ID.to_vec(),
            nonce: 0,
            value: rust_biguint!(FUNGIBLE_AMOUNT),
        },
    ];
    b_mock
        .execute_esdt_multi_transfer(&user, &merging_sc, &esdt_transfers, |sc| {
            let merged_token = sc.merge_tokens_endpoint();
            assert_eq!(
                merged_token.token_identifier,
                managed_token_id!(MERGED_TOKEN_ID)
            );
            assert_eq!(merged_token.token_nonce, 1);
            assert_eq!(merged_token.amount, managed_biguint!(NFT_AMOUNT));

            let merged_token_data = sc.blockchain().get_esdt_token_data(
                &managed_address!(&user),
                &managed_token_id!(MERGED_TOKEN_ID),
                1,
            );
            let mut expected_uri = ArrayVec::new();
            expected_uri.push(EsdtTokenPayment::new(
                managed_token_id!(NFT_TOKEN_ID),
                FIRST_NFT_NONCE,
                managed_biguint!(NFT_AMOUNT),
            ));
            expected_uri.push(EsdtTokenPayment::new(
                managed_token_id!(NFT_TOKEN_ID),
                SECOND_NFT_NONCE,
                managed_biguint!(NFT_AMOUNT),
            ));
            expected_uri.push(EsdtTokenPayment::new(
                managed_token_id!(FUNGIBLE_TOKEN_ID),
                0,
                managed_biguint!(FUNGIBLE_AMOUNT),
            ));

            let actual_uri = MergedTokenInstances::decode_from_first_uri(&merged_token_data.uris);
            assert_eq!(expected_uri, actual_uri.into_instances());
        })
        .assert_ok();

    // split part of the fungible token
    b_mock
        .execute_esdt_transfer(
            &user,
            &merging_sc,
            MERGED_TOKEN_ID,
            1,
            &rust_biguint!(NFT_AMOUNT),
            |sc| {
                let mut tokens_to_remove = ManagedVec::new();
                tokens_to_remove.push(EsdtTokenPayment::new(
                    managed_token_id!(FUNGIBLE_TOKEN_ID),
                    0,
                    managed_biguint!(40),
                ));
                let output_payments = sc.split_token_partial_endpoint(tokens_to_remove);

                let mut expected_output_payments = ManagedVec::new();
                expected_output_payments.push(EsdtTokenPayment::new(
                    managed_token_id!(FUNGIBLE_TOKEN_ID),
                    0,
                    managed_biguint!(40),
                ));
                expected_output_payments.push(EsdtTokenPayment::new(
                    managed_token_id!(MERGED_TOKEN_ID),
                    2,
                    managed_biguint!(NFT_AMOUNT),
                ));
                assert_eq!(output_payments, expected_output_payments);
            },
        )
        .assert_ok();

    // fully remove instance
    b_mock
        .execute_esdt_transfer(
            &user,
            &merging_sc,
            MERGED_TOKEN_ID,
            2,
            &rust_biguint!(NFT_AMOUNT),
            |sc| {
                let mut tokens_to_remove = ManagedVec::new();
                tokens_to_remove.push(EsdtTokenPayment::new(
                    managed_token_id!(NFT_TOKEN_ID),
                    FIRST_NFT_NONCE,
                    managed_biguint!(NFT_AMOUNT),
                ));
                let output_payments = sc.split_token_partial_endpoint(tokens_to_remove);

                let mut expected_output_payments = ManagedVec::new();
                expected_output_payments.push(EsdtTokenPayment::new(
                    managed_token_id!(NFT_TOKEN_ID),
                    FIRST_NFT_NONCE,
                    managed_biguint!(NFT_AMOUNT),
                ));
                expected_output_payments.push(EsdtTokenPayment::new(
                    managed_token_id!(MERGED_TOKEN_ID),
                    3,
                    managed_biguint!(NFT_AMOUNT),
                ));
                assert_eq!(output_payments, expected_output_payments);

                // check newest token attributes
                let merged_token_data = sc.blockchain().get_esdt_token_data(
                    &managed_address!(&user),
                    &managed_token_id!(MERGED_TOKEN_ID),
                    3,
                );
                let mut expected_uri = ArrayVec::new();
                expected_uri.push(EsdtTokenPayment::new(
                    managed_token_id!(NFT_TOKEN_ID),
                    SECOND_NFT_NONCE,
                    managed_biguint!(NFT_AMOUNT),
                ));
                expected_uri.push(EsdtTokenPayment::new(
                    managed_token_id!(FUNGIBLE_TOKEN_ID),
                    0,
                    managed_biguint!(FUNGIBLE_AMOUNT - 40),
                ));

                let actual_uri =
                    MergedTokenInstances::decode_from_first_uri(&merged_token_data.uris);
                assert_eq!(expected_uri, actual_uri.into_instances());

                assert_eq!(
                    merged_token_data.royalties,
                    managed_biguint!(SECOND_ROYALTIES)
                );
            },
        )
        .assert_ok();
}

#[test]
fn custom_attributes_test() {
    let rust_zero = rust_biguint!(0);
    let mut b_mock = BlockchainStateWrapper::new();
    let owner = b_mock.create_user_account(&rust_zero);
    let user = b_mock.create_user_account(&rust_zero);
    let merging_sc = b_mock.create_sc_account(
        &rust_zero,
        Some(&owner),
        use_module::contract_obj,
        "wasm path",
    );

    b_mock
        .execute_tx(&owner, &merging_sc, &rust_zero, |sc| {
            sc.merged_token()
                .set_token_id(managed_token_id!(MERGED_TOKEN_ID));
            let _ = sc
                .mergeable_tokens_whitelist()
                .insert(managed_token_id!(NFT_TOKEN_ID));
            let _ = sc
                .mergeable_tokens_whitelist()
                .insert(managed_token_id!(FUNGIBLE_TOKEN_ID));
        })
        .assert_ok();
    b_mock.set_esdt_local_roles(
        merging_sc.address_ref(),
        MERGED_TOKEN_ID,
        &[EsdtLocalRole::NftCreate, EsdtLocalRole::NftBurn],
    );

    b_mock.set_esdt_balance(&user, FUNGIBLE_TOKEN_ID, &rust_biguint!(FUNGIBLE_AMOUNT));
    b_mock.set_nft_balance_all_properties(
        &user,
        NFT_TOKEN_ID,
        FIRST_NFT_NONCE,
        &rust_biguint!(NFT_AMOUNT),
        &FIRST_ATTRIBUTES.to_vec(),
        FIRST_ROYALTIES,
        None,
        None,
        None,
        &uris_to_vec(FIRST_URIS),
    );
    b_mock.set_nft_balance_all_properties(
        &user,
        NFT_TOKEN_ID,
        SECOND_NFT_NONCE,
        &rust_biguint!(NFT_AMOUNT),
        &SECOND_ATTRIBUTES.to_vec(),
        SECOND_ROYALTIES,
        None,
        None,
        None,
        &uris_to_vec(SECOND_URIS),
    );

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
    b_mock
        .execute_esdt_multi_transfer(&user, &merging_sc, &nft_transfers, |sc| {
            let merged_token = sc.merge_tokens_custom_attributes_endpoint();
            assert_eq!(
                merged_token.token_identifier,
                managed_token_id!(MERGED_TOKEN_ID)
            );
            assert_eq!(merged_token.token_nonce, 1);
            assert_eq!(merged_token.amount, managed_biguint!(NFT_AMOUNT));

            let merged_token_data = sc.blockchain().get_esdt_token_data(
                &managed_address!(&user),
                &managed_token_id!(MERGED_TOKEN_ID),
                1,
            );
            let mut expected_uri = ArrayVec::new();
            expected_uri.push(EsdtTokenPayment::new(
                managed_token_id!(NFT_TOKEN_ID),
                FIRST_NFT_NONCE,
                managed_biguint!(NFT_AMOUNT),
            ));
            expected_uri.push(EsdtTokenPayment::new(
                managed_token_id!(NFT_TOKEN_ID),
                SECOND_NFT_NONCE,
                managed_biguint!(NFT_AMOUNT),
            ));

            let actual_uri = MergedTokenInstances::decode_from_first_uri(&merged_token_data.uris);
            assert_eq!(expected_uri, actual_uri.into_instances());

            let expected_attributes = CustomAttributes {
                first: 5u32,
                second: 10u64,
            };
            let actual_attributes: CustomAttributes = merged_token_data.decode_attributes();
            assert_eq!(expected_attributes, actual_attributes);

            assert_eq!(
                merged_token_data.royalties,
                managed_biguint!(SECOND_ROYALTIES)
            );
        })
        .assert_ok();

    b_mock.check_nft_balance(
        &user,
        MERGED_TOKEN_ID,
        1,
        &rust_biguint!(NFT_AMOUNT),
        Option::<&Empty>::None,
    );
    b_mock.check_nft_balance(
        merging_sc.address_ref(),
        NFT_TOKEN_ID,
        FIRST_NFT_NONCE,
        &rust_biguint!(NFT_AMOUNT),
        Option::<&Empty>::None,
    );
    b_mock.check_nft_balance(
        merging_sc.address_ref(),
        NFT_TOKEN_ID,
        SECOND_NFT_NONCE,
        &rust_biguint!(NFT_AMOUNT),
        Option::<&Empty>::None,
    );
}

fn uris_to_vec(uris: &[&[u8]]) -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    for uri in uris {
        out.push((*uri).to_vec());
    }

    out
}
