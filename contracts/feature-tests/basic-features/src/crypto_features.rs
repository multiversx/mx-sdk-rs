elrond_wasm::imports!();

/// All crypto functions provided by Arwen exposed here.
#[elrond_wasm_derive::module]
pub trait CryptoFeatures {
	#[endpoint(computeSha256)]
	fn compute_sha256(&self, input: Vec<u8>) -> H256 {
		self.crypto().sha256(&input)
	}

	#[endpoint(computeKeccak256)]
	fn compute_keccak256(&self, input: Vec<u8>) -> H256 {
		self.crypto().keccak256(&input)
	}

	#[endpoint]
	fn verify_bls_signature(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool {
		self.crypto().verify_bls(key, message, signature)
	}

	#[endpoint]
	fn verify_ed25519_signature(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool {
		self.crypto().verify_ed25519(key, message, signature)
	}

	#[endpoint]
	fn verify_secp256k1_signature(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool {
		self.crypto().verify_secp256k1(key, message, signature)
	}
}
