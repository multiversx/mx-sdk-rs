// #![allow(deprecated)] // TODO: migrate to unified syntax at some point

use multiversx_sc_scenario::imports::*;

use multiversx_sc_modules::token_merge::{
    merged_token_instances::MergedTokenInstances, merged_token_setup::MergedTokenSetupModule,
};
use use_module::token_merge_mod_impl::{CustomAttributes, TokenMergeModImpl};

const OWNER_ADDRESS_EXPR: &str = "address:owner";
const OWNER_ADDRESS_EXPRR: TestAddress = TestAddress::new("owner");
const USER_ADDRESS_EXPR: &str = "address:user";
const USER_ADDRESS_EXPRR: TestAddress = TestAddress::new("user");

const USE_MODULE_ADDRESS_EXPR: &str = "sc:use-module";
const USE_MODULE_ADDRESS_EXPRR: TestSCAddress = TestSCAddress::new("use-module");
const USE_MODULE_PATH_EXPR: &str = "mxsc:output/use-module.mxsc.json";
const USE_MODULE_PATH_EXPRR: MxscPath = MxscPath::new("mxsc:output/use-module.mxsc.json");

const MERGED_TOKEN_ID_EXPR: &str = "str:MERGED-123456";
const MERGED_TOKEN_ID_EXPRR: TestTokenIdentifier = TestTokenIdentifier::new("MERGED-123456");
const MERGED_TOKEN_ID: &[u8] = b"MERGED-123456";
const NFT_TOKEN_ID_EXPR: &str = "str:NFT-123456";
const NFT_TOKEN_ID_EXPRR: TestTokenIdentifier = TestTokenIdentifier::new("NFT-123456");
const NFT_TOKEN_ID: &[u8] = b"NFT-123456";
const FUNGIBLE_TOKEN_ID_EXPR: &str = "str:FUN-123456";
const FUNGIBLE_TOKEN_ID_EXPRR: TestTokenIdentifier = TestTokenIdentifier::new("FUN-123456");
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
    blockchain.register_contract(USE_MODULE_PATH_EXPRR, use_module::ContractBuilder);
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

    world.account(OWNER_ADDRESS_EXPRR).nonce(1);
    world
        .account(USER_ADDRESS_EXPRR)
        .nonce(1)
        .esdt_balance(FUNGIBLE_TOKEN_ID_EXPRR, FUNGIBLE_AMOUNT)
        .esdt_nft_all_properties(
            NFT_TOKEN_ID_EXPRR,
            FIRST_NFT_NONCE,
            NFT_AMOUNT,
            managed_buffer!(FIRST_ATTRIBUTES),
            FIRST_ROYALTIES,
            None::<Address>,
            (),
            first_uris,
        )
        .esdt_nft_all_properties(
            NFT_TOKEN_ID_EXPRR,
            SECOND_NFT_NONCE,
            NFT_AMOUNT,
            managed_buffer!(SECOND_ATTRIBUTES),
            SECOND_ROYALTIES,
            None::<Address>,
            (),
            second_uris,
        );

    world
        .account(USE_MODULE_ADDRESS_EXPRR)
        .nonce(1)
        .code(USE_MODULE_PATH_EXPRR)
        .owner(OWNER_ADDRESS_EXPRR)
        .esdt_roles(MERGED_TOKEN_ID_EXPRR, roles);

    world
        .tx()
        .from(OWNER_ADDRESS_EXPRR)
        .to(USE_MODULE_ADDRESS_EXPRR)
        .whitebox(use_module::contract_obj, |sc| {
            sc.merged_token()
                .set_token_id(MERGED_TOKEN_ID_EXPRR.to_token_identifier());
            let _ = sc
                .mergeable_tokens_whitelist()
                .insert(NFT_TOKEN_ID_EXPRR.to_token_identifier());
            let _ = sc
                .mergeable_tokens_whitelist()
                .insert(FUNGIBLE_TOKEN_ID_EXPRR.to_token_identifier());
        });

    // merge two NFTs
    let nft_transfers = vec![
        TestEsdtTransfer(
            NFT_TOKEN_ID_EXPRR,
            FIRST_NFT_NONCE.into(),
            NFT_AMOUNT.into(),
        ),
        TestEsdtTransfer(
            NFT_TOKEN_ID_EXPRR,
            SECOND_NFT_NONCE.into(),
            NFT_AMOUNT.into(),
        ),
    ];

    world
        .tx()
        .from(USER_ADDRESS_EXPRR)
        .to(USE_MODULE_ADDRESS_EXPRR)
        .multi_esdt(nft_transfers)
        .whitebox(use_module::contract_obj, |sc| {
            let merged_token = sc.merge_tokens_endpoint();
            assert_eq!(
                merged_token.token_identifier,
                MERGED_TOKEN_ID_EXPRR.to_token_identifier()
            );
            assert_eq!(merged_token.token_nonce, 1);
            assert_eq!(merged_token.amount, managed_biguint!(NFT_AMOUNT));

            let merged_token_data = sc.blockchain().get_esdt_token_data(
                &USER_ADDRESS_EXPRR.to_managed_address(),
                &MERGED_TOKEN_ID_EXPRR.to_token_identifier(),
                1,
            );
            let expected_uri = vec![
                TestEsdtTransfer(NFT_TOKEN_ID_EXPRR, FIRST_NFT_NONCE, NFT_AMOUNT.into()),
                TestEsdtTransfer(NFT_TOKEN_ID_EXPRR, SECOND_NFT_NONCE, NFT_AMOUNT.into()),
            ];
            // expected_uri.push();
            // expected_uri.push();

            let actual_urii = MergedTokenInstances::decode_from_first_uri(&merged_token_data.uris);
            let mut actual_uri = Vec::new();
            for uri in actual_urii.into_instances() {
                let token_identifier_string = uri.token_identifier.to_string();
                let token: TestTokenIdentifier =
                    TestTokenIdentifier::new(token_identifier_string);
                actual_uri.push(TestEsdtTransfer(
                    token,
                    uri.token_nonce,
                    uri.amount.to_u64().unwrap(),
                ));
            }
            assert_eq!(expected_uri, actual_uri);

            assert_eq!(
                merged_token_data.royalties,
                managed_biguint!(SECOND_ROYALTIES)
            );
        });

    world
        .check_account(USER_ADDRESS_EXPRR)
        .esdt_nft_balance_and_attributes(MERGED_TOKEN_ID_EXPRR, 1, NFT_AMOUNT, &Empty);

    world
        .check_account(USE_MODULE_ADDRESS_EXPRR)
        .esdt_nft_balance_and_attributes(
            NFT_TOKEN_ID_EXPRR,
            FIRST_NFT_NONCE,
            NFT_AMOUNT,
            FIRST_ATTRIBUTES,
        );

    world
        .check_account(USE_MODULE_ADDRESS_EXPRR)
        .esdt_nft_balance_and_attributes(
            NFT_TOKEN_ID_EXPRR,
            SECOND_NFT_NONCE,
            NFT_AMOUNT,
            SECOND_ATTRIBUTES,
        );

    // split nfts
    world
        .tx()
        .from(USER_ADDRESS_EXPRR)
        .to(USE_MODULE_ADDRESS_EXPRR)
        .payment(TestEsdtTransfer(MERGED_TOKEN_ID_EXPRR, 1, NFT_AMOUNT))
        .whitebox(use_module::contract_obj, |sc| {
            let output_tokens = sc.split_tokens_endpoint();
            let expected_output_tokens = vec![
                EsdtTokenPayment::new(NFT_TOKEN_ID.into(), FIRST_NFT_NONCE, NFT_AMOUNT.into()),
                EsdtTokenPayment::new(NFT_TOKEN_ID.into(), SECOND_NFT_NONCE, NFT_AMOUNT.into()),
            ];
            assert_eq!(output_tokens, expected_output_tokens.into());
        });

    world
        .check_account(USER_ADDRESS_EXPRR)
        .esdt_nft_balance_and_attributes(
            NFT_TOKEN_ID_EXPRR,
            FIRST_NFT_NONCE,
            NFT_AMOUNT,
            FIRST_ATTRIBUTES,
        );

    world
        .check_account(USER_ADDRESS_EXPRR)
        .esdt_nft_balance_and_attributes(
            NFT_TOKEN_ID_EXPRR,
            SECOND_NFT_NONCE,
            NFT_AMOUNT,
            SECOND_ATTRIBUTES,
        );

    // merge the NFT with fungible
    let esdt_transfers = vec![
        EsdtTokenPayment::new(
            NFT_TOKEN_ID.into(),
            FIRST_NFT_NONCE.into(),
            NFT_AMOUNT.into(),
        ),
        EsdtTokenPayment::new(
            FUNGIBLE_TOKEN_ID.into(),
            0u64.into(),
            FUNGIBLE_AMOUNT.into(),
        ),
    ];

    world
        .tx()
        .from(USER_ADDRESS_EXPRR)
        .to(USE_MODULE_ADDRESS_EXPRR)
        .multi_esdt(esdt_transfers)
        .whitebox(use_module::contract_obj, |sc| {
            let merged_token = sc.merge_tokens_endpoint();
            assert_eq!(
                merged_token.token_identifier,
                MERGED_TOKEN_ID_EXPRR.to_token_identifier()
            );
            assert_eq!(merged_token.token_nonce, 2);
            assert_eq!(merged_token.amount, managed_biguint!(NFT_AMOUNT));

            let merged_token_data = sc.blockchain().get_esdt_token_data(
                &USER_ADDRESS_EXPRR.to_managed_address(),
                &MERGED_TOKEN_ID_EXPRR.to_token_identifier(),
                2,
            );
            let mut expected_uri = ArrayVec::new();
            expected_uri.push(EsdtTokenPayment::new(
                NFT_TOKEN_ID.into(),
                FIRST_NFT_NONCE,
                NFT_AMOUNT.into(),
            ));
            expected_uri.push(EsdtTokenPayment::new(
                FUNGIBLE_TOKEN_ID.into(),
                0,
                FUNGIBLE_AMOUNT.into(),
            ));

            let actual_uri = MergedTokenInstances::decode_from_first_uri(&merged_token_data.uris);
            assert_eq!(expected_uri, actual_uri.into_instances());

            assert_eq!(
                merged_token_data.royalties,
                managed_biguint!(FIRST_ROYALTIES)
            );
        });

    world
        .check_account(USER_ADDRESS_EXPRR)
        .esdt_nft_balance_and_attributes(MERGED_TOKEN_ID_EXPRR, 2, NFT_AMOUNT, &Empty);

    // merge NFT with an already merged token
    let combined_transfers = vec![
        EsdtTokenPayment::new(
            NFT_TOKEN_ID.into(),
            SECOND_NFT_NONCE.into(),
            NFT_AMOUNT.into(),
        ),
        EsdtTokenPayment::new(MERGED_TOKEN_ID.into(), 2u64.into(), NFT_AMOUNT.into()),
    ];

    world
        .tx()
        .from(USER_ADDRESS_EXPRR)
        .to(USE_MODULE_ADDRESS_EXPRR)
        .multi_esdt(combined_transfers)
        .whitebox(use_module::contract_obj, |sc| {
            let merged_token = sc.merge_tokens_endpoint();
            assert_eq!(
                merged_token.token_identifier,
                MERGED_TOKEN_ID_EXPRR.to_token_identifier()
            );
            assert_eq!(merged_token.token_nonce, 3);
            assert_eq!(merged_token.amount, managed_biguint!(NFT_AMOUNT));

            let merged_token_data = sc.blockchain().get_esdt_token_data(
                &USER_ADDRESS_EXPRR.to_managed_address(),
                &MERGED_TOKEN_ID_EXPRR.into(),
                3,
            );
            let mut expected_uri = ArrayVec::new();
            expected_uri.push(EsdtTokenPayment::new(
                NFT_TOKEN_ID.into(),
                FIRST_NFT_NONCE,
                NFT_AMOUNT.into(),
            ));
            expected_uri.push(EsdtTokenPayment::new(
                FUNGIBLE_TOKEN_ID.into(),
                0,
                FUNGIBLE_AMOUNT.into(),
            ));
            expected_uri.push(EsdtTokenPayment::new(
                NFT_TOKEN_ID.into(),
                SECOND_NFT_NONCE,
                NFT_AMOUNT.into(),
            ));

            let actual_uri = MergedTokenInstances::decode_from_first_uri(&merged_token_data.uris);
            assert_eq!(expected_uri, actual_uri.into_instances());

            assert_eq!(
                merged_token_data.royalties,
                managed_biguint!(SECOND_ROYALTIES)
            );
        });

    world
        .check_account(USER_ADDRESS_EXPRR)
        .esdt_nft_balance_and_attributes(MERGED_TOKEN_ID_EXPRR, 3, NFT_AMOUNT, &Empty);

    world
        .tx()
        .from(USER_ADDRESS_EXPRR)
        .to(USE_MODULE_ADDRESS_EXPRR)
        .esdt((
            MERGED_TOKEN_ID_EXPRR.to_token_identifier(),
            3,
            NFT_AMOUNT.into(),
        ))
        .whitebox(use_module::contract_obj, |sc| {
            let output_tokens = sc.split_tokens_endpoint();
            let mut expected_output_tokens = ManagedVec::new();
            expected_output_tokens.push(EsdtTokenPayment::new(
                NFT_TOKEN_ID.into(),
                FIRST_NFT_NONCE,
                NFT_AMOUNT.into(),
            ));
            expected_output_tokens.push(EsdtTokenPayment::new(
                FUNGIBLE_TOKEN_ID.into(),
                0,
                FUNGIBLE_AMOUNT.into(),
            ));
            expected_output_tokens.push(EsdtTokenPayment::new(
                NFT_TOKEN_ID.into(),
                SECOND_NFT_NONCE,
                NFT_AMOUNT.into(),
            ));

            assert_eq!(output_tokens, expected_output_tokens);
        });

    world
        .check_account(USER_ADDRESS_EXPRR)
        .esdt_nft_balance_and_attributes(
            NFT_TOKEN_ID_EXPRR,
            FIRST_NFT_NONCE,
            NFT_AMOUNT,
            FIRST_ATTRIBUTES,
        );

    world
        .check_account(USER_ADDRESS_EXPRR)
        .esdt_nft_balance_and_attributes(
            NFT_TOKEN_ID_EXPRR,
            SECOND_NFT_NONCE,
            NFT_AMOUNT,
            SECOND_ATTRIBUTES,
        );

    world
        .check_account(USER_ADDRESS_EXPRR)
        .esdt_balance(FUNGIBLE_TOKEN_ID_EXPRR, FUNGIBLE_AMOUNT);
}

// #[test]
// fn test_partial_split() {
//     let mut world = world();

//     let use_module_whitebox =
//         WhiteboxContract::new(USE_MODULE_ADDRESS_EXPR, use_module::contract_obj);
//     let use_module_code = world.code_expression(USE_MODULE_PATH_EXPR);

//     let roles = vec![
//         "ESDTRoleNFTCreate".to_string(),
//         "ESDTRoleNFTBurn".to_string(),
//     ];

//     world.set_state_step(
//         SetStateStep::new()
//             .put_account(OWNER_ADDRESS_EXPR, Account::new().nonce(1))
//             .put_account(
//                 USER_ADDRESS_EXPR,
//                 Account::new()
//                     .nonce(1)
//                     .esdt_balance(FUNGIBLE_TOKEN_ID_EXPR, FUNGIBLE_AMOUNT)
//                     .esdt_nft_all_properties(
//                         NFT_TOKEN_ID_EXPR,
//                         FIRST_NFT_NONCE,
//                         NFT_AMOUNT,
//                         Some(FIRST_ATTRIBUTES),
//                         FIRST_ROYALTIES,
//                         None::<AddressValue>,
//                         None,
//                         Vec::from(FIRST_URIS),
//                     )
//                     .esdt_nft_all_properties(
//                         NFT_TOKEN_ID_EXPR,
//                         SECOND_NFT_NONCE,
//                         NFT_AMOUNT,
//                         Some(SECOND_ATTRIBUTES),
//                         SECOND_ROYALTIES,
//                         None::<AddressValue>,
//                         None,
//                         Vec::from(SECOND_URIS),
//                     ),
//             )
//             .put_account(
//                 USE_MODULE_ADDRESS_EXPR,
//                 Account::new()
//                     .nonce(1)
//                     .code(use_module_code)
//                     .owner(OWNER_ADDRESS_EXPR)
//                     .esdt_roles(MERGED_TOKEN_ID_EXPR, roles),
//             ),
//     );

//     world.whitebox_call(
//         &use_module_whitebox,
//         ScCallStep::new().from(OWNER_ADDRESS_EXPR),
//         |sc| {
//             sc.merged_token()
//                 .set_token_id(managed_token_id!(MERGED_TOKEN_ID));
//             let _ = sc
//                 .mergeable_tokens_whitelist()
//                 .insert(managed_token_id!(NFT_TOKEN_ID));
//             let _ = sc
//                 .mergeable_tokens_whitelist()
//                 .insert(managed_token_id!(FUNGIBLE_TOKEN_ID));
//         },
//     );

//     // merge 2 NFTs and a fungible token
//     let esdt_transfers = vec![
//         TxESDT {
//             esdt_token_identifier: NFT_TOKEN_ID.into(),
//             nonce: FIRST_NFT_NONCE.into(),
//             esdt_value: NFT_AMOUNT.into(),
//         },
//         TxESDT {
//             esdt_token_identifier: NFT_TOKEN_ID.into(),
//             nonce: SECOND_NFT_NONCE.into(),
//             esdt_value: NFT_AMOUNT.into(),
//         },
//         TxESDT {
//             esdt_token_identifier: FUNGIBLE_TOKEN_ID.into(),
//             nonce: 0u64.into(),
//             esdt_value: FUNGIBLE_AMOUNT.into(),
//         },
//     ];

//     world.whitebox_call(
//         &use_module_whitebox,
//         ScCallStep::new()
//             .from(USER_ADDRESS_EXPR)
//             .multi_esdt_transfer(esdt_transfers),
//         |sc| {
//             let merged_token = sc.merge_tokens_endpoint();
//             assert_eq!(
//                 merged_token.token_identifier,
//                 managed_token_id!(MERGED_TOKEN_ID)
//             );
//             assert_eq!(merged_token.token_nonce, 1);
//             assert_eq!(merged_token.amount, managed_biguint!(NFT_AMOUNT));

//             let merged_token_data = sc.blockchain().get_esdt_token_data(
//                 &managed_address!(&address_expr_to_address(USER_ADDRESS_EXPR)),
//                 &managed_token_id!(MERGED_TOKEN_ID),
//                 1,
//             );
//             let mut expected_uri = ArrayVec::new();
//             expected_uri.push(EsdtTokenPayment::new(
//                 managed_token_id!(NFT_TOKEN_ID),
//                 FIRST_NFT_NONCE,
//                 managed_biguint!(NFT_AMOUNT),
//             ));
//             expected_uri.push(EsdtTokenPayment::new(
//                 managed_token_id!(NFT_TOKEN_ID),
//                 SECOND_NFT_NONCE,
//                 managed_biguint!(NFT_AMOUNT),
//             ));
//             expected_uri.push(EsdtTokenPayment::new(
//                 managed_token_id!(FUNGIBLE_TOKEN_ID),
//                 0,
//                 managed_biguint!(FUNGIBLE_AMOUNT),
//             ));

//             let actual_uri = MergedTokenInstances::decode_from_first_uri(&merged_token_data.uris);
//             assert_eq!(expected_uri, actual_uri.into_instances());
//         },
//     );

//     // split part of the fungible token
//     world.whitebox_call(
//         &use_module_whitebox,
//         ScCallStep::new().from(USER_ADDRESS_EXPR).esdt_transfer(
//             MERGED_TOKEN_ID_EXPR,
//             1,
//             NFT_AMOUNT,
//         ),
//         |sc| {
//             let mut tokens_to_remove = ManagedVec::new();
//             tokens_to_remove.push(EsdtTokenPayment::new(
//                 managed_token_id!(FUNGIBLE_TOKEN_ID),
//                 0,
//                 managed_biguint!(40),
//             ));
//             let output_payments = sc.split_token_partial_endpoint(tokens_to_remove);

//             let mut expected_output_payments = ManagedVec::new();
//             expected_output_payments.push(EsdtTokenPayment::new(
//                 managed_token_id!(FUNGIBLE_TOKEN_ID),
//                 0,
//                 managed_biguint!(40),
//             ));
//             expected_output_payments.push(EsdtTokenPayment::new(
//                 managed_token_id!(MERGED_TOKEN_ID),
//                 2,
//                 managed_biguint!(NFT_AMOUNT),
//             ));
//             assert_eq!(output_payments, expected_output_payments);
//         },
//     );

//     // fully remove instance
//     world.whitebox_call(
//         &use_module_whitebox,
//         ScCallStep::new().from(USER_ADDRESS_EXPR).esdt_transfer(
//             MERGED_TOKEN_ID_EXPR,
//             2,
//             NFT_AMOUNT,
//         ),
//         |sc| {
//             let mut tokens_to_remove = ManagedVec::new();
//             tokens_to_remove.push(EsdtTokenPayment::new(
//                 managed_token_id!(NFT_TOKEN_ID),
//                 FIRST_NFT_NONCE,
//                 managed_biguint!(NFT_AMOUNT),
//             ));
//             let output_payments = sc.split_token_partial_endpoint(tokens_to_remove);

//             let mut expected_output_payments = ManagedVec::new();
//             expected_output_payments.push(EsdtTokenPayment::new(
//                 managed_token_id!(NFT_TOKEN_ID),
//                 FIRST_NFT_NONCE,
//                 managed_biguint!(NFT_AMOUNT),
//             ));
//             expected_output_payments.push(EsdtTokenPayment::new(
//                 managed_token_id!(MERGED_TOKEN_ID),
//                 3,
//                 managed_biguint!(NFT_AMOUNT),
//             ));
//             assert_eq!(output_payments, expected_output_payments);

//             // check newest token attributes
//             let merged_token_data = sc.blockchain().get_esdt_token_data(
//                 &managed_address!(&address_expr_to_address(USER_ADDRESS_EXPR)),
//                 &managed_token_id!(MERGED_TOKEN_ID),
//                 3,
//             );
//             let mut expected_uri = ArrayVec::new();
//             expected_uri.push(EsdtTokenPayment::new(
//                 managed_token_id!(NFT_TOKEN_ID),
//                 SECOND_NFT_NONCE,
//                 managed_biguint!(NFT_AMOUNT),
//             ));
//             expected_uri.push(EsdtTokenPayment::new(
//                 managed_token_id!(FUNGIBLE_TOKEN_ID),
//                 0,
//                 managed_biguint!(FUNGIBLE_AMOUNT - 40),
//             ));

//             let actual_uri = MergedTokenInstances::decode_from_first_uri(&merged_token_data.uris);
//             assert_eq!(expected_uri, actual_uri.into_instances());

//             assert_eq!(
//                 merged_token_data.royalties,
//                 managed_biguint!(SECOND_ROYALTIES)
//             );
//         },
//     );
// }

// #[test]
// fn test_custom_attributes() {
//     let mut world = world();

//     let use_module_whitebox =
//         WhiteboxContract::new(USE_MODULE_ADDRESS_EXPR, use_module::contract_obj);
//     let use_module_code = world.code_expression(USE_MODULE_PATH_EXPR);

//     let roles = vec![
//         "ESDTRoleNFTCreate".to_string(),
//         "ESDTRoleNFTBurn".to_string(),
//     ];

//     world.set_state_step(
//         SetStateStep::new()
//             .put_account(OWNER_ADDRESS_EXPR, Account::new().nonce(1))
//             .put_account(
//                 USER_ADDRESS_EXPR,
//                 Account::new()
//                     .nonce(1)
//                     .esdt_balance(FUNGIBLE_TOKEN_ID_EXPR, FUNGIBLE_AMOUNT)
//                     .esdt_nft_all_properties(
//                         NFT_TOKEN_ID_EXPR,
//                         FIRST_NFT_NONCE,
//                         NFT_AMOUNT,
//                         Some(FIRST_ATTRIBUTES),
//                         FIRST_ROYALTIES,
//                         None::<AddressValue>,
//                         None,
//                         Vec::from(FIRST_URIS),
//                     )
//                     .esdt_nft_all_properties(
//                         NFT_TOKEN_ID_EXPR,
//                         SECOND_NFT_NONCE,
//                         NFT_AMOUNT,
//                         Some(SECOND_ATTRIBUTES),
//                         SECOND_ROYALTIES,
//                         None::<AddressValue>,
//                         None,
//                         Vec::from(SECOND_URIS),
//                     ),
//             )
//             .put_account(
//                 USE_MODULE_ADDRESS_EXPR,
//                 Account::new()
//                     .nonce(1)
//                     .code(use_module_code)
//                     .owner(OWNER_ADDRESS_EXPR)
//                     .esdt_roles(MERGED_TOKEN_ID_EXPR, roles),
//             ),
//     );

//     world.whitebox_call(
//         &use_module_whitebox,
//         ScCallStep::new().from(OWNER_ADDRESS_EXPR),
//         |sc| {
//             sc.merged_token()
//                 .set_token_id(managed_token_id!(MERGED_TOKEN_ID));
//             let _ = sc
//                 .mergeable_tokens_whitelist()
//                 .insert(managed_token_id!(NFT_TOKEN_ID));
//             let _ = sc
//                 .mergeable_tokens_whitelist()
//                 .insert(managed_token_id!(FUNGIBLE_TOKEN_ID));
//         },
//     );

//     // merge two NFTs
//     let nft_transfers = vec![
//         TxESDT {
//             esdt_token_identifier: NFT_TOKEN_ID.into(),
//             nonce: FIRST_NFT_NONCE.into(),
//             esdt_value: NFT_AMOUNT.into(),
//         },
//         TxESDT {
//             esdt_token_identifier: NFT_TOKEN_ID.into(),
//             nonce: SECOND_NFT_NONCE.into(),
//             esdt_value: NFT_AMOUNT.into(),
//         },
//     ];

//     let expected_attributes = CustomAttributes {
//         first: 5u32,
//         second: 10u64,
//     };

//     world.whitebox_call(
//         &use_module_whitebox,
//         ScCallStep::new()
//             .from(USER_ADDRESS_EXPR)
//             .multi_esdt_transfer(nft_transfers),
//         |sc| {
//             let merged_token = sc.merge_tokens_custom_attributes_endpoint();
//             assert_eq!(
//                 merged_token.token_identifier,
//                 managed_token_id!(MERGED_TOKEN_ID)
//             );
//             assert_eq!(merged_token.token_nonce, 1);
//             assert_eq!(merged_token.amount, managed_biguint!(NFT_AMOUNT));

//             let merged_token_data = sc.blockchain().get_esdt_token_data(
//                 &managed_address!(&address_expr_to_address(USER_ADDRESS_EXPR)),
//                 &managed_token_id!(MERGED_TOKEN_ID),
//                 1,
//             );
//             let mut expected_uri = ArrayVec::new();
//             expected_uri.push(EsdtTokenPayment::new(
//                 managed_token_id!(NFT_TOKEN_ID),
//                 FIRST_NFT_NONCE,
//                 managed_biguint!(NFT_AMOUNT),
//             ));
//             expected_uri.push(EsdtTokenPayment::new(
//                 managed_token_id!(NFT_TOKEN_ID),
//                 SECOND_NFT_NONCE,
//                 managed_biguint!(NFT_AMOUNT),
//             ));

//             let actual_uri = MergedTokenInstances::decode_from_first_uri(&merged_token_data.uris);
//             assert_eq!(expected_uri, actual_uri.into_instances());

//             let actual_attributes: CustomAttributes = merged_token_data.decode_attributes();
//             assert_eq!(expected_attributes, actual_attributes);

//             assert_eq!(
//                 merged_token_data.royalties,
//                 managed_biguint!(SECOND_ROYALTIES)
//             );
//         },
//     );

//     world.check_state_step(CheckStateStep::new().put_account(
//         USER_ADDRESS_EXPR,
//         CheckAccount::new().esdt_nft_balance_and_attributes(
//             MERGED_TOKEN_ID_EXPR,
//             1,
//             NFT_AMOUNT,
//             Some(top_encode_to_vec_u8_or_panic(&expected_attributes)),
//         ),
//     ));

//     world.check_state_step(CheckStateStep::new().put_account(
//         USE_MODULE_ADDRESS_EXPR,
//         CheckAccount::new().esdt_nft_balance_and_attributes(
//             NFT_TOKEN_ID_EXPR,
//             FIRST_NFT_NONCE,
//             NFT_AMOUNT,
//             Some(FIRST_ATTRIBUTES),
//         ),
//     ));

//     world.check_state_step(CheckStateStep::new().put_account(
//         USE_MODULE_ADDRESS_EXPR,
//         CheckAccount::new().esdt_nft_balance_and_attributes(
//             NFT_TOKEN_ID_EXPR,
//             SECOND_NFT_NONCE,
//             NFT_AMOUNT,
//             Some(SECOND_ATTRIBUTES),
//         ),
//     ));
// }

fn address_expr_to_address(address_expr: &str) -> Address {
    AddressValue::from(address_expr).to_address()
}
