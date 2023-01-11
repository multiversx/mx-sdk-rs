use forwarder::nft::{Color, ForwarderNftModule};
use multiversx_sc::{contract_base::ContractBase, types::EsdtLocalRole};
use multiversx_sc_scenario::{
    managed_address, managed_biguint, managed_token_id, rust_biguint,
    testing_framework::BlockchainStateWrapper,
};

static NFT_TOKEN_ID: &[u8] = b"COOL-123456";

#[test]
fn nft_update_attributes_and_send_test() {
    let mut b_mock = BlockchainStateWrapper::new();
    let rust_zero = rust_biguint!(0);

    let user = b_mock.create_user_account(&rust_zero);
    let forw_wrapper =
        b_mock.create_sc_account(&rust_zero, None, forwarder::contract_obj, "forwarder path");

    b_mock.set_esdt_local_roles(
        forw_wrapper.address_ref(),
        NFT_TOKEN_ID,
        &[EsdtLocalRole::NftCreate, EsdtLocalRole::NftUpdateAttributes],
    );

    let original_attributes = Color { r: 0, g: 0, b: 0 };
    b_mock
        .execute_tx(&user, &forw_wrapper, &rust_zero, |sc| {
            sc.nft_create_compact(
                managed_token_id!(NFT_TOKEN_ID),
                managed_biguint!(1),
                original_attributes,
            );

            sc.send().direct_esdt(
                &managed_address!(&user),
                &managed_token_id!(NFT_TOKEN_ID),
                1,
                &managed_biguint!(1),
            );
        })
        .assert_ok();

    b_mock.check_nft_balance(
        &user,
        NFT_TOKEN_ID,
        1,
        &rust_biguint!(1),
        Some(&original_attributes),
    );

    let new_attributes = Color {
        r: 255,
        g: 255,
        b: 255,
    };
    b_mock
        .execute_esdt_transfer(
            &user,
            &forw_wrapper,
            NFT_TOKEN_ID,
            1,
            &rust_biguint!(1),
            |sc| {
                sc.nft_update_attributes(managed_token_id!(NFT_TOKEN_ID), 1, new_attributes);

                sc.send().direct_esdt(
                    &managed_address!(&user),
                    &managed_token_id!(NFT_TOKEN_ID),
                    1,
                    &managed_biguint!(1),
                );
            },
        )
        .assert_ok();

    b_mock.check_nft_balance(
        &user,
        NFT_TOKEN_ID,
        1,
        &rust_biguint!(1),
        Some(&new_attributes),
    );
}
