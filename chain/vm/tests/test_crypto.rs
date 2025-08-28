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
#[ignore]
fn test_verify_bls_aggregated_signature() {
    // let message = b"hello";

    // let pk1 = b"82eb2ddfa71f1673fbfbd17952838cbca3816d5e60bf5cdb220d8cad6cb800e2ed18bb747ef45b17c9b8cbc971c6b980";
    // let pk2 = b"a81795a7afa09274717a170d6ba42ab06b65b25c7887eca7be46dfddae4e5b1a249f104b15551a7a445cccac9b403926";
    // let pk3 = b"8bf9e68f8fc54d8cb808ba43f0ada562cafa3c07448ab038eff6f579f1e4c1d497a957f50f6eca2608f36c39d874cbea";

    // let pk1_bytes: Vec<u8> = FromHex::from_hex(pk1).unwrap();
    // let pk2_bytes: Vec<u8> = FromHex::from_hex(pk2).unwrap();
    // let pk3_bytes: Vec<u8> = FromHex::from_hex(pk3).unwrap();

    // let sig1 = b"979a97882bd59dd97d860c99f9c4295e7d63e3fede1823b942d31d71ea3707d8c179ab733d38f7497b53bfa1535fe5e202f2a1c6e4df1dbc97dbe315dccd51676dbef31af1fe60d4b11c304db61913dc1d39e929f80f2cd10b72cbc661235048";
    // let sig2 = b"a5a465bb34264faa5d695ad844cfbacc5514ef8cb50abdc874bbbe065e74fc2dcd69f22e6e3a25260c8a9b6c55d7ca6d0d42227a58385f7af440b85d447675bee433c88726c582ef66fba5357b60e6d980ce43e7c6a6929ed605dbe51d47c453";
    // let sig3 = b"9936262ea50315d00c2b082cae3a97315f8852a021de195cb256bf532d439f1965a65c9fa536693ecb7a238611cb9ac30926d4958a7852955ff56aba9f60d9bda7ba10a939bd8b370bc42b667ee2f4fe4bb671482abc93bcac11ec10c0230229";

    // let sig1_bytes: Vec<u8> = FromHex::from_hex(sig1).unwrap();
    // let sig2_bytes: Vec<u8> = FromHex::from_hex(sig2).unwrap();
    // let sig3_bytes: Vec<u8> = FromHex::from_hex(sig3).unwrap();

    // let sig1_converted = Signature::from_bytes(&sig1_bytes).unwrap();
    // let sig2_converted = Signature::from_bytes(&sig2_bytes).unwrap();
    // let sig3_converted = Signature::from_bytes(&sig3_bytes).unwrap();

    // let mut agg_sig = AggregateSignature::from_signature(&sig1_converted);
    // agg_sig.add_signature(&sig2_converted, true).unwrap();
    // agg_sig.add_signature(&sig3_converted, true).unwrap();
    // let final_agg_sig = agg_sig.to_signature();

    // let success = crypto_functions::verify_bls_aggregated_signature(
    //     vec![pk1_bytes, pk2_bytes, pk3_bytes],
    //     message,
    //     &final_agg_sig.to_bytes(),
    // );

    // assert!(success)
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
#[should_panic(expected = "Failed to deserialize public key: []. Error: InvalidData")]
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
#[should_panic(expected = "Failed to deserialize signature: []. Error: InvalidData")]
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

// #[test]
// #[ignore]
// fn test_verify_bls_aggregated_signature_bad_sig() {
//     let message = b"hello";

//     let pk1 = b"82eb2ddfa71f1673fbfbd17952838cbca3816d5e60bf5cdb220d8cad6cb800e2ed18bb747ef45b17c9b8cbc971c6b980";
//     let pk2 = b"a81795a7afa09274717a170d6ba42ab06b65b25c7887eca7be46dfddae4e5b1a249f104b15551a7a445cccac9b403926";
//     let pk3 = b"8bf9e68f8fc54d8cb808ba43f0ada562cafa3c07448ab038eff6f579f1e4c1d497a957f50f6eca2608f36c39d874cbea";

//     let pk1_bytes: Vec<u8> = FromHex::from_hex(pk1).unwrap();
//     let pk2_bytes: Vec<u8> = FromHex::from_hex(pk2).unwrap();
//     let pk3_bytes: Vec<u8> = FromHex::from_hex(pk3).unwrap();

//     let sig1 = b"979b87882bd59dd97d860c99f9c4295e7d63e3fede1823b942d31d71ea3707d8c179ab733d38f7497b53bfa1535fe5e202f2a1c6e4df1dbc97dbe315dccd51676dbef31af1fe60d4b11c304db61913dc1d39e929f80f2cd10b72cbc661235048";
//     let sig2 = b"a5a465bb34264faa5d695ad844cfbacc5514ef8cb50abdc874bbbe065e74fc2dcd69f22e6e3a25260c8a9b6c55d7ca6d0d42227a58385f7af440b85d447675bee433c88726c582ef66fba5357b60e6d980ce43e7c6a6929ed605dbe51d47c453";
//     let sig3 = b"9936262ea50315d00c2b082cae3a97315f8852a021de195cb256bf532d439f1965a65c9fa536693ecb7a238611cb9ac30926d4958a7852955ff56aba9f60d9bda7ba10a939bd8b370bc42b667ee2f4fe4bb671482abc93bcac11ec10c0230229";

//     let sig1_bytes: Vec<u8> = FromHex::from_hex(sig1).unwrap();
//     let sig2_bytes: Vec<u8> = FromHex::from_hex(sig2).unwrap();
//     let sig3_bytes: Vec<u8> = FromHex::from_hex(sig3).unwrap();

//     let sig1_converted = Signature::from_bytes(&sig1_bytes).unwrap();
//     let sig2_converted = Signature::from_bytes(&sig2_bytes).unwrap();
//     let sig3_converted = Signature::from_bytes(&sig3_bytes).unwrap();

//     let mut agg_sig = AggregateSignature::from_signature(&sig1_converted);
//     agg_sig.add_signature(&sig2_converted, true).unwrap();
//     agg_sig.add_signature(&sig3_converted, true).unwrap();
//     let final_agg_sig = agg_sig.to_signature();

//     let success = crypto_functions::verify_bls_aggregated_signature(
//         vec![pk1_bytes, pk2_bytes, pk3_bytes],
//         message,
//         &final_agg_sig.to_bytes(),
//     );

//     assert!(!success)
// }
