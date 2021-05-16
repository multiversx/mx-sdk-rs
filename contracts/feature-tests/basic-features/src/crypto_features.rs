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

	#[endpoint]
	fn add_ec(
		&self,
		curve: &EllipticCurve<Self::BigUint>,
		x_first_point: Self::BigUint,
		y_first_point: Self::BigUint,
		x_second_point: Self::BigUint,
		y_second_point: Self::BigUint,
	) -> (Self::BigUint, Self::BigUint) {
		self.crypto().add_ec(
			&curve,
			x_first_point,
			y_first_point,
			x_second_point,
			y_second_point,
		)
	}

	#[endpoint]
	fn double_ec(
		&self,
		curve: &EllipticCurve<Self::BigUint>,
		x_point: Self::BigUint,
		y_point: Self::BigUint,
	) -> (Self::BigUint, Self::BigUint) {
		self.crypto().double_ec(curve, x_point, y_point)
	}

	#[endpoint]
	fn is_on_curve_ec(
		&self,
		curve: &EllipticCurve<Self::BigUint>,
		x_point: Self::BigUint,
		y_point: Self::BigUint,
	) -> bool {
		self.crypto().is_on_curve_ec(curve, x_point, y_point)
	}

	#[endpoint]
	fn scalar_mult(
		&self,
		curve: &EllipticCurve<Self::BigUint>,
		x_point: Self::BigUint,
		y_point: Self::BigUint,
		data: BoxedBytes,
	) -> (Self::BigUint, Self::BigUint) {
		self.crypto().scalar_mult(curve, x_point, y_point, data)
	}

	#[endpoint]
	fn scalar_base_mult(
		&self,
		curve: &EllipticCurve<Self::BigUint>,
		data: BoxedBytes,
	) -> (Self::BigUint, Self::BigUint) {
		self.crypto().scalar_base_mult(curve, data)
	}

	#[endpoint]
	fn marshal_ec(
		&self,
		curve: &EllipticCurve<Self::BigUint>,
		x_pair: Self::BigUint,
		y_pair: Self::BigUint,
	) -> BoxedBytes {
		self.crypto().marshal_ec(curve, x_pair, y_pair)
	}

	#[endpoint]
	fn marshal_compressed_ec(
		&self,
		curve: &EllipticCurve<Self::BigUint>,
		x_pair: Self::BigUint,
		y_pair: Self::BigUint,
	) -> BoxedBytes {
		self.crypto().marshal_compressed_ec(curve, x_pair, y_pair)
	}

	#[endpoint]
	fn unmarshal_ec(
		&self,
		curve: &EllipticCurve<Self::BigUint>,
		data: BoxedBytes,
	) -> (Self::BigUint, Self::BigUint) {
		self.crypto().unmarshal_ec(curve, data)
	}

	#[endpoint]
	fn unmarshal_compressed_ec(
		&self,
		curve: &EllipticCurve<Self::BigUint>,
		data: BoxedBytes,
	) -> (Self::BigUint, Self::BigUint) {
		self.crypto().unmarshal_compressed_ec(curve, data)
	}

	#[endpoint]
	fn generate_key_ec(
		&self,
		curve: &EllipticCurve<Self::BigUint>,
	) -> (Self::BigUint, Self::BigUint, BoxedBytes) {
		self.crypto().generate_key_ec(curve)
	}
}
