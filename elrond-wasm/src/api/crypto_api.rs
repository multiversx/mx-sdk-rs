use super::BigUintApi;
use crate::types::{BoxedBytes, EllipticCurve, H256};

pub trait CryptoApi {
	/// Numeric type used in some of the Arwen hooks.
	type BigUint: BigUintApi + 'static;

	fn sha256(&self, data: &[u8]) -> H256;

	fn keccak256(&self, data: &[u8]) -> H256;

	fn verify_bls(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool;

	fn verify_ed25519(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool;

	/// Note: the signature is minimum 2 bytes in length,
	/// the second byte encodes the length of the remaining signature bytes.
	fn verify_secp256k1(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool;

	//Elliptic Curves functionalities

	fn add_ec(
		&self,
		curve: &EllipticCurve<Self::BigUint>,
		x_first_point: Self::BigUint,
		y_first_point: Self::BigUint,
		x_second_point: Self::BigUint,
		y_second_point: Self::BigUint,
	) -> (Self::BigUint, Self::BigUint);

	fn double_ec(
		&self,
		curve: &EllipticCurve<Self::BigUint>,
		x_point: Self::BigUint,
		y_point: Self::BigUint,
	) -> (Self::BigUint, Self::BigUint);

	fn is_on_curve_ec(
		&self,
		curve: &EllipticCurve<Self::BigUint>,
		x_point: Self::BigUint,
		y_point: Self::BigUint,
	) -> bool;

	fn scalar_mult(
		&self,
		curve: &EllipticCurve<Self::BigUint>,
		x_point: Self::BigUint,
		y_point: Self::BigUint,
		data: BoxedBytes,
	) -> (Self::BigUint, Self::BigUint);

	fn scalar_base_mult(
		&self,
		curve: &EllipticCurve<Self::BigUint>,
		data: BoxedBytes,
	) -> (Self::BigUint, Self::BigUint);

	fn marshal_ec(
		&self,
		curve: &EllipticCurve<Self::BigUint>,
		x_pair: Self::BigUint,
		y_pair: Self::BigUint,
	) -> BoxedBytes;

	fn marshal_compressed_ec(
		&self,
		curve: &EllipticCurve<Self::BigUint>,
		x_pair: Self::BigUint,
		y_pair: Self::BigUint,
	) -> BoxedBytes;

	fn unmarshal_ec(
		&self,
		curve: &EllipticCurve<Self::BigUint>,
		data: BoxedBytes,
	) -> (Self::BigUint, Self::BigUint);

	fn unmarshal_compressed_ec(
		&self,
		curve: &EllipticCurve<Self::BigUint>,
		data: BoxedBytes,
	) -> (Self::BigUint, Self::BigUint);

	fn generate_key_ec(
		&self,
		curve: &EllipticCurve<Self::BigUint>,
	) -> (Self::BigUint, Self::BigUint, BoxedBytes);
}
