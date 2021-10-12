use crate::DebugApi;
use elrond_wasm::{
    api::CryptoApi,
    types::{BoxedBytes, MessageHashType, H256},
};
use sha2::Sha256;
use sha3::{Digest, Keccak256};

impl CryptoApi for DebugApi {
    fn sha256(&self, data: &[u8]) -> H256 {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let hash: [u8; 32] = hasher.finalize().into();
        hash.into()
    }

    fn keccak256(&self, data: &[u8]) -> H256 {
        let mut hasher = Keccak256::new();
        hasher.update(data);
        let hash: [u8; 32] = hasher.finalize().into();
        hash.into()
    }

    fn ripemd160(&self, _data: &[u8]) -> Box<[u8; 20]> {
        panic!("ripemd160 not implemented yet!")
    }

    fn verify_bls(&self, _key: &[u8], _message: &[u8], _signature: &[u8]) -> bool {
        panic!("verify_bls not implemented yet!")
    }

    fn verify_ed25519(&self, _key: &[u8], _message: &[u8], _signature: &[u8]) -> bool {
        panic!("verify_ed25519 not implemented yet!")
    }

    fn verify_secp256k1(&self, _key: &[u8], _message: &[u8], _signature: &[u8]) -> bool {
        panic!("verify_secp256k1 not implemented yet!")
    }

    fn verify_custom_secp256k1(
        &self,
        _key: &[u8],
        _message: &[u8],
        _signature: &[u8],
        _hash_type: MessageHashType,
    ) -> bool {
        panic!("verify_custom_secp256k1 not implemented yet!")
    }

    fn encode_secp256k1_der_signature(&self, _r: &[u8], _s: &[u8]) -> BoxedBytes {
        panic!("encode_secp256k1_signature not implemented yet!")
    }
}
