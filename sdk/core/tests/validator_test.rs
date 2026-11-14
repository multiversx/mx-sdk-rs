use multiversx_sdk::validator::Validator;

/// Two test validator keys, in a single validators PEM (same keys as in `tools/interactor-delegation-func-calls`).
const VALIDATORS_PEM: &str = "-----BEGIN PRIVATE KEY for 62b55cc67494ea9488e4406a2ebd52ffb2f8d58ea08eb37c15a3b5c7398f9b4209f9fa8b93c9d09e6f5a2aef06adc20d412c4ebbaccbc88556f2d05ff519243c1411a4847efb45887d2b6c7ae0e304da006f7d3565d9aa505fe7b2e170cdaf09-----
YjEzNDA2YjM1YTY4ODlkNWM4M2IwMDliZDFmNTA4N2FlMjBjZTM1ZDMwZDFkZGJj
MjVkMTc0YjQ2NTE4MzMyYw==
-----END PRIVATE KEY for 62b55cc67494ea9488e4406a2ebd52ffb2f8d58ea08eb37c15a3b5c7398f9b4209f9fa8b93c9d09e6f5a2aef06adc20d412c4ebbaccbc88556f2d05ff519243c1411a4847efb45887d2b6c7ae0e304da006f7d3565d9aa505fe7b2e170cdaf09-----
-----BEGIN PRIVATE KEY for 69ee4118e974b4dedaed1b49f630eeaac9bce10993b75f1be8d64594034ae6333394f1de4cacada14f3ee28582a8950b791bd0de492ba8737e8cfc0a66e8d0fb77763c2db5cced46b99a7f094a977908e34f0cc66044ae1b69505aacbfb17589-----
NjgyMGEzMTU3ZTk4NmQxMGViY2EzMDYwZTYzN2YzZWQxNzVmMTQ5MjlhNzBhZTgy
ZGI2ZjJjMzk1NDkwNGExMA==
-----END PRIVATE KEY for 69ee4118e974b4dedaed1b49f630eeaac9bce10993b75f1be8d64594034ae6333394f1de4cacada14f3ee28582a8950b791bd0de492ba8737e8cfc0a66e8d0fb77763c2db5cced46b99a7f094a977908e34f0cc66044ae1b69505aacbfb17589-----
";

const PUBLIC_KEY_1: &str = "62b55cc67494ea9488e4406a2ebd52ffb2f8d58ea08eb37c15a3b5c7398f9b4209f9fa8b93c9d09e6f5a2aef06adc20d412c4ebbaccbc88556f2d05ff519243c1411a4847efb45887d2b6c7ae0e304da006f7d3565d9aa505fe7b2e170cdaf09";
const PUBLIC_KEY_2: &str = "69ee4118e974b4dedaed1b49f630eeaac9bce10993b75f1be8d64594034ae6333394f1de4cacada14f3ee28582a8950b791bd0de492ba8737e8cfc0a66e8d0fb77763c2db5cced46b99a7f094a977908e34f0cc66044ae1b69505aacbfb17589";

#[test]
fn validator_pem_load_all() {
    let validators = Validator::from_pem_file_contents_all(VALIDATORS_PEM).unwrap();
    assert_eq!(validators.len(), 2);
    assert_eq!(hex::encode(validators[0].public_key.to_vec()), PUBLIC_KEY_1);
    assert_eq!(hex::encode(validators[1].public_key.to_vec()), PUBLIC_KEY_2);
    assert_eq!(
        hex::encode(validators[0].secret_key_bytes().unwrap()),
        "b13406b35a6889d5c83b009bd1f5087ae20ce35d30d1ddbc25d174b46518332c"
    );

    // the single-key loader returns the first key
    let first = Validator::from_pem_file_contents(VALIDATORS_PEM.to_string()).unwrap();
    assert_eq!(first.public_key, validators[0].public_key);
}

#[test]
fn validator_pem_load_empty() {
    assert!(Validator::from_pem_file_contents_all("").is_err());
}

#[cfg(feature = "bls")]
mod bls {
    use super::*;
    use multiversx_bls::{G1, G2};

    /// The signature verifies against the public key, for the signed message only.
    #[test]
    fn validator_sign_and_verify() {
        let validators = Validator::from_pem_file_contents_all(VALIDATORS_PEM).unwrap();
        let message = [7u8; 32]; // e.g. the staker address

        for validator in &validators {
            let signature = validator.sign(&message).unwrap();
            let sig = G1::from_serialized(&signature.to_vec()).unwrap();
            let pk = G2::from_serialized(&validator.public_key.to_vec()).unwrap();
            assert!(sig.verify(pk, &message));
            assert!(!sig.verify(pk, &[8u8; 32]));
        }
    }

    /// A secret key that does not match the public key in the label is rejected.
    #[test]
    fn validator_sign_mismatched_key() {
        let mut validators = Validator::from_pem_file_contents_all(VALIDATORS_PEM).unwrap();
        validators[0].public_key = validators[1].public_key;
        assert!(validators[0].sign(&[7u8; 32]).is_err());
    }
}
