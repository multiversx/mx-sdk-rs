use super::VmApiImpl;
use elrond_wasm::{
    api::CryptoApi,
    types::{BoxedBytes, MessageHashType, H256},
    Box,
};

extern "C" {

    fn sha256(dataOffset: *const u8, length: i32, resultOffset: *mut u8) -> i32;

    fn keccak256(dataOffset: *const u8, length: i32, resultOffset: *mut u8) -> i32;

    fn ripemd160(dataOffset: *const u8, length: i32, resultOffset: *mut u8) -> i32;

    fn verifyBLS(
        keyOffset: *const u8,
        messageOffset: *const u8,
        messageLength: i32,
        sigOffset: *const u8,
    ) -> i32;

    fn verifyEd25519(
        keyOffset: *const u8,
        messageOffset: *const u8,
        messageLength: i32,
        sigOffset: *const u8,
    ) -> i32;

    fn verifySecp256k1(
        keyOffset: *const u8,
        keyLength: i32,
        messageOffset: *const u8,
        messageLength: i32,
        sigOffset: *const u8,
    ) -> i32;

    fn verifyCustomSecp256k1(
        keyOffset: *const u8,
        keyLength: i32,
        messageOffset: *const u8,
        messageLength: i32,
        sigOffset: *const u8,
        hashType: i32,
    ) -> i32;

    fn encodeSecp256k1DerSignature(
        rOffset: *const u8,
        rLength: i32,
        sOffset: *const u8,
        sLength: i32,
        sigOffset: *const u8,
    ) -> i32;

}

impl CryptoApi for VmApiImpl {
    fn sha256(&self, data: &[u8]) -> H256 {
        unsafe {
            let mut res = H256::zero();
            sha256(data.as_ptr(), data.len() as i32, res.as_mut_ptr());
            res
        }
    }

    fn keccak256(&self, data: &[u8]) -> H256 {
        unsafe {
            let mut res = H256::zero();
            keccak256(data.as_ptr(), data.len() as i32, res.as_mut_ptr());
            res
        }
    }

    fn ripemd160(&self, data: &[u8]) -> Box<[u8; 20]> {
        unsafe {
            let mut res = [0u8; 20];
            ripemd160(data.as_ptr(), data.len() as i32, res.as_mut_ptr());
            Box::new(res)
        }
    }

    // the verify functions return 0 if valid signature, -1 if invalid

    fn verify_bls(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool {
        unsafe {
            verifyBLS(
                key.as_ptr(),
                message.as_ptr(),
                message.len() as i32,
                signature.as_ptr(),
            ) == 0
        }
    }

    fn verify_ed25519(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool {
        unsafe {
            verifyEd25519(
                key.as_ptr(),
                message.as_ptr(),
                message.len() as i32,
                signature.as_ptr(),
            ) == 0
        }
    }

    fn verify_secp256k1(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool {
        unsafe {
            verifySecp256k1(
                key.as_ptr(),
                key.len() as i32,
                message.as_ptr(),
                message.len() as i32,
                signature.as_ptr(),
            ) == 0
        }
    }

    fn verify_custom_secp256k1(
        &self,
        key: &[u8],
        message: &[u8],
        signature: &[u8],
        hash_type: MessageHashType,
    ) -> bool {
        unsafe {
            verifyCustomSecp256k1(
                key.as_ptr(),
                key.len() as i32,
                message.as_ptr(),
                message.len() as i32,
                signature.as_ptr(),
                hash_type.as_u8() as i32,
            ) == 0
        }
    }

    fn encode_secp256k1_der_signature(&self, r: &[u8], s: &[u8]) -> BoxedBytes {
        unsafe {
            // 3 for "magic" numbers in the signature + 3 for lengths: total_sig_length, r_length, s_length
            let mut sig_length = 6 + r.len() + s.len();
            let mask = 0x80;

            // 1 additional zero-byte is added for r and s if they could be misinterpreted as a negative number
            if r[0] & mask != 0 {
                sig_length += 1;
            }
            if s[0] & mask != 0 {
                sig_length += 1;
            }

            let mut sig_output = BoxedBytes::allocate(sig_length);

            encodeSecp256k1DerSignature(
                r.as_ptr(),
                r.len() as i32,
                s.as_ptr(),
                s.len() as i32,
                sig_output.as_mut_ptr(),
            );

            sig_output
        }
    }
}
