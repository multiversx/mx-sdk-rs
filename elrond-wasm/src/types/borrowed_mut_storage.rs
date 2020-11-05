use crate::*;
use core::marker::PhantomData;
use core::ops::Deref;
use core::ops::DerefMut;
use elrond_codec::*;

/// Internal key container for BorrowedMutStorage.
enum BorrowedMutStorageKey {
	Const(&'static [u8]),
	Generated(Vec<u8>),
}

impl BorrowedMutStorageKey {
	fn as_bytes(&self) -> &[u8] {
		match self {
			BorrowedMutStorageKey::Generated(v) => v.as_slice(),
			BorrowedMutStorageKey::Const(v) => v,
		}
	}
}

/// Contains a value taken from storage and a reference to the api.
/// The value can be changed and will be saved back to storage
/// when the lifetime of the BorrowedMutStorage expires.
/// Optimization: will only save back to storage if the value is referenced with deref_mut(),
/// because only in such way can it be changed.
pub struct BorrowedMutStorage<A, BigInt, BigUint, T>
where
	BigInt: NestedEncode + 'static,
	BigUint: NestedEncode + 'static,
	A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + 'static,
	T: TopEncode + TopDecode,
{
	api: A,
	key: BorrowedMutStorageKey,
	value: T,
	dirty: bool,
	_phantom1: PhantomData<BigInt>,
	_phantom2: PhantomData<BigUint>,
}

impl<A, BigInt, BigUint, T> BorrowedMutStorage<A, BigInt, BigUint, T>
where
	BigInt: NestedEncode + 'static,
	BigUint: NestedEncode + 'static,
	A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + 'static,
	T: TopEncode + TopDecode,
{
	pub fn with_const_key(api: A, key: &'static [u8]) -> Self {
		let value: T = storage_get(api.clone(), key);
		BorrowedMutStorage {
			api,
			key: BorrowedMutStorageKey::Const(key),
			value,
			dirty: false,
			_phantom1: PhantomData,
			_phantom2: PhantomData,
		}
	}

	pub fn with_generated_key(api: A, key: Vec<u8>) -> Self {
		let value: T = storage_get(api.clone(), key.as_slice());
		BorrowedMutStorage {
			api,
			key: BorrowedMutStorageKey::Generated(key),
			value,
			dirty: false,
			_phantom1: PhantomData,
			_phantom2: PhantomData,
		}
	}
}

impl<A, BigInt, BigUint, T> Drop for BorrowedMutStorage<A, BigInt, BigUint, T>
where
	BigInt: NestedEncode + 'static,
	BigUint: NestedEncode + 'static,
	A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + 'static,
	T: TopEncode + TopDecode,
{
	fn drop(&mut self) {
		if self.dirty {
			storage_set(self.api.clone(), self.key.as_bytes(), &self.value);
		}
	}
}

impl<A, BigInt, BigUint, T> Deref for BorrowedMutStorage<A, BigInt, BigUint, T>
where
	BigInt: NestedEncode + 'static,
	BigUint: NestedEncode + 'static,
	A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + 'static,
	T: TopEncode + TopDecode,
{
	type Target = T;

	fn deref(&self) -> &T {
		&self.value
	}
}

impl<A, BigInt, BigUint, T> DerefMut for BorrowedMutStorage<A, BigInt, BigUint, T>
where
	BigUint: BigUintApi + 'static,
	BigInt: BigIntApi<BigUint> + 'static,
	A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + 'static,
	T: TopEncode + TopDecode,
{
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.dirty = true;
		&mut self.value
	}
}

impl<A, BigInt, BigUint, T> EndpointResult<A, BigInt, BigUint>
	for BorrowedMutStorage<A, BigInt, BigUint, T>
where
	BigInt: BigIntApi<BigUint> + 'static,
	BigUint: BigUintApi + 'static,
	A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + 'static,
	T: TopEncode + TopDecode + EndpointResult<A, BigInt, BigUint>,
{
	fn finish(&self, api: A) {
		core::ops::Deref::deref(self).finish(api);
	}
}
