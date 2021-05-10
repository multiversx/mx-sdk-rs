use super::{BigIntApi, BigUintApi};
use crate::types::{BoxedBytes, EllipticCurve, H256};

pub trait CryptoApi<BigInt, BigUint>
where
	BigInt: BigIntApi<BigUint> + 'static,
	BigUint: BigUintApi + 'static,
{
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
		curve: &EllipticCurve<BigUint>,
		x_first_point: BigUint,
		y_first_point: BigUint,
		x_second_point: BigUint,
		y_second_point: BigUint,
	) -> (BigUint, BigUint);

	fn double_ec(
		&self,
		curve: &EllipticCurve<BigUint>,
		x_point: BigUint,
		y_point: BigUint,
	) -> (BigUint, BigUint);

	fn is_on_curve_ec(
		&self,
		curve: &EllipticCurve<BigUint>,
		x_point: BigUint,
		y_point: BigUint,
	) -> bool;

	fn scalar_mult(
		&self,
		curve: &EllipticCurve<BigUint>,
		x_point: BigUint,
		y_point: BigUint,
		data: BoxedBytes,
	) -> (BigUint, BigUint);

	fn scalar_base_mult(
		&self,
		curve: &EllipticCurve<BigUint>,
		data: BoxedBytes,
	) -> (BigUint, BigUint);

	fn marshal_ec(
		&self,
		curve: &EllipticCurve<BigUint>,
		x_pair: BigUint,
		y_pair: BigUint,
	) -> BoxedBytes;

	fn marshal_compressed_ec(
		&self,
		curve: &EllipticCurve<BigUint>,
		x_pair: BigUint,
		y_pair: BigUint,
	) -> BoxedBytes;

	fn unmarshal_ec(&self, curve: &EllipticCurve<BigUint>, data: BoxedBytes) -> (BigUint, BigUint);

	fn unmarshal_compressed_ec(
		&self,
		curve: &EllipticCurve<BigUint>,
		data: BoxedBytes,
	) -> (BigUint, BigUint);

	fn generate_key_ec(&self, curve: &EllipticCurve<BigUint>) -> (BigUint, BigUint, BoxedBytes);
}
