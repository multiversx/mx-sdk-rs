use super::big_int_api_mock::*;
use super::big_uint_api_mock::*;
use crate::TxContext;
use elrond_wasm::api::CryptoApi;
use elrond_wasm::types::{BoxedBytes, EllipticCurve, H256};
use sha2::Sha256;
use sha3::{Digest, Keccak256};

impl CryptoApi<RustBigInt, RustBigUint> for TxContext {
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

	fn verify_bls(&self, _key: &[u8], _message: &[u8], _signature: &[u8]) -> bool {
		panic!("verify_bls not implemented yet!")
	}

	fn verify_ed25519(&self, _key: &[u8], _message: &[u8], _signature: &[u8]) -> bool {
		panic!("verify_ed25519 not implemented yet!")
	}

	fn verify_secp256k1(&self, _key: &[u8], _message: &[u8], _signature: &[u8]) -> bool {
		panic!("verify_secp256k1 not implemented yet!")
	}

	fn add_ec(
		&self,
		curve: &EllipticCurve<RustBigUint>,
		x_first_point: RustBigUint,
		y_first_point: RustBigUint,
		x_second_point: RustBigUint,
		y_second_point: RustBigUint,
	) -> (RustBigUint, RustBigUint) {
		panic!("add_ec not implemented yet!")
	}

	fn double_ec(
		&self,
		curve: &EllipticCurve<RustBigUint>,
		x_point: RustBigUint,
		y_point: RustBigUint,
	) -> (RustBigUint, RustBigUint) {
		panic!("double_ec not implemented yet!")
	}

	fn is_on_curve_ec(
		&self,
		curve: &EllipticCurve<RustBigUint>,
		x_point: RustBigUint,
		y_point: RustBigUint,
	) -> bool {
		panic!("is_on_curve_ec not implemented yet!")
	}

	fn scalar_mult(
		&self,
		curve: &EllipticCurve<RustBigUint>,
		x_point: RustBigUint,
		y_point: RustBigUint,
		data: BoxedBytes,
	) -> (RustBigUint, RustBigUint) {
		panic!("scalar_mult not implemented yet!")
	}

	fn scalar_base_mult(
		&self,
		curve: &EllipticCurve<RustBigUint>,
		data: BoxedBytes,
	) -> (RustBigUint, RustBigUint) {
		panic!("scalar_base_mult not implemented yet!")
	}

	fn marshal_ec(
		&self,
		curve: &EllipticCurve<RustBigUint>,
		x_pair: RustBigUint,
		y_pair: RustBigUint,
	) -> BoxedBytes {
		panic!("marshal_ec not implemented yet!")
	}

	fn marshal_compressed_ec(
		&self,
		curve: &EllipticCurve<RustBigUint>,
		x_pair: RustBigUint,
		y_pair: RustBigUint,
	) -> BoxedBytes {
		panic!("marshal_compressed_ec not implemented yet!")
	}

	fn unmarshal_ec(
		&self,
		curve: &EllipticCurve<RustBigUint>,
		data: BoxedBytes,
	) -> (RustBigUint, RustBigUint) {
		panic!("unmarshal_ec not implemented yet!")
	}

	fn unmarshal_compressed_ec(
		&self,
		curve: &EllipticCurve<RustBigUint>,
		data: BoxedBytes,
	) -> (RustBigUint, RustBigUint) {
		panic!("unmarshal_compressed_ec not implemented yet!")
	}

	fn generate_key_ec(
		&self,
		curve: &EllipticCurve<RustBigUint>,
	) -> (RustBigUint, RustBigUint, BoxedBytes) {
		panic!("generate_key_ec not implemented yet!")
	}
}
