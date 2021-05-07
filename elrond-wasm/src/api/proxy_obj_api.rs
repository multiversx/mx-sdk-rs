use super::{BigIntApi, BigUintApi, ErrorApi, SendApi, StorageReadApi, StorageWriteApi};
use crate::types::{Address, TokenIdentifier};

pub trait ProxyObjApi {
	type BigUint: BigUintApi + 'static;

	type BigInt: BigIntApi + 'static;

	/// The code generator produces the same types in the proxy, as for the main contract.
	/// Sometimes endpoints return types that contain a `Self::Storage` type argument,
	/// as for example in `SingleValueMapper<Self::Storage, i32>`.
	/// In order for the proxy code to compile, it is necessary to specify this type here too
	/// (even though it is not required by the trait's methods per se).
	type Storage: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static;

	type ProxySendApi: SendApi<AmountType = Self::BigUint, ProxyBigInt = Self::BigInt>
		+ Clone
		+ 'static;

	// type ContractCall<R>;

	fn new_proxy_obj(api: Self::ProxySendApi, address: Address) -> Self;

	fn with_token_transfer(self, token: TokenIdentifier, payment: Self::BigUint) -> Self;

	fn into_fields(self) -> (Self::ProxySendApi, Address, TokenIdentifier, Self::BigUint);
}
