use hex::FromHex;
use multiversx_chain_vm::crypto_functions;

#[test]
fn test_verify_ed25519_basic() {
    let public_key: &[u8] = b"832c7d42d05a69647887961552c172b5dd3da1b253ca823d96df2576bf218c61";
    let message: &[u8] = b"033085998ec7c0fa980ed924ab6ea6bf465053d896040aa0f9887ccbca7ecdac";
    let signature: &[u8] = b"40e03623649f6929908a1051d99ef95907f405ce4c04c99598e67373d2e88be03c59ce36d767255e290b91bdd174e71803a222b74d8c138b4650f1e70c26930c";

    let pub_bytes: Vec<u8> = FromHex::from_hex(public_key).unwrap();
    let msg_bytes: Vec<u8> = FromHex::from_hex(message).unwrap();
    let sig_bytes: Vec<u8> = FromHex::from_hex(signature).unwrap();

    let success = crypto_functions::verify_ed25519(&pub_bytes, &msg_bytes, &sig_bytes);
    assert!(success);
}

#[test]
fn test_verify_ed25519_bad_sig() {
    let public_key: &[u8] = b"832c7d42d05a69647887961552c172b5dd3da1b253ca823d96df2576bf218c61";
    let message: &[u8] = b"033085998ec7c0fa980ed924ab6ea6bf465053d896040aa0f9887ccbca7ecdac";
    let signature: &[u8] = b"40e03623649f6929908a1051d99ef95907f405ce4c04c99598e67373d2e88be03c59ce36d767255e290b91bdd174e71803a222b74d8c138b4650f1e70c26930d";

    let pub_bytes: Vec<u8> = FromHex::from_hex(public_key).unwrap();
    let msg_bytes: Vec<u8> = FromHex::from_hex(message).unwrap();
    let sig_bytes: Vec<u8> = FromHex::from_hex(signature).unwrap();

    let success = crypto_functions::verify_ed25519(&pub_bytes, &msg_bytes, &sig_bytes);
    assert!(!success);
}

#[test]
fn test_verify_ed25519_invalid_args() {
    let public_key: &[u8] = b"832c7d42d05a69647887961552c172b5dd3da1b253ca823d96df2576bf218c";
    let message: &[u8] = b"033085998ec7c0fa980ed924ab6ea6bf465053d896040aa0f9887ccbca7ecd";
    let signature: &[u8] = b"40e03623649f6929908a1051d99ef95907f405ce4c04c99598e67373d2e88be03c59ce36d767255e290b91bdd174e71803a222b74d8c138b4650f1e70c2693";

    let pub_bytes: Vec<u8> = FromHex::from_hex(public_key).unwrap();
    let msg_bytes: Vec<u8> = FromHex::from_hex(message).unwrap();
    let sig_bytes: Vec<u8> = FromHex::from_hex(signature).unwrap();

    let success = crypto_functions::verify_ed25519(&pub_bytes, &msg_bytes, &sig_bytes);
    assert!(!success);
}

#[test]
fn test_verify_bls_signature_ok_1() {
    let public_key = b"b5823f6e564251cc03ce7bad3da83e72576e92795d3500bba1acb30ec9a94dce87bb8aa794d67b2d61d15c33f28f6c0c23ba1dfcbf21e8f8b46286ff871afabac925303ddcaddce6254fcff6d3155797db40b3d3b5865e8fc0bd770b3d79b381";
    let message = b"6d65737361676520746f206265207369676e6564";
    let signature = b"af32a2ddf341c08d1eb7232f05dc34e4454155e676b58c40fddf9a036562ac2c01533d2d557cb49d73aa9d7a89744696";

    let pk_bytes: Vec<u8> = FromHex::from_hex(public_key).unwrap();
    let msg_bytes: Vec<u8> = FromHex::from_hex(message).unwrap();
    let sig_bytes: Vec<u8> = FromHex::from_hex(signature).unwrap();

    let success = crypto_functions::verify_bls(&pk_bytes, &msg_bytes, &sig_bytes);

    assert!(success);
}

#[test]
fn test_verify_bls_signature_ok_2() {
    let public_key = b"4b8aafd2f7421817df7a372e5eda8dac113e38d3974e7eb96a942e9cc6940c3bac2ccf9cf66576153d3b6fffc2201a08812ee1b6d47231d7e2883352ceec89f17ff29b35ae9b1d935fdbf69deac2920907dae0018e63189dea30d8016f710102";
    let message = b"6d65737361676520746f206265207369676e6564";
    let signature= b"6564590f65d4156a970b7758c415a99d039afaf0d80e6e04639fc315ebfa80486599226cb9515b726fd3045248687002";

    let pk_bytes: Vec<u8> = FromHex::from_hex(public_key).unwrap();
    let msg_bytes: Vec<u8> = FromHex::from_hex(message).unwrap();
    let sig_bytes: Vec<u8> = FromHex::from_hex(signature).unwrap();

    let success = crypto_functions::verify_bls(&pk_bytes, &msg_bytes, &sig_bytes);

    assert!(success);
}

#[test]
fn test_bls_signer_verify_empty_pk_err() {
    let message = b"6d65737361676520746f206265207369676e6564";
    let signature = b"6564590f65d4156a970b7758c415a99d039afaf0d80e6e04639fc315ebfa80486599226cb9515b726fd3045248687002";

    let msg_bytes: Vec<u8> = FromHex::from_hex(message).unwrap();
    let sig_bytes: Vec<u8> = FromHex::from_hex(signature).unwrap();

    let success = crypto_functions::verify_bls(&[], &msg_bytes, &sig_bytes);

    assert!(!success);
}

#[test]
fn test_bls_signer_verify_empty_message_err() {
    let public_key= b"494a592c78795857a8cb71537fc3508839ab22f18cc61b2c83ae33e5adde2d34b304b6183116281a7f558dc6d758c00979da47633ad62414ff967f94158558e2e346bf6c60c3e6d2525450bf82a86c578b8050e21073d94ad7f41ade8855da0b";
    let signature = b"cb614cd8dd40d1ec746c7e328087a894948ed291b8943bc97ae61cf84524c79a967342f307c88129915993d38aa00699";

    let pk_bytes: Vec<u8> = FromHex::from_hex(public_key).unwrap();
    let sig_bytes: Vec<u8> = FromHex::from_hex(signature).unwrap();

    let success = crypto_functions::verify_bls(&pk_bytes, &[], &sig_bytes);

    assert!(!success);
}

#[test]
fn test_bls_signer_verify_empty_signature_err() {
    let public_key = b"494a592c78795857a8cb71537fc3508839ab22f18cc61b2c83ae33e5adde2d34b304b6183116281a7f558dc6d758c00979da47633ad62414ff967f94158558e2e346bf6c60c3e6d2525450bf82a86c578b8050e21073d94ad7f41ade8855da0b";
    let message = b"6d65737361676520746f206265207369676e6564";

    let pk_bytes: Vec<u8> = FromHex::from_hex(public_key).unwrap();
    let msg_bytes: Vec<u8> = FromHex::from_hex(message).unwrap();

    let success = crypto_functions::verify_bls(&pk_bytes, &msg_bytes, &[]);

    assert!(!success);
}

#[test]
/// Verifies that BLS signature validation fails when the message differs from the one originally signed.
fn test_bls_verify_wrong_message() {
    let public_key = b"b989e7b7f46cf6eea635361c28a8a04cf0966b5e95e21d0507ead4b8f86a21b4050b885915f5e6719a37cf34bf0092035fa4e72fc5ac8e84366de2e4fed7121ab19d83629ff1254adfad79a8811b7c13452e713907a4ec90ff59c8d81ea6f70f";
    // originally signed message: "6d65737361676520746f206265207369676e6564" ("message to be signed")
    let message = b"0065737361676520746f206265207369676e6564";
    let signature = b"d62f00fceb2bb96c112a15e1f417d8d6c387085d14352098f58ae6e2bcf40a77b25420d9b6d1cb9982ae5e436df69189";

    let pk_bytes: Vec<u8> = FromHex::from_hex(public_key).unwrap();
    let msg_bytes: Vec<u8> = FromHex::from_hex(message).unwrap();
    let sig_bytes: Vec<u8> = FromHex::from_hex(signature).unwrap();

    let success = crypto_functions::verify_bls(&pk_bytes, &msg_bytes, &sig_bytes);

    assert!(!success);
}

#[test]
fn test_verify_bls_aggregated_signature_ok_1() {
    let message = b"message";

    let public_key_1 = b"51aca422768434d408cecbba4a559313928299622dfa7cbc3179c018db9ecac9b83cac0fe39b9bcbeb8017ca54c47d02c1ac1f0be7089dd94a755613d27d66d4d2d37bb6f42e8edc30f51152e6abe5feb032e282e100fb9b7aa66bfd71c9c486";
    let public_key_2 = b"2295cee09a2a258f56e5dadfd7600674a6d5e8e1570f5ba091d630d5d76769de4ed44cbfd2a519184c7c3f88ef2910099fea79038396e5edb346040f061081b230b1e5743c11eb3e17b7e38569a7055198ce5e8d40e4ba6a839e12f782062301";
    let public_key_3 = b"d1c10e8a448f8f8e900234b668ca4b4d2e84a1233bef0dbdfcdd5163e1f34dde7c59559acbbd753d73dbce182181cc0f1d6c3aa5ea58ab27514553f370e9b750198a61b1650a97a6f8352004576cb5cd51b8d36b62b7856cff43dfc5777f4299";
    let sig = b"351b11c424587709b703227fb3252562af696c15fd46b5a45cbc0d3aaa5407eccaa3436a57a3ca691bf12f82df9a0090";

    let pk1_bytes: Vec<u8> = FromHex::from_hex(public_key_1).unwrap();
    let pk2_bytes: Vec<u8> = FromHex::from_hex(public_key_2).unwrap();
    let pk3_bytes: Vec<u8> = FromHex::from_hex(public_key_3).unwrap();
    let sig_bytes: Vec<u8> = FromHex::from_hex(sig).unwrap();

    let success = crypto_functions::verify_bls_aggregated_signature(
        vec![pk1_bytes, pk2_bytes, pk3_bytes],
        message,
        &sig_bytes,
    );

    assert!(success)
}

#[test]
fn test_verify_bls_aggregated_signature_ok_2() {
    let message = b"message to be signed";

    let public_key_1 = b"79b942c7369ff529a657688ca802c5a75d3e520d4da8f26191d434408229c825265e38ddfd86138578b27f46af4b630b3dccd2a6f2cf077389e087aded73b1f13063cf30c206a23d84d01723c6ad9ffbaeed072bcfca433629164a63a41f858c";
    let public_key_2 = b"557c0ba5a6484df8bddec070e8502e6bf7afb18724d2ea115f3624639764749814e7236fa1877e70cf4fcaaacda9411039b7577bd0a2c7a30b7e19ab73fcca0f6fe22d839ffcd8fad0efb0be7d81783079de601ac0d368da4cd1ad1e81a28a03";
    let public_key_3 = b"30c651f679d7811875dfb4f937cff9c45ff7c299d7f94f4456fb955b6265b32d24b5a74f535e3231d3eb167bc792ef0eb06eda8d44b6d3bb6b44a644ff32fcefc8d72f0ba56b16bbbbd22b1696858ffb9f3e782c209d2d7980d7b2e177584e8d";
    let public_key_4 = b"0faab0db00303da011e3186c931f0d300ddb104da47145b6b4cae06c28e69aee9e249c05f055de88426c0d2611c8a9095b0fa38df48085a9d22d5a3358fa96cb57c467229f52552ac8f13f4de59ccf61035061c19986bd3ea35f54f675ea6898";
    let sig = b"6ff8ab2a3688731886342b00e1499f9c6bc3407d6d79b9248c597caa028e91a1548e540d66c88e633424139617d57992";

    let pk1_bytes: Vec<u8> = FromHex::from_hex(public_key_1).unwrap();
    let pk2_bytes: Vec<u8> = FromHex::from_hex(public_key_2).unwrap();
    let pk3_bytes: Vec<u8> = FromHex::from_hex(public_key_3).unwrap();
    let pk4_bytes: Vec<u8> = FromHex::from_hex(public_key_4).unwrap();
    let sig_bytes: Vec<u8> = FromHex::from_hex(sig).unwrap();

    let success = crypto_functions::verify_bls_aggregated_signature(
        vec![pk1_bytes, pk2_bytes, pk3_bytes, pk4_bytes],
        message,
        &sig_bytes,
    );

    assert!(success)
}

#[test]
fn test_verify_bls_aggregated_signature_bad_sig() {
    let message = b"hello";

    let public_key_1 = b"82eb2ddfa71f1673fbfbd17952838cbca3816d5e60bf5cdb220d8cad6cb800e2ed18bb747ef45b17c9b8cbc971c6b980";
    let public_key_2 = b"a81795a7afa09274717a170d6ba42ab06b65b25c7887eca7be46dfddae4e5b1a249f104b15551a7a445cccac9b403926";
    let public_key_3 = b"8bf9e68f8fc54d8cb808ba43f0ada562cafa3c07448ab038eff6f579f1e4c1d497a957f50f6eca2608f36c39d874cbea";

    let pk1_bytes: Vec<u8> = FromHex::from_hex(public_key_1).unwrap();
    let pk2_bytes: Vec<u8> = FromHex::from_hex(public_key_2).unwrap();
    let pk3_bytes: Vec<u8> = FromHex::from_hex(public_key_3).unwrap();

    let sig1 = b"979b87882bd59dd97d860c99f9c4295e7d63e3fede1823b942d31d71ea3707d8c179ab733d38f7497b53bfa1535fe5e202f2a1c6e4df1dbc97dbe315dccd51676dbef31af1fe60d4b11c304db61913dc1d39e929f80f2cd10b72cbc661235048";

    let sig1_bytes: Vec<u8> = FromHex::from_hex(sig1).unwrap();

    let success = crypto_functions::verify_bls_aggregated_signature(
        vec![pk1_bytes, pk2_bytes, pk3_bytes],
        message,
        &sig1_bytes,
    );

    assert!(!success)
}

#[test]
fn test_verify_bls_aggregated_signature_empty_pk_err() {
    let message = b"message";

    let sig = b"351b11c424587709b703227fb3252562af696c15fd46b5a45cbc0d3aaa5407eccaa3436a57a3ca691bf12f82df9a0090";
    let sig_bytes: Vec<u8> = FromHex::from_hex(sig).unwrap();

    let success = crypto_functions::verify_bls_aggregated_signature(vec![], message, &sig_bytes);

    assert!(!success)
}

#[test]
fn test_verify_bls_aggregated_signature_empty_msg() {
    let public_key_1 = b"79b942c7369ff529a657688ca802c5a75d3e520d4da8f26191d434408229c825265e38ddfd86138578b27f46af4b630b3dccd2a6f2cf077389e087aded73b1f13063cf30c206a23d84d01723c6ad9ffbaeed072bcfca433629164a63a41f858c";
    let public_key_2 = b"557c0ba5a6484df8bddec070e8502e6bf7afb18724d2ea115f3624639764749814e7236fa1877e70cf4fcaaacda9411039b7577bd0a2c7a30b7e19ab73fcca0f6fe22d839ffcd8fad0efb0be7d81783079de601ac0d368da4cd1ad1e81a28a03";
    let sig = b"6ff8ab2a3688731886342b00e1499f9c6bc3407d6d79b9248c597caa028e91a1548e540d66c88e633424139617d57992";

    let pk1_bytes: Vec<u8> = FromHex::from_hex(public_key_1).unwrap();
    let pk2_bytes: Vec<u8> = FromHex::from_hex(public_key_2).unwrap();
    let sig_bytes: Vec<u8> = FromHex::from_hex(sig).unwrap();

    let success = crypto_functions::verify_bls_aggregated_signature(
        vec![pk1_bytes, pk2_bytes],
        &[],
        &sig_bytes,
    );

    assert!(!success)
}

#[test]
fn test_verify_bls_aggregated_signature_empty_sig() {
    let message = b"message";

    let public_key_1 = b"79b942c7369ff529a657688ca802c5a75d3e520d4da8f26191d434408229c825265e38ddfd86138578b27f46af4b630b3dccd2a6f2cf077389e087aded73b1f13063cf30c206a23d84d01723c6ad9ffbaeed072bcfca433629164a63a41f858c";
    let public_key_2 = b"557c0ba5a6484df8bddec070e8502e6bf7afb18724d2ea115f3624639764749814e7236fa1877e70cf4fcaaacda9411039b7577bd0a2c7a30b7e19ab73fcca0f6fe22d839ffcd8fad0efb0be7d81783079de601ac0d368da4cd1ad1e81a28a03";

    let pk1_bytes: Vec<u8> = FromHex::from_hex(public_key_1).unwrap();
    let pk2_bytes: Vec<u8> = FromHex::from_hex(public_key_2).unwrap();

    let success =
        crypto_functions::verify_bls_aggregated_signature(vec![pk1_bytes, pk2_bytes], message, &[]);

    assert!(!success)
}

#[test]
// Verifies that BLS signature validation fails when the message differs from the one originally signed.
fn test_verify_bls_aggregated_signature_wrong_message() {
    let message = b"to be signed";

    let public_key_1 = b"b96174b5e710cd2f9a3d4515efca89fea9d3276b1d8ad0409980885ea5663d34405156011f0abf1560d1c281e70127195a6d63633906cbfa13e7acdb0d221918a05233104801046f0dcea859986be037f534bf5cd5470f96a10b7d8ea276538e";
    let public_key_2 = b"f70583d6334585a126f7d72baebd29f4470ab950d69284926715e9f12b2efc7aab97bfe0cf29ea149926fc57b50f5d18b79edb4acb8b0f18b32e7d45bd04592cc718f7d58486a3ae29a20a5f8b87f18a150c88d885e2000466a8f4d1b8b42d04";
    // originally signed message: "message to be signed"
    let sig = b"3e49288d1b8efe857fd9cb06fceaf782406e113ec15f1b3255b756ec8493913b5d144aa481d661f309b926062f962794";

    let pk1_bytes: Vec<u8> = FromHex::from_hex(public_key_1).unwrap();
    let pk2_bytes: Vec<u8> = FromHex::from_hex(public_key_2).unwrap();
    let sig_bytes: Vec<u8> = FromHex::from_hex(sig).unwrap();

    let success = crypto_functions::verify_bls_aggregated_signature(
        vec![pk1_bytes, pk2_bytes],
        message,
        &sig_bytes,
    );

    assert!(!success)
}
