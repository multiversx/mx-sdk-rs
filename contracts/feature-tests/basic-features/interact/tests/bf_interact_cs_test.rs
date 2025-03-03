use basic_features_interact::{BasicFeaturesInteract, Config};
use multiversx_sc_snippets::imports::{
    BigUint, EgldDecimals, ManagedBuffer, ManagedDecimal, ManagedOption, ManagedVec, RustBigUint,
    StaticApi,
};

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn simulator_basic_features_test() {
    let mut bf_interact = BasicFeaturesInteract::init(Config::chain_simulator_config()).await;

    bf_interact.deploy_storage_bytes().await;
    bf_interact.large_storage(15).await;

    let data = bf_interact.get_large_storage().await.to_vec();
    assert_eq!(bf_interact.large_storage_payload, data);

    bf_interact.deploy().await;

    let expected_return_egld_decimal =
        ManagedDecimal::<StaticApi, EgldDecimals>::const_decimals_from_raw(BigUint::from(5u64));
    let return_egld_decimal = bf_interact.returns_egld_decimal(5).await;
    assert_eq!(expected_return_egld_decimal, return_egld_decimal);

    let expected_type_managed_option = ManagedOption::some(BigUint::from(8u16));
    let type_managed_option = bf_interact
        .echo_managed_option(expected_type_managed_option)
        .await;
    assert_eq!(Some(RustBigUint::from(8u16)), type_managed_option);
}

#[tokio::test]
#[ignore = "signature verification is currently unavailable"]
async fn simulator_crypto_test() {
    let mut bf_interact = BasicFeaturesInteract::init(Config::chain_simulator_config()).await;

    bf_interact.deploy_crypto().await;

    verify_secp256r1_signature(&mut bf_interact).await;
    verify_bls_signature(&mut bf_interact).await;
    verify_bls_aggregated_signature(&mut bf_interact).await;
}

async fn verify_secp256r1_signature(interact: &mut BasicFeaturesInteract) {
    let private_key_bytes = vec![
        2, 230, 136, 67, 101, 214, 86, 54, 35, 88, 245, 173, 229, 104, 249, 247, 109, 218, 249,
        227, 194, 239, 119, 60, 216, 3, 73, 239, 101, 21, 29, 52, 144,
    ];
    let private_key: ManagedBuffer<StaticApi> = ManagedBuffer::from(private_key_bytes);

    let mut message_bytes = vec![
        251, 147, 239, 126, 223, 35, 81, 254, 30, 151, 35, 164, 29, 15, 101, 26, 54, 140, 55, 49,
        179, 45, 58, 84, 91, 235, 25, 6, 138, 32, 231, 241, 113, 180, 40, 135, 112, 10, 164, 143,
        26, 237, 140, 207, 194, 3, 162, 251, 137, 86, 65, 46, 178, 171, 112, 59, 180, 55, 241, 45,
        123, 56, 3, 8, 54, 223, 74, 165, 196, 1, 59, 26, 218, 215, 177, 178, 234, 18, 97, 42, 157,
        72, 114, 32, 3, 101, 157, 201, 147, 80, 79, 67, 114, 124, 166, 251, 230, 194, 61, 121,
    ];
    let mut message: ManagedBuffer<StaticApi> = ManagedBuffer::from(message_bytes.clone());

    let signature_bytes = vec![
        83, 96, 73, 13, 12, 227, 27, 74, 135, 228, 150, 120, 31, 36, 229, 204, 150, 30, 3, 120,
        163, 123, 228, 179, 194, 142, 241, 116, 29, 170, 98, 200, 73, 194, 70, 6, 212, 20, 158, 57,
        5, 7, 113, 197, 110, 9, 228, 185, 193, 25, 46, 54, 246, 145, 20, 112, 197, 92, 80, 175,
        193, 162, 255, 248,
    ];
    let signature: ManagedBuffer<StaticApi> = ManagedBuffer::from(signature_bytes);

    interact
        .verify_secp256r1_signature(&private_key, &message, &signature, None)
        .await;

    message_bytes[0] += 1;
    message = ManagedBuffer::from(message_bytes.clone());
    interact
        .verify_secp256r1_signature(
            &private_key,
            &message,
            &signature,
            Some("signature verification failed"),
        )
        .await;
}

async fn verify_bls_signature(interact: &mut BasicFeaturesInteract) {
    let private_key_bytes = vec![
        62, 136, 106, 76, 110, 16, 154, 21, 31, 65, 5, 174, 230, 90, 81, 146, 209, 80, 239, 31,
        166, 141, 60, 215, 105, 100, 160, 176, 134, 0, 109, 190, 67, 36, 201, 137, 222, 176, 228,
        65, 108, 109, 103, 6, 219, 27, 25, 16, 235, 39, 50, 240, 136, 66, 251, 72, 134, 6, 123,
        158, 209, 145, 16, 154, 194, 24, 141, 118, 0, 45, 46, 17, 218, 128, 163, 240, 234, 137,
        254, 230, 181, 156, 131, 76, 196, 120, 166, 189, 73, 203, 138, 25, 59, 26, 187, 22,
    ];
    let private_key: ManagedBuffer<StaticApi> = ManagedBuffer::from(private_key_bytes);

    let mut message_bytes = vec![
        233, 107, 208, 243, 107, 112, 197, 204, 192, 196, 57, 99, 67, 189, 125, 130, 85, 184, 165,
        38, 197, 95, 161, 226, 24, 81, 31, 175, 230, 83, 155, 142,
    ];
    let mut message: ManagedBuffer<StaticApi> = ManagedBuffer::from(message_bytes.clone());

    let signature_bytes = vec![
        4, 114, 93, 177, 149, 227, 122, 162, 55, 205, 187, 218, 118, 39, 13, 74, 34, 155, 110, 122,
        54, 81, 16, 77, 197, 140, 67, 73, 192, 56, 142, 133, 70, 151, 111, 229, 74, 4, 36, 5, 48,
        185, 144, 100, 228, 52, 201, 15,
    ];
    let signature: ManagedBuffer<StaticApi> = ManagedBuffer::from(signature_bytes);

    interact
        .verify_bls_signature_share(&private_key, &message, &signature, None)
        .await;

    message_bytes[0] += 1;
    message = ManagedBuffer::from(message_bytes.clone());
    interact
        .verify_bls_signature_share(
            &private_key,
            &message,
            &signature,
            Some("signature is invalid"),
        )
        .await;
}

async fn verify_bls_aggregated_signature(interact: &mut BasicFeaturesInteract) {
    let private_key_0_bytes = vec![
        151, 35, 187, 5, 78, 140, 121, 239, 24, 220, 36, 211, 41, 248, 76, 126, 109, 189, 67, 238,
        26, 16, 100, 241, 247, 236, 175, 152, 190, 86, 149, 177, 166, 44, 120, 181, 48, 207, 236,
        182, 147, 4, 240, 124, 239, 183, 107, 2, 205, 174, 214, 60, 178, 246, 34, 20, 151, 17, 116,
        246, 3, 112, 66, 18, 214, 144, 245, 239, 118, 241, 113, 142, 193, 233, 32, 176, 10, 192,
        121, 41, 73, 217, 247, 55, 27, 188, 92, 158, 5, 79, 4, 7, 117, 238, 157, 6,
    ];
    let private_key_1_bytes = vec![
        100, 2, 223, 146, 202, 215, 201, 240, 251, 6, 56, 31, 102, 148, 2, 102, 25, 60, 134, 91,
        166, 233, 15, 8, 173, 188, 204, 80, 73, 19, 212, 184, 0, 91, 116, 179, 33, 14, 56, 186,
        100, 79, 65, 184, 224, 175, 21, 25, 201, 1, 55, 145, 170, 167, 152, 221, 25, 83, 110, 61,
        222, 241, 249, 196, 154, 131, 186, 176, 82, 21, 3, 249, 174, 223, 16, 92, 243, 42, 244, 33,
        207, 65, 247, 126, 167, 210, 109, 180, 101, 10, 135, 173, 1, 120, 243, 135,
    ];
    let private_key_2_bytes = vec![
        167, 189, 112, 217, 238, 180, 236, 11, 175, 248, 112, 51, 92, 109, 165, 146, 203, 119, 170,
        30, 253, 74, 11, 20, 14, 95, 38, 58, 123, 163, 70, 71, 74, 162, 181, 219, 44, 64, 123, 71,
        53, 79, 235, 252, 139, 193, 171, 24, 21, 124, 232, 217, 165, 90, 173, 243, 126, 28, 74,
        228, 196, 215, 177, 174, 142, 4, 152, 197, 32, 174, 189, 46, 250, 195, 44, 168, 34, 103,
        194, 79, 243, 19, 32, 6, 209, 74, 229, 20, 40, 37, 18, 147, 91, 248, 26, 6,
    ];
    let private_key_3_bytes = vec![
        64, 142, 232, 235, 197, 38, 149, 153, 201, 236, 175, 204, 230, 215, 135, 111, 95, 199, 187,
        227, 232, 108, 240, 191, 161, 29, 52, 223, 145, 198, 116, 81, 223, 114, 117, 174, 142, 57,
        157, 52, 221, 66, 215, 23, 47, 184, 244, 22, 5, 225, 104, 128, 73, 126, 18, 56, 226, 224,
        208, 133, 92, 51, 31, 91, 66, 52, 121, 132, 182, 218, 54, 200, 129, 159, 19, 254, 199, 166,
        163, 160, 182, 165, 90, 91, 38, 159, 25, 184, 5, 134, 56, 31, 206, 223, 242, 151,
    ];
    let private_key_4_bytes = vec![
        225, 63, 17, 70, 29, 14, 17, 247, 141, 237, 214, 202, 191, 180, 17, 69, 22, 51, 143, 3,
        126, 28, 248, 18, 27, 200, 66, 231, 77, 67, 74, 27, 114, 136, 85, 161, 82, 103, 245, 219,
        171, 126, 49, 161, 233, 3, 238, 9, 89, 86, 120, 23, 171, 116, 63, 91, 172, 87, 183, 130,
        225, 132, 201, 138, 85, 77, 9, 38, 89, 251, 114, 54, 191, 31, 81, 19, 164, 36, 170, 66, 98,
        86, 8, 206, 86, 70, 202, 224, 103, 225, 167, 101, 118, 231, 42, 1,
    ];
    let private_key: ManagedVec<StaticApi, ManagedBuffer<StaticApi>> = ManagedVec::from(vec![
        ManagedBuffer::from(private_key_0_bytes),
        ManagedBuffer::from(private_key_1_bytes),
        ManagedBuffer::from(private_key_2_bytes),
        ManagedBuffer::from(private_key_3_bytes),
        ManagedBuffer::from(private_key_4_bytes),
    ]);

    let mut message_bytes = vec![109, 101, 115, 115, 97, 103, 101, 48];
    let mut message: ManagedBuffer<StaticApi> = ManagedBuffer::from(message_bytes.clone());

    let signature_bytes = vec![
        129, 198, 17, 200, 234, 139, 166, 197, 249, 2, 7, 249, 0, 46, 67, 110, 156, 185, 126, 146,
        116, 130, 250, 117, 91, 70, 116, 157, 207, 141, 53, 28, 41, 117, 110, 52, 65, 126, 2, 70,
        135, 98, 156, 28, 240, 180, 236, 153,
    ];
    let signature: ManagedBuffer<StaticApi> = ManagedBuffer::from(signature_bytes);

    interact
        .verify_bls_aggregated_signature(private_key.clone(), &message, &signature, None)
        .await;

    message_bytes[0] += 1;
    message = ManagedBuffer::from(message_bytes.clone());
    interact
        .verify_bls_aggregated_signature(
            private_key,
            &message,
            &signature,
            Some("aggregate signature is invalid"),
        )
        .await;
}
