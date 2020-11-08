use crate::*;
use core::marker::PhantomData;
use elrond_codec::*;

struct StorageGetInput<'k, A, BigInt, BigUint>
where
	BigInt: NestedEncode + 'static,
	BigUint: NestedEncode + 'static,
	A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + 'static,
{
	api: A,
	key: &'k [u8],
	_phantom1: PhantomData<BigInt>,
	_phantom2: PhantomData<BigUint>,
}

impl<'k, A, BigInt, BigUint> StorageGetInput<'k, A, BigInt, BigUint>
where
	BigInt: NestedEncode + 'static,
	BigUint: NestedEncode + 'static,
	A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + 'static,
{
	#[inline]
	fn new(api: A, key: &'k [u8]) -> Self {
		StorageGetInput {
			api,
			key,
			_phantom1: PhantomData,
			_phantom2: PhantomData,
		}
	}
}

impl<'k, A, BigInt, BigUint> TopDecodeInput for StorageGetInput<'k, A, BigInt, BigUint>
where
	BigInt: NestedEncode + 'static,
	BigUint: NestedEncode + 'static,
	A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + 'static,
{
	fn byte_len(&self) -> usize {
		self.api.storage_load_len(self.key)
	}

	fn into_boxed_slice_u8(self) -> Box<[u8]> {
		self.api.storage_load_boxed_bytes(self.key).into_box()
	}

	fn into_u64(self) -> u64 {
		self.api.storage_load_u64(self.key)
	}

	fn into_i64(self) -> i64 {
		self.api.storage_load_i64(self.key)
	}
}

pub fn storage_get<'k, A, BigInt, BigUint, T>(api: A, key: &'k [u8]) -> T
where
	T: TopDecode,
	BigInt: NestedEncode + 'static,
	BigUint: NestedEncode + 'static,
	A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + 'static,
{
	// the compiler is smart enough to evaluate this match at compile time
	match T::TYPE_INFO {
		TypeInfo::BigUint => {
			// self must be of type BigUint
			// performing a forceful cast
			let big_uint_value = api.storage_load_big_uint(key);
			let cast_big_uint: T = unsafe { core::mem::transmute_copy(&big_uint_value) };
			core::mem::forget(big_uint_value); // otherwise the data gets deallocated twice
			cast_big_uint
		},
		_ => T::top_decode_or_exit(
			StorageGetInput::new(api.clone(), key),
			api,
			storage_get_exit,
		),
	}
}

#[inline(always)]
fn storage_get_exit<A, BigInt, BigUint>(api: A, de_err: DecodeError) -> !
where
	BigInt: NestedEncode + 'static,
	BigUint: NestedEncode + 'static,
	A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + 'static,
{
	let mut decode_err_message: Vec<u8> = Vec::new();
	decode_err_message.extend_from_slice(err_msg::STORAGE_DECODE_ERROR);
	decode_err_message.extend_from_slice(de_err.message_bytes());
	api.signal_error(decode_err_message.as_slice())
}
