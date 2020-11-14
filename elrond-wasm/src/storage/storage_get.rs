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

	fn try_get_big_uint_handle(&self) -> (bool, i32) {
		(true, self.api.storage_load_big_uint_raw(self.key))
	}

	// TODO: there is currently no API hook for storage of signed big ints
}

pub fn storage_get<'k, A, BigInt, BigUint, T>(api: A, key: &'k [u8]) -> T
where
	T: TopDecode,
	BigInt: NestedEncode + 'static,
	BigUint: NestedEncode + 'static,
	A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + 'static,
{
	T::top_decode_or_exit(
		StorageGetInput::new(api.clone(), key),
		api,
		storage_get_exit,
	)
}

#[inline(always)]
fn storage_get_exit<A, BigInt, BigUint>(api: A, de_err: DecodeError) -> !
where
	BigInt: NestedEncode + 'static,
	BigUint: NestedEncode + 'static,
	A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + 'static,
{
	let decode_err_message =
		BoxedBytes::from_concat(&[err_msg::STORAGE_DECODE_ERROR, de_err.message_bytes()][..]);
	api.signal_error(decode_err_message.as_slice())
}
