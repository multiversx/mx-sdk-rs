use multiversx_sc_scenario::imports::*;

use multiversx_sc_modules::token_merge::{
    merged_token_instances::MergedTokenInstances, merged_token_setup::MergedTokenSetupModule,
};
use use_module::token_merge_mod_impl::{CustomAttributes, TokenMergeModImpl};

const OWNER_ADDRESS_EXPR: TestAddress = TestAddress::new("owner");
const USER_ADDRESS_EXPR: TestAddress = TestAddress::new("user");

const USE_MODULE_ADDRESS_EXPR: TestSCAddress = TestSCAddress::new("use-module");
const USE_MODULE_PATH_EXPR: MxscPath = MxscPath::new("mxsc:output/use-module.mxsc.json");

const MERGED_TOKEN_ID_EXPR: TestTokenIdentifier = TestTokenIdentifier::new("MERGED-123456");
const NFT_TOKEN_ID_EXPR: TestTokenIdentifier = TestTokenIdentifier::new("NFT-123456");
const FUNGIBLE_TOKEN_ID_EXPR: TestTokenIdentifier = TestTokenIdentifier::new("FUN-123456");

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

    blockchain.set_current_dir_from_workspace("contracts/feature-tests/use-module");
    blockchain.register_contract(USE_MODULE_PATH_EXPR, use_module::ContractBuilder);
    blockchain
}

#[test]
fn test_token_merge() {
    let mut world = world();

    let roles = vec![
        "ESDTRoleNFTCreate".to_string(),
        "ESDTRoleNFTBurn".to_string(),
    ];
    let first_uris = FIRST_URIS
        .iter()
        .map(|first_uri| managed_buffer!(first_uri))
        .collect();
    let second_uris = SECOND_URIS
        .iter()
        .map(|second_uri| managed_buffer!(second_uri))
        .collect();

    world.account(OWNER_ADDRESS_EXPR).nonce(1);
    world
        .account(USER_ADDRESS_EXPR)
        .nonce(1)
        .esdt_balance(FUNGIBLE_TOKEN_ID_EXPR, FUNGIBLE_AMOUNT)
        .esdt_nft_all_properties(
            NFT_TOKEN_ID_EXPR,
            FIRST_NFT_NONCE,
            NFT_AMOUNT,
            managed_buffer!(FIRST_ATTRIBUTES),
            FIRST_ROYALTIES,
            None::<Address>,
            (),
            first_uris,
        )
        .esdt_nft_all_properties(
            NFT_TOKEN_ID_EXPR,
            SECOND_NFT_NONCE,
            NFT_AMOUNT,
            managed_buffer!(SECOND_ATTRIBUTES),
            SECOND_ROYALTIES,
            None::<Address>,
            (),
            second_uris,
        );

    world
        .account(USE_MODULE_ADDRESS_EXPR)
        .nonce(1)
        .code(USE_MODULE_PATH_EXPR)
        .owner(OWNER_ADDRESS_EXPR)
        .esdt_roles(MERGED_TOKEN_ID_EXPR, roles);

    world
        .tx()
        .from(OWNER_ADDRESS_EXPR)
        .to(USE_MODULE_ADDRESS_EXPR)
        .whitebox(use_module::contract_obj, |sc| {
            sc.merged_token()
                .set_token_id(MERGED_TOKEN_ID_EXPR.to_esdt_token_identifier());
            let _ = sc
                .mergeable_tokens_whitelist()
                .insert(NFT_TOKEN_ID_EXPR.to_esdt_token_identifier());
            let _ = sc
                .mergeable_tokens_whitelist()
                .insert(FUNGIBLE_TOKEN_ID_EXPR.to_esdt_token_identifier());
        });

    // merge two NFTs
    world
        .tx()
        .from(USER_ADDRESS_EXPR)
        .to(USE_MODULE_ADDRESS_EXPR)
        .payment(Payment::try_new(NFT_TOKEN_ID_EXPR, FIRST_NFT_NONCE, NFT_AMOUNT).unwrap())
        .payment(Payment::try_new(NFT_TOKEN_ID_EXPR, SECOND_NFT_NONCE, NFT_AMOUNT).unwrap())
        .whitebox(use_module::contract_obj, |sc| {
            let merged_token = sc.merge_tokens_endpoint();
            assert_eq!(
                merged_token.token_identifier,
                MERGED_TOKEN_ID_EXPR.to_esdt_token_identifier()
            );
            assert_eq!(merged_token.token_nonce, 1);
            assert_eq!(merged_token.amount, managed_biguint!(NFT_AMOUNT));

            let merged_token_data = sc.blockchain().get_esdt_token_data(
                &USER_ADDRESS_EXPR.to_managed_address(),
                &MERGED_TOKEN_ID_EXPR.to_esdt_token_identifier(),
                1,
            );
            let expected_uri = ArrayVec::from([
                EsdtTokenPayment::new(
                    NFT_TOKEN_ID_EXPR.to_esdt_token_identifier(),
                    FIRST_NFT_NONCE,
                    managed_biguint!(NFT_AMOUNT),
                ),
                EsdtTokenPayment::new(
                    NFT_TOKEN_ID_EXPR.to_esdt_token_identifier(),
                    SECOND_NFT_NONCE,
                    managed_biguint!(NFT_AMOUNT),
                ),
            ]);

            let actual_uri = MergedTokenInstances::decode_from_first_uri(&merged_token_data.uris);
            assert_eq!(expected_uri, *actual_uri.into_instances());

            assert_eq!(
                merged_token_data.royalties,
                managed_biguint!(SECOND_ROYALTIES)
            );
        });

    world
        .check_account(USER_ADDRESS_EXPR)
        .esdt_nft_balance_and_attributes(MERGED_TOKEN_ID_EXPR, 1, NFT_AMOUNT, &Empty);

    world
        .check_account(USE_MODULE_ADDRESS_EXPR)
        .esdt_nft_balance_and_attributes(
            NFT_TOKEN_ID_EXPR,
            FIRST_NFT_NONCE,
            NFT_AMOUNT,
            FIRST_ATTRIBUTES,
        );

    world
        .check_account(USE_MODULE_ADDRESS_EXPR)
        .esdt_nft_balance_and_attributes(
            NFT_TOKEN_ID_EXPR,
            SECOND_NFT_NONCE,
            NFT_AMOUNT,
            SECOND_ATTRIBUTES,
        );

    // split nfts
    world
        .tx()
        .from(USER_ADDRESS_EXPR)
        .to(USE_MODULE_ADDRESS_EXPR)
        .payment(Payment::try_new(MERGED_TOKEN_ID_EXPR, 1u64, NFT_AMOUNT).unwrap())
        .whitebox(use_module::contract_obj, |sc| {
            let output_tokens = sc.split_tokens_endpoint();
            let expected_output_tokens = vec![
                EsdtTokenPayment::new(NFT_TOKEN_ID_EXPR.into(), FIRST_NFT_NONCE, NFT_AMOUNT.into()),
                EsdtTokenPayment::new(
                    NFT_TOKEN_ID_EXPR.into(),
                    SECOND_NFT_NONCE,
                    NFT_AMOUNT.into(),
                ),
            ];
            assert_eq!(output_tokens, expected_output_tokens.into());
        });

    world
        .check_account(USER_ADDRESS_EXPR)
        .esdt_nft_balance_and_attributes(
            NFT_TOKEN_ID_EXPR,
            FIRST_NFT_NONCE,
            NFT_AMOUNT,
            FIRST_ATTRIBUTES,
        );

    world
        .check_account(USER_ADDRESS_EXPR)
        .esdt_nft_balance_and_attributes(
            NFT_TOKEN_ID_EXPR,
            SECOND_NFT_NONCE,
            NFT_AMOUNT,
            SECOND_ATTRIBUTES,
        );

    // merge the NFT with fungible
    world
        .tx()
        .from(USER_ADDRESS_EXPR)
        .to(USE_MODULE_ADDRESS_EXPR)
        .payment(Payment::try_new(NFT_TOKEN_ID_EXPR, FIRST_NFT_NONCE, NFT_AMOUNT).unwrap())
        .payment(Payment::try_new(FUNGIBLE_TOKEN_ID_EXPR, 0u64, FUNGIBLE_AMOUNT).unwrap())
        .whitebox(use_module::contract_obj, |sc| {
            let merged_token = sc.merge_tokens_endpoint();
            assert_eq!(
                merged_token.token_identifier,
                MERGED_TOKEN_ID_EXPR.to_esdt_token_identifier()
            );
            assert_eq!(merged_token.token_nonce, 2);
            assert_eq!(merged_token.amount, managed_biguint!(NFT_AMOUNT));

            let merged_token_data = sc.blockchain().get_esdt_token_data(
                &USER_ADDRESS_EXPR.to_managed_address(),
                &MERGED_TOKEN_ID_EXPR.to_esdt_token_identifier(),
                2,
            );
            let expected_uri = ArrayVec::from([
                EsdtTokenPayment::new(
                    NFT_TOKEN_ID_EXPR.to_esdt_token_identifier(),
                    FIRST_NFT_NONCE,
                    managed_biguint!(NFT_AMOUNT),
                ),
                EsdtTokenPayment::new(
                    FUNGIBLE_TOKEN_ID_EXPR.to_esdt_token_identifier(),
                    0,
                    managed_biguint!(FUNGIBLE_AMOUNT),
                ),
            ]);

            let actual_uri = MergedTokenInstances::decode_from_first_uri(&merged_token_data.uris);
            assert_eq!(expected_uri, *actual_uri.into_instances());

            assert_eq!(
                merged_token_data.royalties,
                managed_biguint!(FIRST_ROYALTIES)
            );
        });

    world
        .check_account(USER_ADDRESS_EXPR)
        .esdt_nft_balance_and_attributes(MERGED_TOKEN_ID_EXPR, 2, NFT_AMOUNT, &Empty);

    // merge NFT with an already merged token
    world
        .tx()
        .from(USER_ADDRESS_EXPR)
        .to(USE_MODULE_ADDRESS_EXPR)
        .payment(Payment::try_new(NFT_TOKEN_ID_EXPR, SECOND_NFT_NONCE, NFT_AMOUNT).unwrap())
        .payment(Payment::try_new(MERGED_TOKEN_ID_EXPR, 2u64, NFT_AMOUNT).unwrap())
        .whitebox(use_module::contract_obj, |sc| {
            let merged_token = sc.merge_tokens_endpoint();
            assert_eq!(
                merged_token.token_identifier,
                MERGED_TOKEN_ID_EXPR.to_esdt_token_identifier()
            );
            assert_eq!(merged_token.token_nonce, 3);
            assert_eq!(merged_token.amount, managed_biguint!(NFT_AMOUNT));

            let merged_token_data = sc.blockchain().get_esdt_token_data(
                &USER_ADDRESS_EXPR.to_managed_address(),
                &MERGED_TOKEN_ID_EXPR.into(),
                3,
            );
            let expected_uri = ArrayVec::from([
                EsdtTokenPayment::new(
                    NFT_TOKEN_ID_EXPR.to_esdt_token_identifier(),
                    FIRST_NFT_NONCE,
                    managed_biguint!(NFT_AMOUNT),
                ),
                EsdtTokenPayment::new(
                    FUNGIBLE_TOKEN_ID_EXPR.to_esdt_token_identifier(),
                    0,
                    managed_biguint!(FUNGIBLE_AMOUNT),
                ),
                EsdtTokenPayment::new(
                    NFT_TOKEN_ID_EXPR.to_esdt_token_identifier(),
                    SECOND_NFT_NONCE,
                    managed_biguint!(NFT_AMOUNT),
                ),
            ]);

            let actual_uri = MergedTokenInstances::decode_from_first_uri(&merged_token_data.uris);
            assert_eq!(expected_uri, *actual_uri.into_instances());

            assert_eq!(
                merged_token_data.royalties,
                managed_biguint!(SECOND_ROYALTIES)
            );
        });

    world
        .check_account(USER_ADDRESS_EXPR)
        .esdt_nft_balance_and_attributes(MERGED_TOKEN_ID_EXPR, 3, NFT_AMOUNT, &Empty);

    world
        .tx()
        .from(USER_ADDRESS_EXPR)
        .to(USE_MODULE_ADDRESS_EXPR)
        .payment(Payment::try_new(MERGED_TOKEN_ID_EXPR, 3u64, NFT_AMOUNT).unwrap())
        .whitebox(use_module::contract_obj, |sc| {
            let output_tokens = sc.split_tokens_endpoint();
            let expected_output_tokens = vec![
                EsdtTokenPayment::new(NFT_TOKEN_ID_EXPR.into(), FIRST_NFT_NONCE, NFT_AMOUNT.into()),
                EsdtTokenPayment::new(FUNGIBLE_TOKEN_ID_EXPR.into(), 0, FUNGIBLE_AMOUNT.into()),
                EsdtTokenPayment::new(
                    NFT_TOKEN_ID_EXPR.into(),
                    SECOND_NFT_NONCE,
                    NFT_AMOUNT.into(),
                ),
            ];

            assert_eq!(output_tokens, expected_output_tokens.into());
        });

    world
        .check_account(USER_ADDRESS_EXPR)
        .esdt_nft_balance_and_attributes(
            NFT_TOKEN_ID_EXPR,
            FIRST_NFT_NONCE,
            NFT_AMOUNT,
            FIRST_ATTRIBUTES,
        );

    world
        .check_account(USER_ADDRESS_EXPR)
        .esdt_nft_balance_and_attributes(
            NFT_TOKEN_ID_EXPR,
            SECOND_NFT_NONCE,
            NFT_AMOUNT,
            SECOND_ATTRIBUTES,
        );

    world
        .check_account(USER_ADDRESS_EXPR)
        .esdt_balance(FUNGIBLE_TOKEN_ID_EXPR, FUNGIBLE_AMOUNT);
}

#[test]
fn test_partial_split() {
    let mut world = world();

    let roles = vec![
        "ESDTRoleNFTCreate".to_string(),
        "ESDTRoleNFTBurn".to_string(),
    ];
    let first_uris = FIRST_URIS
        .iter()
        .map(|first_uri| managed_buffer!(first_uri))
        .collect();
    let second_uris = SECOND_URIS
        .iter()
        .map(|second_uri| managed_buffer!(second_uri))
        .collect();

    world.account(OWNER_ADDRESS_EXPR).nonce(1);
    world
        .account(USER_ADDRESS_EXPR)
        .nonce(1)
        .esdt_balance(FUNGIBLE_TOKEN_ID_EXPR, FUNGIBLE_AMOUNT)
        .esdt_nft_all_properties(
            NFT_TOKEN_ID_EXPR,
            FIRST_NFT_NONCE,
            NFT_AMOUNT,
            managed_buffer!(FIRST_ATTRIBUTES),
            FIRST_ROYALTIES,
            None::<AddressValue>,
            (),
            first_uris,
        )
        .esdt_nft_all_properties(
            NFT_TOKEN_ID_EXPR,
            SECOND_NFT_NONCE,
            NFT_AMOUNT,
            managed_buffer!(SECOND_ATTRIBUTES),
            SECOND_ROYALTIES,
            None::<AddressValue>,
            (),
            second_uris,
        );

    world
        .account(USE_MODULE_ADDRESS_EXPR)
        .nonce(1)
        .code(USE_MODULE_PATH_EXPR)
        .owner(OWNER_ADDRESS_EXPR)
        .esdt_roles(MERGED_TOKEN_ID_EXPR, roles);

    world
        .tx()
        .from(OWNER_ADDRESS_EXPR)
        .to(USE_MODULE_ADDRESS_EXPR)
        .whitebox(use_module::contract_obj, |sc| {
            sc.merged_token()
                .set_token_id(MERGED_TOKEN_ID_EXPR.to_esdt_token_identifier());
            sc.mergeable_tokens_whitelist()
                .insert(NFT_TOKEN_ID_EXPR.to_esdt_token_identifier());
            sc.mergeable_tokens_whitelist()
                .insert(FUNGIBLE_TOKEN_ID_EXPR.to_esdt_token_identifier());
        });

    // merge 2 NFTs and a fungible token
    world
        .tx()
        .from(USER_ADDRESS_EXPR)
        .to(USE_MODULE_ADDRESS_EXPR)
        .payment(Payment::try_new(NFT_TOKEN_ID_EXPR, FIRST_NFT_NONCE, NFT_AMOUNT).unwrap())
        .payment(Payment::try_new(NFT_TOKEN_ID_EXPR, SECOND_NFT_NONCE, NFT_AMOUNT).unwrap())
        .payment(Payment::try_new(FUNGIBLE_TOKEN_ID_EXPR, 0u64, FUNGIBLE_AMOUNT).unwrap())
        .whitebox(use_module::contract_obj, |sc| {
            let merged_token = sc.merge_tokens_endpoint();
            assert_eq!(
                merged_token.token_identifier,
                MERGED_TOKEN_ID_EXPR.to_esdt_token_identifier()
            );
            assert_eq!(merged_token.token_nonce, 1);
            assert_eq!(merged_token.amount, managed_biguint!(NFT_AMOUNT));

            let merged_token_data = sc.blockchain().get_esdt_token_data(
                &USER_ADDRESS_EXPR.to_managed_address(),
                &MERGED_TOKEN_ID_EXPR.to_esdt_token_identifier(),
                1,
            );
            let expected_uri = ArrayVec::from([
                EsdtTokenPayment::new(
                    NFT_TOKEN_ID_EXPR.to_esdt_token_identifier(),
                    FIRST_NFT_NONCE,
                    managed_biguint!(NFT_AMOUNT),
                ),
                EsdtTokenPayment::new(
                    NFT_TOKEN_ID_EXPR.to_esdt_token_identifier(),
                    SECOND_NFT_NONCE,
                    managed_biguint!(NFT_AMOUNT),
                ),
                EsdtTokenPayment::new(
                    FUNGIBLE_TOKEN_ID_EXPR.to_esdt_token_identifier(),
                    0,
                    managed_biguint!(FUNGIBLE_AMOUNT),
                ),
            ]);

            let actual_uri = MergedTokenInstances::decode_from_first_uri(&merged_token_data.uris);
            assert_eq!(expected_uri, *actual_uri.into_instances());
        });

    // split part of the fungible token
    world
        .tx()
        .from(USER_ADDRESS_EXPR)
        .to(USE_MODULE_ADDRESS_EXPR)
        .payment(Payment::try_new(MERGED_TOKEN_ID_EXPR, 1u64, NFT_AMOUNT).unwrap())
        .whitebox(use_module::contract_obj, |sc| {
            let mut tokens_to_remove = ManagedVec::new();
            tokens_to_remove.push(EsdtTokenPayment::new(
                FUNGIBLE_TOKEN_ID_EXPR.to_esdt_token_identifier(),
                0,
                managed_biguint!(40),
            ));

            let output_payments = sc.split_token_partial_endpoint(tokens_to_remove);
            let mut expected_output_payments = ManagedVec::new();
            expected_output_payments.push(EsdtTokenPayment::new(
                FUNGIBLE_TOKEN_ID_EXPR.to_esdt_token_identifier(),
                0,
                managed_biguint!(40),
            ));
            expected_output_payments.push(EsdtTokenPayment::new(
                MERGED_TOKEN_ID_EXPR.to_esdt_token_identifier(),
                2,
                managed_biguint!(NFT_AMOUNT),
            ));
            assert_eq!(output_payments, expected_output_payments);
        });

    // fully remove instance
    world
        .tx()
        .from(USER_ADDRESS_EXPR)
        .to(USE_MODULE_ADDRESS_EXPR)
        .payment(Payment::try_new(MERGED_TOKEN_ID_EXPR, 2u64, NFT_AMOUNT).unwrap())
        .whitebox(use_module::contract_obj, |sc| {
            let mut tokens_to_remove = ManagedVec::new();
            tokens_to_remove.push(EsdtTokenPayment::new(
                NFT_TOKEN_ID_EXPR.to_esdt_token_identifier(),
                FIRST_NFT_NONCE,
                managed_biguint!(NFT_AMOUNT),
            ));
            let output_payments = sc.split_token_partial_endpoint(tokens_to_remove);

            let mut expected_output_payments = ManagedVec::new();
            expected_output_payments.push(EsdtTokenPayment::new(
                NFT_TOKEN_ID_EXPR.to_esdt_token_identifier(),
                FIRST_NFT_NONCE,
                managed_biguint!(NFT_AMOUNT),
            ));
            expected_output_payments.push(EsdtTokenPayment::new(
                MERGED_TOKEN_ID_EXPR.to_esdt_token_identifier(),
                3,
                managed_biguint!(NFT_AMOUNT),
            ));
            assert_eq!(output_payments, expected_output_payments);

            // check newest token attributes
            let merged_token_data = sc.blockchain().get_esdt_token_data(
                &USER_ADDRESS_EXPR.to_managed_address(),
                &MERGED_TOKEN_ID_EXPR.to_esdt_token_identifier(),
                3,
            );
            let expected_uri = ArrayVec::from([
                EsdtTokenPayment::new(
                    NFT_TOKEN_ID_EXPR.to_esdt_token_identifier(),
                    SECOND_NFT_NONCE,
                    managed_biguint!(NFT_AMOUNT),
                ),
                EsdtTokenPayment::new(
                    FUNGIBLE_TOKEN_ID_EXPR.to_esdt_token_identifier(),
                    0,
                    managed_biguint!(FUNGIBLE_AMOUNT - 40),
                ),
            ]);

            let actual_uri = MergedTokenInstances::decode_from_first_uri(&merged_token_data.uris);
            assert_eq!(expected_uri, *actual_uri.into_instances());

            assert_eq!(
                merged_token_data.royalties,
                managed_biguint!(SECOND_ROYALTIES)
            );
        });
}

#[test]
fn test_custom_attributes() {
    let mut world = world();

    let roles = vec![
        "ESDTRoleNFTCreate".to_string(),
        "ESDTRoleNFTBurn".to_string(),
    ];

    let first_uris = FIRST_URIS
        .iter()
        .map(|first_uri| managed_buffer!(first_uri))
        .collect();
    let second_uris = SECOND_URIS
        .iter()
        .map(|second_uri| managed_buffer!(second_uri))
        .collect();
    world.account(OWNER_ADDRESS_EXPR).nonce(1);
    world
        .account(USER_ADDRESS_EXPR)
        .nonce(1)
        .esdt_balance(FUNGIBLE_TOKEN_ID_EXPR, FUNGIBLE_AMOUNT)
        .esdt_nft_all_properties(
            NFT_TOKEN_ID_EXPR,
            FIRST_NFT_NONCE,
            NFT_AMOUNT,
            managed_buffer!(FIRST_ATTRIBUTES),
            FIRST_ROYALTIES,
            None::<AddressValue>,
            (),
            first_uris,
        )
        .esdt_nft_all_properties(
            NFT_TOKEN_ID_EXPR,
            SECOND_NFT_NONCE,
            NFT_AMOUNT,
            managed_buffer!(SECOND_ATTRIBUTES),
            SECOND_ROYALTIES,
            None::<AddressValue>,
            (),
            second_uris,
        );
    world
        .account(USE_MODULE_ADDRESS_EXPR)
        .nonce(1)
        .code(USE_MODULE_PATH_EXPR)
        .owner(OWNER_ADDRESS_EXPR)
        .esdt_roles(MERGED_TOKEN_ID_EXPR, roles);

    world
        .tx()
        .from(OWNER_ADDRESS_EXPR)
        .to(USE_MODULE_ADDRESS_EXPR)
        .whitebox(use_module::contract_obj, |sc| {
            sc.merged_token()
                .set_token_id(MERGED_TOKEN_ID_EXPR.to_esdt_token_identifier());
            let _ = sc
                .mergeable_tokens_whitelist()
                .insert(NFT_TOKEN_ID_EXPR.to_esdt_token_identifier());
            let _ = sc
                .mergeable_tokens_whitelist()
                .insert(FUNGIBLE_TOKEN_ID_EXPR.to_esdt_token_identifier());
        });

    // merge two NFTs
    let expected_attributes = CustomAttributes {
        first: 5u32,
        second: 10u64,
    };

    world
        .tx()
        .from(USER_ADDRESS_EXPR)
        .to(USE_MODULE_ADDRESS_EXPR)
        .payment(Payment::try_new(NFT_TOKEN_ID_EXPR, FIRST_NFT_NONCE, NFT_AMOUNT).unwrap())
        .payment(Payment::try_new(NFT_TOKEN_ID_EXPR, SECOND_NFT_NONCE, NFT_AMOUNT).unwrap())
        .whitebox(use_module::contract_obj, |sc| {
            let merged_token = sc.merge_tokens_custom_attributes_endpoint();
            assert_eq!(
                merged_token.token_identifier,
                MERGED_TOKEN_ID_EXPR.to_esdt_token_identifier()
            );
            assert_eq!(merged_token.token_nonce, 1);
            assert_eq!(merged_token.amount, managed_biguint!(NFT_AMOUNT));

            let merged_token_data = sc.blockchain().get_esdt_token_data(
                &USER_ADDRESS_EXPR.to_managed_address(),
                &MERGED_TOKEN_ID_EXPR.to_esdt_token_identifier(),
                1,
            );
            let expected_uri = ArrayVec::from([
                EsdtTokenPayment::new(
                    NFT_TOKEN_ID_EXPR.to_esdt_token_identifier(),
                    FIRST_NFT_NONCE,
                    managed_biguint!(NFT_AMOUNT),
                ),
                EsdtTokenPayment::new(
                    NFT_TOKEN_ID_EXPR.to_esdt_token_identifier(),
                    SECOND_NFT_NONCE,
                    managed_biguint!(NFT_AMOUNT),
                ),
            ]);
            let actual_uri = MergedTokenInstances::decode_from_first_uri(&merged_token_data.uris);
            assert_eq!(expected_uri, *actual_uri.into_instances());

            let actual_attributes: CustomAttributes = merged_token_data.decode_attributes();
            assert_eq!(expected_attributes, actual_attributes);

            assert_eq!(
                merged_token_data.royalties,
                managed_biguint!(SECOND_ROYALTIES)
            );
        });

    world
        .check_account(USER_ADDRESS_EXPR)
        .esdt_nft_balance_and_attributes(MERGED_TOKEN_ID_EXPR, 1, NFT_AMOUNT, expected_attributes);

    world
        .check_account(USE_MODULE_ADDRESS_EXPR)
        .esdt_nft_balance_and_attributes(
            NFT_TOKEN_ID_EXPR,
            FIRST_NFT_NONCE,
            NFT_AMOUNT,
            FIRST_ATTRIBUTES,
        );
    world
        .check_account(USE_MODULE_ADDRESS_EXPR)
        .esdt_nft_balance_and_attributes(
            NFT_TOKEN_ID_EXPR,
            SECOND_NFT_NONCE,
            NFT_AMOUNT,
            SECOND_ATTRIBUTES,
        );
}
